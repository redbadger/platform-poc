use crate::{
    api::handlers::{create_product, get_all_products, health, root},
    config::Config,
};
use axum::{
    routing::{get, post},
    Router,
};
use firestore::FirestoreDb;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub struct AppState {
    pub db: FirestoreDb,
}

pub async fn create(config: Config, db: FirestoreDb) -> anyhow::Result<()> {
    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/", get(root))
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
