use backoff::{retry, ExponentialBackoff};
use dotenv::dotenv;
use order_service::{api::server, config::Config};
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, Consumer as SecondConsumer, DefaultConsumerContext},
    metadata::Metadata,
    producer::FutureProducer,
    ClientConfig,
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "order_service=debug,tower_http=debug,axum::rejection=trace".into()
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

    let producer = create_producer(&kafka)?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    server::create(config, pool, producer).await?;
    Ok(())
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

fn create_producer(config: &ClientConfig) -> anyhow::Result<FutureProducer> {
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
