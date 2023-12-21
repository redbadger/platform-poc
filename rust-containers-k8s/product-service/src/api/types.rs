use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "product_id")]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: isize,
    pub sku_code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequest {
    pub name: String,
    pub description: String,
    pub price: isize,
    pub sku_code: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub name: String,
    pub description: String,
    pub price: isize,
    pub sku_code: String,
}

impl From<Product> for ProductResponse {
    fn from(value: Product) -> Self {
        ProductResponse {
            name: value.name,
            description: value.description,
            price: value.price,
            sku_code: value.sku_code,
        }
    }
}
