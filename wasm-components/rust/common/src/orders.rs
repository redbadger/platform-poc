use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub order_number: String, #[doc = r" UUID"]
    pub line_items: Vec<LineItem>, #[doc = r" amount in pennies"]
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LineItem {
    pub sku: String, #[doc = r" amount in pennies"]
    pub price: i32,
    pub quantity: i32,
}
