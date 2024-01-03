use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka_url: String,
    #[serde(default = "defaults::topic")]
    pub kafka_topic: String,
}

mod defaults {
    pub fn topic() -> String {
        "orders".to_string()
    }
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
