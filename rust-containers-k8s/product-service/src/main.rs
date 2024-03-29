use dotenv::dotenv;
use firestore::FirestoreDb;
use product_service::{api::server, config::Config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "product_service=info,firestore=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    let config = Config::new().expect("Config couldn't be loaded");

    tracing::info!("{:?}", config);

    let db = FirestoreDb::new(&config.gcp_project_id).await?;

    server::create(config, db).await?;

    Ok(())
}
