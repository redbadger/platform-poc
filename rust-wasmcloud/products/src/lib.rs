// Use WIT bindgen instead of cargo component bindgen because ... generated files in target/ are icky
wit_bindgen::generate!({
    world: "product-service",
    exports: {
        "platform-poc:products/products": ProductService,
    }
});

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use exports::platform_poc::products::products as product_service;
use platform_poc::keyvalue::keyvalue::{self as kv, Bucket};

const COLLECTION: &str = "products";

struct ProductService;

impl ProductService {
    fn store_product(product: Product) -> Result<(), product_service::Error> {
        let bucket = Bucket::open(COLLECTION)?;
        let key = product.id.to_string();

        let bytes = serde_json::to_vec(&product)?;
        bucket.set(&key, &bytes)?;

        Ok(())
    }
}

impl product_service::Guest for ProductService {
    fn start() -> Result<(), product_service::Error> {
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

    fn create_product(product: product_service::Product) -> Result<(), product_service::Error> {
        Self::store_product(product.try_into()?)?;

        Ok(())
    }

    fn list_products() -> Result<Vec<product_service::Product>, product_service::Error> {
        let bucket = Bucket::open(COLLECTION)?;

        // incoming bytes -> local Product -> outgoing Product
        bucket
            .get_all()
            .map_err(|e| e.into())
            .and_then(|kv_pairs| {
                kv_pairs
                    .into_iter()
                    .map(|(_, bytes)| serde_json::from_slice::<Product>(&bytes))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.into())
            })
            .map(|products| products.into_iter().map(|product| product.into()).collect())
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

impl TryFrom<product_service::Product> for Product {
    type Error = uuid::Error;

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

impl From<kv::Error> for product_service::Error {
    fn from(value: kv::Error) -> Self {
        product_service::Error::StoreError(format!("Keyvalue store error: {}", value))
    }
}

impl From<uuid::Error> for product_service::Error {
    fn from(value: uuid::Error) -> Self {
        product_service::Error::Internal(format!("Error parsing uuid: {}", value))
    }
}

impl From<serde_json::Error> for product_service::Error {
    fn from(value: serde_json::Error) -> Self {
        product_service::Error::Internal(format!("Error parsing JSON: {}", value))
    }
}
