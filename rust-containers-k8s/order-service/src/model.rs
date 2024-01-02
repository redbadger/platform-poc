use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i64>,
    pub order_number: Uuid,
    pub line_items: Vec<LineItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: Option<i64>,
    pub sku: String,
    pub price: BigDecimal,
    pub quantity: i32,
}
