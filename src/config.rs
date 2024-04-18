use std::{error::Error, fs::read_to_string};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub feed: Vec<Feed>,
}

#[derive(Deserialize)]
pub struct Feed {
    pub url: String,
    pub interval: u64,
    pub option: String,
    pub path: String,
    pub update: bool,
    pub delay: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = read_to_string(path)?;
        let config: Self = toml::from_str(&config)?;
        Ok(config)
    }
}
