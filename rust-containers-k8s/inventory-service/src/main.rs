use dotenv::dotenv;
use inventory_service::{api::server, config::Config};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = Config::new().expect("Config couldn't be loaded");

    println!("{:?}", config);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    server::create(config, pool).await?;

    Ok(())
}
