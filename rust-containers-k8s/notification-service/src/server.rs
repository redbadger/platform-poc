use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

pub async fn create(port: u16) -> anyhow::Result<()> {
    let app = Router::new().route("/health", get(health));

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

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
