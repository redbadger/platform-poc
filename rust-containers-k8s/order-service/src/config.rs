use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u32,
}

impl Config {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
