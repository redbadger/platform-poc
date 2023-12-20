use axum::{extract::State, http::StatusCode, response::Result, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use std::sync::Arc;

use super::{server::AppState, types::GetInventoryResponse};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn health() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryQueryParams {
    pub sku_code: Vec<String>,
}

pub async fn get_inventory(
    State(state): State<Arc<AppState>>,
    Query(params): Query<InventoryQueryParams>,
) -> Result<Json<Vec<GetInventoryResponse>>> {
    let query = &params.sku_code;

    let result = vec![GetInventoryResponse {
        sku_code: "x".to_string(),
        is_in_stock: true,
    }];
    Ok(Json(result))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
