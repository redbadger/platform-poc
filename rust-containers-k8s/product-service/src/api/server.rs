use axum::{
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{get, post},
    Json, Router,
};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::trace::TraceLayer;

use super::core::{Product, Service, Store, StoreError};
use super::types::{ProductRequest, ProductResponse};
use crate::config::Config;

pub struct AppState {
    pub(crate) service: Service<RedisStore>,
}

pub struct RedisStore {
    db: MultiplexedConnection,
}

impl Store for RedisStore {
    async fn insert_product(&mut self, product: Product) -> Result<(), StoreError> {
        self.db
            .set(
                format!("products:{id}", id = product.id),
                serde_json::to_string(&product).map_err(|e| StoreError::Other(e.to_string()))?,
            )
            .await
            .map_err(|e| StoreError::Other(e.to_string()))
    }

    async fn get_all_products(&mut self) -> Result<Vec<Product>, StoreError> {
        let keys = self
            .db
            .keys::<String, Vec<String>>("products:*".to_string())
            .await
            .map_err(|e| StoreError::Other(e.to_string()))?;

        let mut products = Vec::new();

        for key in keys {
            let product: String = self
                .db
                .get(key)
                .await
                .map_err(|e| StoreError::Other(e.to_string()))?;
            let product: Product =
                serde_json::from_str(&product).map_err(|e| StoreError::Other(e.to_string()))?;
            products.push(product);
        }

        Ok(products)
    }

    async fn is_empty(&mut self) -> Result<bool, StoreError> {
        let keys = self
            .db
            .keys::<String, Vec<String>>("products:*".to_string())
            .await
            .map_err(|e| StoreError::Other(e.to_string()))?;

        Ok(keys.is_empty())
    }
}

pub async fn create(config: Config) -> anyhow::Result<()> {
    let client = Client::open(config.redis_url)?;
    let db = client.get_multiplexed_async_connection().await?;

    let mut state = AppState {
        service: Service::new(RedisStore { db }),
    };
    state.service.start().await?;

    let state = Arc::new(Mutex::new(state));

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
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<ProductResponse>>> {
    let products: Vec<Product> = state
        .lock()
        .await
        .service
        .list_products()
        .await
        .map_err(internal_error)?;

    Ok(Json(products.into_iter().map(Into::into).collect()))
}

#[axum::debug_handler]
pub async fn create_product(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ProductRequest>,
) -> Result<StatusCode> {
    let product: Product = payload.into();
    state
        .lock()
        .await
        .service
        .create_product(product)
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::CREATED)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
