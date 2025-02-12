use dotenv::dotenv;
use futures::{future::join_all, StreamExt as _};
use tokio::spawn;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use notification_service::{
    config::Config,
    core::{Logger, OrderPlacedEvent, Service},
    server,
};

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

    let http_handler = spawn(server::create(config.port));
    let message_handler = spawn(async move {
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

        anyhow::Result::<()>::Ok(())
    });

    join_all([http_handler, message_handler])
        .await
        .into_iter()
        .flatten()
        .collect()
}
