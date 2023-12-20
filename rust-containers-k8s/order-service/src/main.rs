use dotenv::dotenv;
use order_service::{api::server, config::Config};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = Config::new()?;

    println!("{:?}", config);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // TODO: paramterise this
        .connect("postgres://commerce:commerce@localhost/order-service")
        .await?;

    server::create(config, pool).await?;
    Ok(())
}
