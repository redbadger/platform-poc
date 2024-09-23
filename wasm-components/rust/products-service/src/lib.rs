wit_bindgen::generate!({
    world: "platform-poc:products-service/products-service",
    path: [
      "../../wit/products",
      "../../wit/deps/wasi/keyvalue",
      "../../wit/deps/wasi/logging",
      "wit",
    ],
    generate_all,
});

use serde::{Deserialize, Serialize};

use exports::platform_poc::products::products::{Error, Guest, Product};
use uuid::Uuid;
use wasi::{
    keyvalue::store::open,
    logging::logging::{log, Level},
};

struct ProductComponent;
export!(ProductComponent);

impl Guest for ProductComponent {
    fn create_product(product: Product) -> Result<(), Error> {
        log(Level::Info, "products-service", "Creating product...");

        let product: SerializableProduct = match product.try_into() {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::BadRequest(format!(
                    "PRODUCTS-SERVICE-CREATE-PRODUCT: malformed product: {e}"
                )))
            }
        };

        let bucket = open("").expect("PRODUCTS-SERVICE-CREATE-PRODUCT: failed to open bucket");

        let product_json = serde_json::to_string(&product)
            .expect("PRODUCTS-SERVICE-CREATE-PRODUCT: failed to convert product to json");
        bucket
            .set(&product.sku, product_json.as_bytes())
            .expect("PRODUCTS-SERVICE-CREATE-PRODUCT: failed to set product");

        Ok(())
    }

    fn list_products() -> Result<Vec<Product>, Error> {
        log(Level::Info, "products-service", "Listing products...");

        let bucket = open("").expect("PRODUCTS-SERVICE-LIST-PRODUCTS: failed to open bucket");

        let mut product_keys = Vec::new();
        let mut cursor = None;
        loop {
            let res = bucket
                .list_keys(cursor)
                .expect("PRODUCTS-SERVICE-LIST-PRODUCTS: failed to list keys");
            product_keys.extend(res.keys);
            cursor = res.cursor;
            if cursor.is_none() {
                break;
            }
        }

        let products: Vec<Product> = product_keys
            .iter()
            .map(|key| {
                let product = bucket
                    .get(key)
                    .expect("PRODUCTS-SERVICE-LIST-PRODUCTS: failed to get key")
                    .expect("PRODUCTS-SERVICE-LIST-PRODUCTS: product not found");
                let p = serde_json::from_slice::<SerializableProduct>(&product)
                    .expect("PRODUCTS-SERVICE-LIST-PRODUCTS: failed to convert product to struct");
                p.into()
            })
            .collect();

        Ok(products)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableProduct {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub sku: String,
}

impl TryFrom<Product> for SerializableProduct {
    type Error = anyhow::Error;

    fn try_from(value: Product) -> Result<Self, Self::Error> {
        Ok(SerializableProduct {
            id: value.id.parse()?,
            name: value.name,
            description: value.description,
            price: value.price,
            sku: value.sku,
        })
    }
}

impl From<SerializableProduct> for Product {
    fn from(val: SerializableProduct) -> Self {
        Product {
            id: val.id.to_string(),
            name: val.name,
            description: val.description,
            price: val.price,
            sku: val.sku,
        }
    }
}
