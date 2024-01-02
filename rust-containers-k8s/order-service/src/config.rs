use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "defaults::port")]
    pub port: u16,
    pub database_url: String,
    pub kafka_url: String,
}

mod defaults {
    pub const fn port() -> u16 {
        8081
    }
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
