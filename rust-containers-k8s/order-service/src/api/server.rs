use crate::api::handlers::{create_order, get_orders, health, root};
use crate::config::Config;
use async_nats::Client;
use axum::{routing::get, routing::post, Router};
use sqlx::{Pool, Postgres};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;

pub struct AppState {
    pub config: Config,
    pub pool: Pool<Postgres>,
    pub client: Client,
}

pub async fn create(config: Config, pool: Pool<Postgres>) -> anyhow::Result<()> {
    let port = config.port;

    let client = async_nats::connect(&config.nats_url).await?;

    let state = Arc::new(AppState {
        config,
        pool,
        client,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/order", get(get_orders))
        .route("/api/order", post(create_order))
        .with_state(state);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let listener = TcpListener::bind(&socket).await.unwrap();
    tracing::info!("listening on {}", socket);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
