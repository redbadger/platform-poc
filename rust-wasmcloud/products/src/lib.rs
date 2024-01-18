// Use WIT bindgen instead of cargo component bindgen because ... generated files in target/ are icky
wit_bindgen::generate!({
    world: "product-service",
    exports: {
        "platform-poc:products/products": ProductsService,
    }
});

use exports::platform_poc::products::products::Guest as ProductsInterface;

use platform_poc::keyvalue::{
    keyvalue::{self as kv},
    types as kv_types,
};
use platform_poc::products::types;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const COLLECTION: &str = "products";

struct ProductsService;

impl ProductsService {
    fn store_product(product: Product) -> Result<(), types::Error> {
        let bucket = kv::open_bucket(COLLECTION)?;
        let key = product.id.to_string();

        let bytes = serde_json::to_vec(&product)?;
        kv::set(bucket, &key, &bytes)?;

        Ok(())
    }
}

impl ProductsInterface for ProductsService {
    fn start() -> Result<(), types::Error> {
        let products = vec![
            Product {
                id: Uuid::parse_str("9d0b941c-6f52-432a-a736-d654db09a624")?,
                name: "Pound of chocolate".to_string(),
                description: "Want a heart attack? It's a bargain too".to_string(),
                price: 20, // 20p is a bargain
                sku_code: "nom_chocolate".to_string(),
            },
            Product {
                id: Uuid::parse_str("829d96f5-131a-4f78-99f1-6b59c28af945")?,
                name: "Teddy bear".to_string(),
                description: "To cuddle and fall asleep with".to_string(),
                price: 1200, // £12.00
                sku_code: "teddy".to_string(),
            },
        ];

        for product in products {
            Self::store_product(product)?;
        }

        Ok(())
    }

    fn create_product(product: types::Product) -> Result<(), types::Error> {
        Self::store_product(product.try_into()?)?;

        Ok(())
    }

    fn list_products() -> Result<Vec<types::Product>, types::Error> {
        let bucket = kv::open_bucket(COLLECTION)?;

        // incoming bytes -> local Product -> outgoing Product
        kv::get_all(bucket)
            .map_err(|e| e.into())
            .and_then(|kv_pairs| {
                kv_pairs
                    .into_iter()
                    .map(|(_, bytes)| serde_json::from_slice::<Product>(&bytes))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.into())
            })
            .map(|products| {
                products
                    .into_iter()
                    .map(|product| product.into())
                    .collect::<Vec<types::Product>>()
            })
    }
}

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

impl TryFrom<types::Product> for Product {
    type Error = uuid::Error;

    fn try_from(value: types::Product) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            name: value.name,
            description: value.description,
            price: value.price as isize,
            sku_code: value.sku_code,
        })
    }
}

impl From<Product> for types::Product {
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

impl From<kv_types::Error> for types::Error {
    fn from(value: kv_types::Error) -> Self {
        types::Error::StoreError(value.to_string())
    }
}

impl From<uuid::Error> for types::Error {
    fn from(value: uuid::Error) -> Self {
        types::Error::Internal(value.to_string())
    }
}

impl From<serde_json::Error> for types::Error {
    fn from(value: serde_json::Error) -> Self {
        types::Error::Internal(value.to_string())
    }
}
