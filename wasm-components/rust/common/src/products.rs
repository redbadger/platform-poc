use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    #[doc = r" UUID"]
    pub name: String,
    pub description: String,
    pub price: i32,
    pub sku: String,
}
