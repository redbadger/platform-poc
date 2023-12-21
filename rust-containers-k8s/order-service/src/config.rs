use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u32,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
