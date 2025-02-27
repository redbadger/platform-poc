use std::sync::Arc;

use axum::{debug_handler, extract::State, http::StatusCode, response::Result, Json};

use super::{
    server::AppState,
    types::{DbLineItem, OrderPlaceEvent},
};
use crate::{
    api::types::{InventoryResponse, OrderRequest},
    model::Order,
};

#[debug_handler]
pub async fn get_orders(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Order>>> {
    let items: Vec<DbLineItem> = sqlx::query_file_as!(DbLineItem, "sql/all_orders.sql",)
        .fetch_all(&state.pool)
        .await
        .map_err(internal_error)?;

    let mut orders: Vec<Order> = vec![];
    let mut current_order: Option<Order> = None;
    for item in items {
        if let Some(order) = &mut current_order {
            if order.order_number.to_string() == item.order_number.as_deref().unwrap_or_default() {
                order.line_items.push(item.into());
            } else {
                let order = Order {
                    id: Some(item.order_id),
                    order_number: item
                        .order_number
                        .clone()
                        .unwrap_or_default()
                        .parse()
                        .unwrap(),
                    line_items: vec![item.into()],
                };
                current_order = Some(order.clone());
                orders.push(order);
            }
        } else {
            let order = Order {
                id: Some(item.order_id),
                order_number: item
                    .order_number
                    .clone()
                    .unwrap_or_default()
                    .parse()
                    .unwrap(),
                line_items: vec![item.into()],
            };
            current_order = Some(order.clone());
            orders.push(order);
        }
    }

    Ok(Json(orders))
}

#[debug_handler]
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OrderRequest>,
) -> Result<(StatusCode, String)> {
    let query: Vec<(String, String)> = payload
        .items
        .iter()
        .map(|i| ("skuCode".to_string(), i.sku.clone()))
        .collect();
    // call inventory service to check stock
    let all_in_stock = state
        .http_client
        .get(&state.config.inventory_url)
        .query(&query)
        .send()
        .await
        .map_err(internal_error)?
        .json::<Vec<InventoryResponse>>()
        .await
        .map_err(internal_error)?
        .iter()
        .all(|i| i.is_in_stock);

    if all_in_stock {
        let order: Order = payload.into();
        let mut transaction = state.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            internal_error(e)
        })?;

        let mut ids = vec![];
        for item in &order.line_items {
            let rec = sqlx::query!(
                r#"
                INSERT INTO t_order_line_items (price, quantity, sku_code)
                VALUES ( $1, $2, $3 )
                RETURNING id
                "#,
                item.price,
                item.quantity,
                item.sku
            )
            .fetch_one(&mut *transaction)
            .await
            .map_err(internal_error)?;
            ids.push(rec.id);
        }

        let rec = sqlx::query!(
            r#"
            INSERT INTO t_orders (order_number)
            VALUES ( $1)
            RETURNING id
            "#,
            &order.order_number.to_string(),
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(internal_error)?;
        let order_id = rec.id;

        for id in ids {
            sqlx::query!(
                r#"
                INSERT INTO t_orders_order_line_items_list (order_id, order_line_items_list_id)
                VALUES ( $1, $2 )
                "#,
                order_id,
                id
            )
            .execute(&mut *transaction)
            .await
            .map_err(internal_error)?;
        }

        transaction.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            internal_error(e)
        })?;

        let bytes = serde_json::to_vec(&OrderPlaceEvent {
            order_number: order.order_number,
        })
        .map_err(internal_error)?;

        state
            .nats_client
            .publish(state.config.nats_topic.clone(), bytes.into())
            .await
            .map_err(internal_error)?;

        Ok((
            StatusCode::CREATED,
            format!(
                "Order Number {order_id} ({order_number}) Placed Successfully",
                order_number = order.order_number
            ),
        ))
    } else {
        Ok((
            StatusCode::BAD_REQUEST,
            "Product is not in stock, please try again later".to_string(),
        ))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn health() -> &'static str {
    "ok"
}
