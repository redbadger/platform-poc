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

use uuid::Uuid;

use common::{notification::OrderNotification, NOTIFICATION_SUBJECT};
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

struct Component;

impl Guest for Component {
    #[doc = r" Creates an `order` for specified line items"]
    fn create_order(items: Vec<LineItem>) -> Result<(), Error> {
        log(Level::Info, "orders-service", "Order request received...");

        let skus: Vec<String> = items.iter().map(|item| item.sku.clone()).collect();

        let availability =
            get_inventory(&skus).expect("ORDER-SERVICE-CREATE-ORDER: Failed to get inventory");

        if availability.iter().all(|item| item.is_in_stock) {
            log(
                Level::Info,
                "orders-service",
                "All requested products are in stock",
            );
            query("BEGIN;", &[]).expect("ORDER-SERVICE-CREATE-ORDER: Failed to begin transaction");

            let mut ids: Vec<String> = vec![];

            // create line items
            for item in &items {
                let params = vec![
                    PgValue::Integer(item.price),
                    PgValue::Integer(item.quantity),
                    PgValue::Text(item.sku.clone()),
                ];

                let id = query(
                    "-- Create line item
                INSERT INTO orders.t_order_line_items (price, quantity, sku)
                VALUES ($1, $2, $3) RETURNING id;",
                    &params,
                )
                .expect("ORDER-SERVICE-CREATE-ORDER: Failed to insert order line item");

                if let PgValue::Int8(id) = id[0][0].value {
                    ids.push(id.to_string());
                }
            }

            let total = &items
                .iter()
                .fold(0, |acc, item| acc + item.price * item.quantity);

            let order_number = Uuid::new_v4().to_string();

            let pg_response = query(
                "-- Create order entry
            INSERT INTO orders.t_orders (order_number, total)
            VALUES ($1, $2) RETURNING id",
                &[
                    PgValue::Text(order_number.clone()),
                    PgValue::Integer(*total),
                ],
            )
            .expect("ORDER-SERVICE-CREATE-ORDER: Failed to insert order");

            let order_id: String;

            if let PgValue::Int8(id) = pg_response[0][0].value {
                order_id = id.to_string();
            } else {
                panic!("RDER-SERVICE-CREATE-ORDER: Failed to get order id");
            }

            for id in ids {
                query(
                    "-- Link order and line items
                    INSERT INTO orders.t_orders_order_line_items_list (order_id, order_line_items_list_id)
                    VALUES ($1, $2);",
                    &[PgValue::BigInt(order_id.parse().unwrap()), PgValue::BigInt(id.parse().unwrap())],
                ).expect("ORDER-SERVICE-CREATE-ORDER: Failed to link order and line items");
            }

            // TODO: make sure no idle transactions are left hanging if things go wrong here (rollback)
            query("COMMIT;", &[])
                .expect("ORDER-SERVICE-CREATE-ORDER: Failed to commit transaction");

            let notification = OrderNotification { order_number };

            let serialized: Vec<u8> =
                serde_json::to_vec(&notification).expect("Serialization failed");

            let msg = BrokerMessage {
                subject: NOTIFICATION_SUBJECT.to_string(),
                reply_to: None,
                body: serialized,
            };

            let res = publish(&msg);

            if let Err(e) = res {
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

    #[doc = r" Lists all orders"]
    fn get_orders() -> Result<Vec<Order>, Error> {
        let get_orders_query = r#"
        SELECT
            "order".order_number,
            "order".total as order_total,
            "line_items".sku,
            "line_items".price,
            "line_items".quantity
        FROM
            orders.t_order_line_items as line_items
            JOIN orders.t_orders_order_line_items_list as order_lines ON "order_lines".order_line_items_list_id = "line_items".id
            JOIN orders.t_orders as "order" ON "order".id = "order_lines".order_id;"#;

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
                    } if column_name == *"order_number" => {
                        order_number = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if column_name == *"order_total" => {
                        order_total = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Text(val),
                    } if column_name == *"sku" => {
                        sku = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if column_name == *"price" => {
                        price = val;
                    }
                    ResultRowEntry {
                        column_name,
                        value: PgValue::Int4(val),
                    } if column_name == *"quantity" => {
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

export!(Component);
