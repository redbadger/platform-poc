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

pub(crate) struct Service<Store> {
    pub store: Store,
}

impl<S: Store> Service<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn create_product(&mut self, product: Product) -> Result<(), StoreError> {
        self.store.insert_product(product).await
    }

    pub async fn list_products(&mut self) -> Result<Vec<Product>, StoreError> {
        self.store.get_all_products().await
    }

    pub async fn start(&mut self) -> Result<(), StoreError> {
        if self.store.is_empty().await? {
            let products = vec![
                Product {
                    id: uuid::Uuid::new_v4(),
                    name: "iPhone 13".to_string(),
                    description: "New iPhone".to_string(),
                    price: 1000,
                    sku_code: "iphone_13".to_string(),
                },
                Product {
                    id: uuid::Uuid::new_v4(),
                    name: "Samsung S23".to_string(),
                    description: "New Samsung".to_string(),
                    price: 800,
                    sku_code: "samsung_s23".to_string(),
                },
                Product {
                    id: uuid::Uuid::new_v4(),
                    name: "Google Pixel 8".to_string(),
                    description: "New Pixel".to_string(),
                    price: 7000,
                    sku_code: "pixel_8".to_string(),
                },
            ];

            for product in products {
                self.store.insert_product(product).await?;
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("Other error: {0}")]
    Other(String),
}

pub(crate) trait Store {
    async fn insert_product(&mut self, product: Product) -> Result<(), StoreError>;
    async fn get_all_products(&mut self) -> Result<Vec<Product>, StoreError>;
    async fn is_empty(&mut self) -> Result<bool, StoreError>;
}
