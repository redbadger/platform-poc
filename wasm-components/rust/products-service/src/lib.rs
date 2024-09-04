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

use exports::platform_poc::products::products::{Error, Guest as ProductGuest, Product};
use wasi::{
    keyvalue::store::open,
    logging::logging::{log, Level},
};

use common::products::Product as ProductData;

struct ProductComponent;

impl From<Product> for ProductData {
    fn from(product: Product) -> Self {
        ProductData {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            sku: product.sku,
        }
    }
}

impl From<ProductData> for Product {
    fn from(val: ProductData) -> Self {
        Product {
            id: val.id,
            name: val.name,
            description: val.description,
            price: val.price,
            sku: val.sku,
        }
    }
}

impl ProductGuest for ProductComponent {
    fn create_product(product: Product) -> Result<(), Error> {
        log(Level::Info, "products-service", "Creating product...");

        let product_data: ProductData = product.into();

        let bucket = open("").expect("PRODUCTS-SERVICE-CREATE-PRODUCT: failed to open bucket");

        let product_json = serde_json::to_string(&product_data)
            .expect("PRODUCTS-SERVICE-CREATE-PRODUCT: failed to convert product to json");
        bucket
            .set(&product_data.sku, product_json.as_bytes())
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
                let p = serde_json::from_slice::<ProductData>(&product)
                    .expect("PRODUCTS-SERVICE-LIST-PRODUCTS: failed to convert product to struct");
                p.into()
            })
            .collect();

        Ok(products)
    }
}

export!(ProductComponent);
