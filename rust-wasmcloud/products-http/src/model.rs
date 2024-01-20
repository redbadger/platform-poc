use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::platform_poc::products::products as product_service;

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

impl TryFrom<product_service::Product> for Product {
    type Error = anyhow::Error;

    fn try_from(value: product_service::Product) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            name: value.name,
            description: value.description,
            price: value.price as isize,
            sku_code: value.sku_code,
        })
    }
}

impl From<Product> for product_service::Product {
    fn from(value: Product) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            description: value.description,
            price: value.price as i32,
            sku_code: value.sku_code,
        }
    }
}
