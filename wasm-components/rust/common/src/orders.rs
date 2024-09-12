use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    /// UUID
    pub order_number: String,
    pub line_items: Vec<LineItem>,
    /// amount in pence
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LineItem {
    pub sku: String,
    /// amount in pence
    pub price: i32,
    pub quantity: i32,
}
