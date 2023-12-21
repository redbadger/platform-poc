use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryResponse {
    pub sku_code: String,
    pub is_in_stock: bool,
}

#[derive(Deserialize)]

pub struct OrderPlaceEvent {
    order_number: Uuid,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    #[serde(rename = "orderLineItemsDtoList")]
    pub items: Vec<LineItemRequest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineItemRequest {
    pub id: String,
    pub sku_code: String,
    pub price: f32,
    pub quantity: i32,
}
