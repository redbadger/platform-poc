use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryResponse {
    pub sku_code: String,
    pub is_in_stock: bool,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
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
