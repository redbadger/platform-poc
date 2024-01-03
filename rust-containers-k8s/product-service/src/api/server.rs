use crate::{
    api::handlers::{create_product, get_all_products, health, root},
    config::Config,
    model::Product,
};
use axum::{
    routing::{get, post},
    Router,
};
use firestore::{FirestoreDb, FirestoreResult};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub const COLLECTION_NAME: &str = "products";

pub struct AppState {
    pub db: FirestoreDb,
}

pub async fn create(config: Config, db: FirestoreDb) -> anyhow::Result<()> {
    populate_firestore(&db).await?;

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

// populate firestore with some data if it's empty
async fn populate_firestore(db: &FirestoreDb) -> FirestoreResult<()> {
    if db
        .fluent()
        .select()
        .from(COLLECTION_NAME)
        .limit(1)
        .query()
        .await?
        .len()
        == 0
    {
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
            db.fluent()
                .insert()
                .into(COLLECTION_NAME)
                .document_id(&product.id.to_string())
                .object(&product)
                .execute()
                .await?;
        }
    }

    Ok(())
}
