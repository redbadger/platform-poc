use dotenv::dotenv;
use notification_service::config::Config;
use notification_service::types::OrderPlacedEvent;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer as SecondConsumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    println!("Notification service started");
    let config = Config::new().expect("No Config provided");

    let consumer = create_consumer(&config.kafka_url)?;

    consumer.subscribe(&["test-topic"])?;

    loop {
        let message = consumer.recv().await?;
        let payload = message.payload();
        let Some(message) = payload else {
            continue;
        };
        let content: Result<OrderPlacedEvent, serde_json::Error> = serde_json::from_slice(message);

        match content {
            Ok(order) => println!("Received Notification for Order - {}", order.order_number),
            Err(_) => println!("Error decoding notification order"),
        }
    }
}

fn create_consumer(bootstrap_server: &str) -> anyhow::Result<StreamConsumer> {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("group.id", "my-group")
        .set("enable.partition.eof", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()?;
    Ok(consumer)
}
