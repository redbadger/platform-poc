// use dotenv::dotenv;
// use order_service::{api::server, config::Config};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//     let config = Config::new()?;

//     println!("{:?}", config);

//     server::create(config).await;
//     Ok(())
// }

use std::{thread, time::Duration};

fn main() {
    println!("Order service started");
    thread::sleep(Duration::from_secs(60 * 60 * 24));
}
