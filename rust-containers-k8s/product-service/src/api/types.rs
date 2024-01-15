use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::core::Product;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequest {
    pub name: String,
    pub description: String,
    pub price: isize,
    pub sku_code: String,
}

impl From<ProductRequest> for Product {
    fn from(value: ProductRequest) -> Self {
        Product {
            id: Uuid::new_v4(),
            name: value.name,
            description: value.description,
            price: value.price,
            sku_code: value.sku_code,
        }
    }
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
