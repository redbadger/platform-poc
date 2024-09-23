wit_bindgen::generate!({
    world: "platform-poc:orders-service/orders-service",
    path: [
        "../../wit/deps/wasi/logging",
        "../../wit/deps/wasmcloud/postgres",
        "../../wit/deps/wasmcloud/messaging",
        "../../wit/inventory",
        "../../wit/orders",
        "wit",
    ],
    generate_all,
});

use std::collections::HashMap;

use indoc::indoc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use exports::platform_poc::orders::orders::Guest;
use platform_poc::{
    inventory::inventory::get_inventory,
    orders::types::{Error, LineItem, Order},
};
use wasi::logging::logging::{log, Level};
use wasmcloud::{
    messaging::consumer::{publish, BrokerMessage},
    postgres::{
        query::{query, PgValue},
        types::ResultRowEntry,
    },
};

const NOTIFICATION_SUBJECT: &str = "platform-poc.order-notification";

struct Component;
export!(Component);

impl Guest for Component {
    fn create_order(items: Vec<LineItem>) -> Result<(), Error> {
        log(
            Level::Info,
            "orders-service",
            &format!("Order request received: {:?}", items),
        );

        let skus: Vec<String> = items.iter().map(|item| item.sku.clone()).collect();

        let availability =
            get_inventory(&skus).expect("ORDER-SERVICE-CREATE-ORDER: Failed to get inventory");

        if availability.iter().all(|item| item.is_in_stock) {
            log(
                Level::Info,
                "orders-service",
                "All requested products are in stock",
            );

            let mut sql = String::from(indoc! {r#"
                WITH order_id AS (
                   	INSERT INTO orders.t_orders (order_number, total)
                   	VALUES ($1, $2) RETURNING id
                ), item_ids AS (
                   	INSERT INTO orders.t_order_line_items (price, quantity, sku)
                   	VALUES $3 RETURNING id
                ), joins AS (
                   	SELECT order_id.id as order_id, item_ids.id as item_id FROM order_id, item_ids
                )
                INSERT INTO orders.t_orders_order_line_items_list (order_id, order_line_items_list_id)
                (SELECT order_id, item_id FROM joins);
            "#});

            let values = items
                .iter()
                .map(|item| format!("({},{},'{}')", item.price, item.quantity, item.sku))
                .collect::<Vec<_>>()
                .join(",");
            sql = sql.replace("$3", &values);

            let total = items
                .iter()
                .fold(0, |acc, item| acc + item.price * item.quantity);

            let order_number = Uuid::new_v4().to_string();

            query(
                &sql,
                &[PgValue::Text(order_number.clone()), PgValue::Integer(total)],
            )
            .map_err(|e| {
                let msg = format!("Failed to insert order: {:?}", e);
                log(Level::Error, "orders-service", &msg);
                Error::Internal(msg)
            })?;

            let notification = OrderNotification { order_number };

            let msg = BrokerMessage {
                subject: NOTIFICATION_SUBJECT.to_string(),
                reply_to: None,
                body: serde_json::to_vec(&notification).expect("serialization failed"),
            };

            let result = publish(&msg);

            if let Err(e) = result {
                log(
                    Level::Error,
                    "orders-service",
                    &format!("Failed to publish notification: {:?}", e),
                );
            }
        } else {
            log(
                Level::Info,
                "orders-service",
                "Product(s) not in stock, please try again later",
            );

            return Err(Error::Internal("Product(s) not in stock".to_string()));
        }
        Ok(())
    }

    fn get_orders() -> Result<Vec<Order>, Error> {
        let get_orders_query = indoc! {r#"
        SELECT
            "order".order_number,
            "order".total as order_total,
            "line_items".sku,
            "line_items".price,
            "line_items".quantity
        FROM
            orders.t_order_line_items as line_items
            JOIN orders.t_orders_order_line_items_list as order_lines ON "order_lines".order_line_items_list_id = "line_items".id
            JOIN orders.t_orders as "order" ON "order".id = "order_lines".order_id
        LIMIT 10;
        "#};

        let rows =
            query(get_orders_query, &[]).expect("ORDER-SERVICE-GET-ORDERS: Failed to get orders");

        let mut orders_map: HashMap<String, Order> = HashMap::new();

        // parse sql result and group by order number
        for row in rows {
            let mut order_number = String::new();
            let mut order_total = 0;
            let mut sku = String::new();
            let mut price = 0;
            let mut quantity = 0;

            for column in row {
                match column {
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Text(val),
                    } if &column_name == "order_number" => {
                        order_number = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if &column_name == "order_total" => {
                        order_total = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Text(val),
                    } if &column_name == "sku" => {
                        sku = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if &column_name == "price" => {
                        price = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if &column_name == "quantity" => {
                        quantity = val;
                    }
                    _ => {}
                }
            }

            let line_item = LineItem {
                sku,
                price,
                quantity,
            };

            if let Some(order) = orders_map.get_mut(&order_number) {
                order.line_items.push(line_item);
            } else {
                orders_map.insert(
                    order_number.clone(),
                    Order {
                        order_number: order_number.clone(),
                        line_items: vec![line_item],
                        total: order_total,
                    },
                );
            }
        }

        let orders: Vec<Order> = orders_map.into_values().collect();

        Ok(orders)
    }
}

#[derive(Serialize, Deserialize, Default)]
struct OrderNotification {
    pub order_number: String,
}
