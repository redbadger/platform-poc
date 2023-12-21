use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequest {
    pub is_in_stock: bool,
    pub sku_code: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "product_id")]
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: isize,
    pub sku_code: String,
}
