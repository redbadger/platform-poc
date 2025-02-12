use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub nats_url: String,
    #[serde(default = "defaults::topic")]
    pub nats_topic: String,
    #[serde(default = "defaults::port")]
    pub port: u16,
}

mod defaults {
    pub fn topic() -> String {
        "orders".to_string()
    }

    pub const fn port() -> u16 {
        8082
    }
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
