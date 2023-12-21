use axum::{extract::State, http::StatusCode, response::Result, Json};
use std::sync::Arc;

use super::{
    server::AppState,
    types::{Product, ProductRequest},
};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn health() -> &'static str {
    "ok"
}

#[axum::debug_handler]
pub async fn get_all_products(State(_state): State<Arc<AppState>>) -> Result<Json<Vec<Product>>> {
    Ok(Json(vec![Product {
        id: 1,
        name: "Product 1".to_string(),
        description: "Product 1 description".to_string(),
        price: 100,
        sku_code: "SKU-1".to_string(),
    }]))
}

#[axum::debug_handler]
pub async fn create_product(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<ProductRequest>,
) -> Result<()> {
    todo!()
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
