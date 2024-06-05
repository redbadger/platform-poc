wit_bindgen::generate!({
    world: "common"
});

use platform_poc::products::types::Product as Wit_Product;
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

impl From<Wit_Product> for Product {
    fn from(product: Wit_Product) -> Self {
        Product {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            sku: product.sku,
        }
    }
}

impl Into<Wit_Product> for Product {
    fn into(self) -> Wit_Product {
        Wit_Product {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price,
            sku: self.sku,
        }
    }
}