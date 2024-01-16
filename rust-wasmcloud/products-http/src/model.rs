use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::platform_poc::products::products::Product as UpstreamProduct;

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

impl TryFrom<UpstreamProduct> for Product {
    type Error = anyhow::Error;

    fn try_from(value: UpstreamProduct) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            name: value.name,
            description: value.description,
            price: value.price as isize,
            sku_code: value.sku_code,
        })
    }
}

impl From<Product> for UpstreamProduct {
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
