use axum::{routing::get, Router};

use crate::config::Config;

pub async fn create(config: Config) {
    // build our application with a route
    let app = Router::new().route("/", get(root));

    // run it
    //todo pass a port
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
