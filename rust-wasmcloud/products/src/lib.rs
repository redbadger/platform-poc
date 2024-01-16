use exports::platform_poc::products::products::Guest;
use platform_poc::products::types::{Error, Product};

// Use WIT bindgen instead of cargo component bindgen because ... generated files in target/ are icky
wit_bindgen::generate!({
    world: "product-service",
    exports: {
        "platform-poc:products/products": Component,
    }
});

struct Component;

impl Guest for Component {
    fn start() -> Result<(), Error> {
        Ok(())
    }

    fn create_product(product: Product) -> Result<(), Error> {
        Ok(())
    }

    fn list_products() -> Result<Vec<Product>, Error> {
        Ok(vec![
            Product {
                id: "abc123".to_string(),
                name: "Pound of cocaine".to_string(),
                description: "Want a heart attack? It's a bargain too".to_string(),
                price: 20, // 20p is a bargain
                sku_code: "cocaine_bap".to_string(),
            },
            Product {
                id: "abc567".to_string(),
                name: "Teddy bear".to_string(),
                description: "When you need a hug the next day".to_string(),
                price: 1200, // £12.00
                sku_code: "teddy".to_string(),
            },
        ])
    }
}
