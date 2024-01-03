use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "defaults::port")]
    pub port: u16,
    pub database_url: String,
    pub kafka_url: String,
    #[serde(default = "defaults::topic")]
    pub kafka_topic: String,
    #[serde(default = "defaults::inventory_url")]
    pub inventory_url: String,
}

mod defaults {
    pub const fn port() -> u16 {
        8081
    }

    pub fn topic() -> String {
        "orders".to_string()
    }

    pub fn inventory_url() -> String {
        "http://inventory-service/api/inventory".to_string()
    }
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
