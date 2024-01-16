use axum::{
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{get, post},
    Json, Router,
};
use firestore::FirestoreDb;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use super::core::{Product, Service, Store, StoreError};
use super::types::{ProductRequest, ProductResponse};
use crate::config::Config;

pub const COLLECTION_NAME: &str = "products";

pub struct AppState {
    pub(crate) service: Service<FirestoreStore>,
}

pub struct FirestoreStore {
    db: FirestoreDb,
}

impl Store for FirestoreStore {
    async fn insert_product(&self, product: Product) -> Result<(), StoreError> {
        self.db
            .fluent()
            .insert()
            .into(COLLECTION_NAME)
            .document_id(&product.id.to_string())
            .object(&product)
            .execute()
            .await
            .map_err(|e| StoreError::Other(e.to_string()))
    }

    async fn get_all_products(&self) -> Result<Vec<Product>, StoreError> {
        self.db
            .fluent()
            .select()
            .from(COLLECTION_NAME)
            .limit(1000)
            .obj()
            .query()
            .await
            .map_err(|e| StoreError::Other(e.to_string()))
    }

    async fn is_empty(&self) -> Result<bool, StoreError> {
        let result: Vec<_> = self
            .db
            .fluent()
            .select()
            .from(COLLECTION_NAME)
            .limit(1)
            .query()
            .await
            .map_err(|e| StoreError::Other(e.to_string()))?;

        Ok(result.is_empty())
    }
}

pub async fn create(config: Config, db: FirestoreDb) -> anyhow::Result<()> {
    let mut service = Service::new(FirestoreStore { db });
    service.start().await?;

    let state = Arc::new(AppState { service });

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/product", post(create_product))
        .route("/api/product", get(get_all_products))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);
    let listener = TcpListener::bind(&socket).await.unwrap();
    tracing::info!("listening on {}", socket);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn health() -> &'static str {
    "ok"
}

#[axum::debug_handler]
pub async fn get_all_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProductResponse>>> {
    let products: Vec<Product> = state
        .service
        .list_products()
        .await
        .map_err(internal_error)?;

    Ok(Json(products.into_iter().map(Into::into).collect()))
}

#[axum::debug_handler]
pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ProductRequest>,
) -> Result<()> {
    let product: Product = payload.into();

    state
        .service
        .create_product(product)
        .await
        .map_err(internal_error)?;

    Ok(())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
