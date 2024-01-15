use super::{server::AppState, types::ProductRequest};
use crate::api::{core::Product, types::ProductResponse};
use axum::{extract::State, http::StatusCode, response::Result, Json};
use std::sync::Arc;

pub async fn health() -> &'static str {
    "ok"
}

#[axum::debug_handler]
pub async fn get_all_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProductResponse>>> {
    let products: Vec<Product> = state
        .service
        .list_products()
        .await
        .map_err(internal_error)?;

    Ok(Json(products.into_iter().map(Into::into).collect()))
}

#[axum::debug_handler]
pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ProductRequest>,
) -> Result<()> {
    let product: Product = payload.into();

    state
        .service
        .create_product(product)
        .await
        .map_err(internal_error)?;

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
