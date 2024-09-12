use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    /// UUID
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub sku: String,
}
