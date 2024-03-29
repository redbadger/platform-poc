use backoff::{retry, ExponentialBackoff};
use dotenv::dotenv;
use notification_service::{
    config::Config,
    core::{Logger, OrderPlacedEvent, Service},
};
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, Consumer as SecondConsumer, DefaultConsumerContext, StreamConsumer},
    metadata::Metadata,
    ClientConfig, Message,
};
use std::time::Duration;
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

    let kafka = create_config(&config.kafka_url);

    // Create the topic if it does not exist
    // this leaves something to be desired, but it works for now
    match fetch_metadata(&config.kafka_topic, &kafka) {
        Err(_) => {
            create_admin_client(&kafka)?
                .create_topics(
                    &[NewTopic {
                        name: &config.kafka_topic,
                        num_partitions: 1,
                        replication: TopicReplication::Fixed(1),
                        config: vec![],
                    }],
                    &AdminOptions::default(),
                )
                .await?;
        }
        _ => (),
    };

    let consumer = create_consumer(&kafka)?;
    consumer.subscribe(&[&config.kafka_topic])?;

    let service = Service::new(TracingLogger);

    loop {
        let message = consumer.recv().await?;
        let payload = message.payload();
        let Some(message) = payload else {
            continue;
        };

        if let Ok(order) = serde_json::from_slice::<OrderPlacedEvent>(message) {
            service.recv(order);
        } else {
            info!(
                "Error decoding notification order: {:?}",
                std::str::from_utf8(message)?
            );
        }
    }
}

fn create_config(bootstrap_server: &str) -> ClientConfig {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("group.id", "my-group")
        .set("enable.partition.eof", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .to_owned()
}

fn create_admin_client(config: &ClientConfig) -> anyhow::Result<AdminClient<DefaultClientContext>> {
    Ok(config.create()?)
}

fn create_consumer(config: &ClientConfig) -> anyhow::Result<StreamConsumer> {
    Ok(config.create()?)
}

fn fetch_metadata(topic: &str, config: &ClientConfig) -> anyhow::Result<Option<Metadata>> {
    let consumer: BaseConsumer<DefaultConsumerContext> =
        config.create().expect("consumer creation failed");
    let timeout = Some(Duration::from_secs(1));

    let mut backoff = ExponentialBackoff::default();
    backoff.max_elapsed_time = Some(Duration::from_secs(5));
    retry(backoff, || {
        let metadata = consumer
            .fetch_metadata(Some(topic), timeout)
            .map_err(|e| e.to_string())?;
        if metadata.topics().len() == 0 {
            Err("metadata fetch returned no topics".to_string())?
        }
        let topic = &metadata.topics()[0];
        if topic.partitions().len() == 0 {
            Err("metadata fetch returned a topic with no partitions".to_string())?
        }
        Ok(Some(metadata))
    })
    .map_err(|e| anyhow::anyhow!("fetch_metadata failed: {}", e))
}
