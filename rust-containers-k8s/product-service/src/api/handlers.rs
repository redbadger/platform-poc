use super::{server::AppState, types::ProductRequest};
use crate::api::types::{Product, ProductResponse};
use axum::{extract::State, http::StatusCode, response::Result, Json};
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn health() -> &'static str {
    "ok"
}

#[axum::debug_handler]
pub async fn get_all_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProductResponse>>> {
    let products: Vec<Product> = state
        .db
        .fluent()
        .select()
        .from("products")
        .limit(10)
        .obj()
        .query()
        .await
        .map_err(internal_error)?;
    Ok(Json(
        products.into_iter().map(ProductResponse::from).collect(),
    ))
}

#[axum::debug_handler]
pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ProductRequest>,
) -> Result<()> {
    let product = Product {
        id: uuid::Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        price: payload.price,
        sku_code: payload.sku_code,
    };

    state
        .db
        .fluent()
        .insert()
        .into("products")
        .document_id(&product.id.to_string())
        .object(&product)
        .execute()
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
