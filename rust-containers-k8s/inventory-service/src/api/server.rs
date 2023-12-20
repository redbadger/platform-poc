use crate::{
    api::handlers::{get_inventory, health, root},
    config::Config,
};
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct AppState {
    pub pool: Pool<Postgres>,
}

pub async fn create(config: Config, pool: Pool<Postgres>) -> anyhow::Result<()> {
    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/inventory", get(get_inventory))
        .with_state(state);

    // run it
    //todo pass a port
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
