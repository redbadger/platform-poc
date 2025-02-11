use ::futures::StreamExt as _;
use dotenv::dotenv;
use notification_service::{
    config::Config,
    core::{Logger, OrderPlacedEvent, Service},
};

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct TracingLogger;

impl Logger for TracingLogger {
    fn info(&self, msg: &str) {
        info!("{}", msg);
    }
}

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

    let service = Service::new(TracingLogger);

    let client = async_nats::connect(config.nats_url).await?;

    let mut subscriber = client.subscribe(config.nats_topic).await?;

    while let Some(message) = subscriber.next().await {
        if let Ok(order) = serde_json::from_slice::<OrderPlacedEvent>(&message.payload) {
            service.recv(order);
        } else {
            info!("Error decoding notification order: {:?}", message);
        }
    }

    Ok(())
}
