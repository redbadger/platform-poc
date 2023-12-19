use super::server::AppState;
use crate::api::types::{InventoryResponse, OrderRequest};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use std::sync::Arc;

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
) -> Result<(), (StatusCode, String)> {
    let query: Vec<(String, String)> = payload
        .items
        .iter()
        .map(|i| ("sku".to_string(), i.sku.clone()))
        .collect();
    //  call inventory service;
    // takes a request of a list of order line items, checks they are all in stock (http call to the inventory service) and if so, creates an order entry in the database
    let client = reqwest::Client::new();
    let all_in_stock = client
        // TODO: update this url
        .get("http://inventory-service/api/inventory")
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
        // TODO: Update SQL  query
        let row: (i64,) = sqlx::query_as("INSERT into sometable values ($1)")
            .bind(150_i64)
            .fetch_one(&state.pool)
            .await
            .map_err(internal_error)?;
    }

    Ok(())
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
