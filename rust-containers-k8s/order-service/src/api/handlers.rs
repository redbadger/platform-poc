use super::server::AppState;
use crate::api::types::{InventoryResponse, OrderRequest};
use axum::{debug_handler, extract::State, http::StatusCode, response::Result, Json};
use sqlx::{Postgres, QueryBuilder, Row};
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_orders() -> &'static str {
    "Hello, World!"
}

#[debug_handler]
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OrderRequest>,
) -> Result<String> {
    let query: Vec<(String, String)> = payload
        .items
        .iter()
        .map(|i| ("skuCode".to_string(), i.sku_code.clone()))
        .collect();

    //  call inventory service;
    // takes a request of a list of order line items, checks they are all in stock (http call to the inventory service) and if so, creates an order entry in the database
    let client = reqwest::Client::new();
    let inventory_url = Url::parse(state.inventory_service_url.as_str())
        .map_err(internal_error)?
        .join("/api/inventory")
        .map_err(internal_error)?;
    let all_in_stock = client
        .get(inventory_url)
        .query(&query)
        .send()
        .await
        .map_err(internal_error)?
        .json::<Vec<InventoryResponse>>()
        .await
        .map_err(internal_error)?
        .iter()
        .all(|i| i.is_in_stock);
    let order_uuid = Uuid::new_v4();

    if all_in_stock {
        // TODO: Update SQL  query
        let row: (i64,) =
            sqlx::query_as("INSERT into t_orders(order_number) values ($1) RETURNING id")
                .bind(order_uuid.to_string())
                .fetch_one(&state.pool)
                .await
                .map_err(internal_error)?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
            // spaces as that might interfere with identifiers or quoted strings where exact
            // values may matter.
            "INSERT INTO t_order_line_items(price, quantity, sku_code) ",
        );
        // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
        query_builder
            .push_values(payload.items.iter(), |mut b, items| {
                // If you wanted to bind these by-reference instead of by-value,
                // you'd need an iterator that yields references that live as long as `query_builder`,
                // e.g. collect it to a `Vec` first.
                b.push_bind(items.price)
                    .push_bind(items.quantity)
                    .push_bind(&items.sku_code);
            })
            .push(" RETURNING id");

        let result = query_builder
            .build()
            .fetch_all(&state.pool)
            .await
            .map_err(internal_error)?;

        let line_item_ids: Vec<i64> = result
            .iter()
            .take(1)
            .map(|row| row.get::<i64, usize>(0))
            .collect();

        let mut query_builder_link_table: QueryBuilder<Postgres> = QueryBuilder::new(
            // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
            // spaces as that might interfere with identifiers or quoted strings where exact
            // values may matter.
            "INSERT INTO t_orders_order_line_items_list(order_id, order_line_items_list_id) ",
        );

        // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
        query_builder_link_table.push_values(line_item_ids.iter(), |mut b, line_item_id| {
            // If you wanted to bind these by-reference instead of by-value,
            // you'd need an iterator that yields references that live as long as `query_builder`,
            // e.g. collect it to a `Vec` first.
            b.push_bind(row.0).push_bind(line_item_id);
        });

        query_builder_link_table
            .build()
            .execute(&state.pool)
            .await
            .map_err(internal_error)?;
    }
    Ok(format!("Order Number {} Placed Successfully", order_uuid))
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
