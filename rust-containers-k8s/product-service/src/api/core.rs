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

pub struct Service<Store> {
    pub store: Store,
}

impl<S: Store> Service<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn create_product(&self, product: Product) -> Result<(), StoreError> {
        self.store.insert_product(product).await
    }

    pub async fn list_products(&self) -> Result<Vec<Product>, StoreError> {
        self.store.get_all_products().await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("Other error: {0}")]
    Other(String),
}

pub trait Store {
    async fn insert_product(&self, product: Product) -> Result<(), StoreError>;
    async fn get_all_products(&self) -> Result<Vec<Product>, StoreError>;
}
