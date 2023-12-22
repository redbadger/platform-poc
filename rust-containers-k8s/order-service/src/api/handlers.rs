use super::server::AppState;
use crate::{api::types::CreateOrderRequest, order::Order};
use axum::{debug_handler, extract::State, http::StatusCode, response::Result, Json};
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
    Json(payload): Json<CreateOrderRequest>,
) -> Result<String> {
    let query: Vec<(String, String)> = payload
        .items
        .iter()
        .map(|i| ("skuCode".to_string(), i.sku_code.clone()))
        .collect();

    let order: Order = payload.into();
    let all_in_stock = order
        .check_line_items_stock(&state.inventory_service_url, query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if all_in_stock {
        order
            .save(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    order.send_notification().await.unwrap();
    Ok(format!("Order Number {} Placed Successfully", order.id))
}

pub async fn health() -> &'static str {
    "ok"
}
