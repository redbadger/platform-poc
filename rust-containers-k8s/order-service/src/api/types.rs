use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InventoryResponse {
    sku: String,
    pub is_in_stock: bool,
}

#[derive(Deserialize)]

pub struct OrderPlaceEvent {
    order_number: Uuid,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    pub items: Vec<LineItemRequest>,
}

#[derive(Deserialize)]
pub struct LineItemRequest {
    pub sku: String,
    pub price_cents: isize,
    pub quantity: i32,
}
