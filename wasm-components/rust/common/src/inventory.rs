use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Availability {
    pub sku: String,
    pub is_in_stock: bool,
}