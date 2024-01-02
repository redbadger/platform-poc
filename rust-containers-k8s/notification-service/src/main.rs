use dotenv::dotenv;
use notification_service::{config::Config, types::OrderPlacedEvent};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer as SecondConsumer, StreamConsumer},
    ClientConfig, Message,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "notification_service=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let config = Config::new()?;
    info!("{:?}", config);

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
            Ok(order) => info!("Received Notification for Order - {}", order.order_number),
            Err(_) => info!("Error decoding notification order"),
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
