use notification_service::types::OrderPlacedEvent;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Notification service started");

    let consumer = create_consumer("localhost:29092")?;

    consumer.subscribe(&["OrderPlacedEvent"])?;

    loop {
        let message = consumer.recv().await?;
        let payload = message.payload().unwrap();
        let content: OrderPlacedEvent = serde_json::from_slice(payload)?;
        println!("Received Notification for Order - {}", content.order_number);
    }
}

fn create_consumer(bootstrap_server: &str) -> anyhow::Result<StreamConsumer> {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("group.id", "notificationId")
        .create()?;
    Ok(consumer)
}
