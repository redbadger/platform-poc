use crate::api::handlers::{create_order, get_orders, health, root};
use crate::config::Config;
use axum::{routing::get, routing::post, Router};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct AppState {
    pub pool: Pool<Postgres>,
    pub inventory_service_url: String,
}

pub async fn create(config: Config, pool: Pool<Postgres>) -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        pool,
        inventory_service_url: config.inventory_service_url,
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/order", get(get_orders))
        .route("/api/order", post(create_order))
        .with_state(state);

    // run it
    //todo pass a port
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
