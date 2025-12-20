use std::{env::var_os, error::Error, fs::read_to_string};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub media_id: u64,
    pub interval: u64,
    pub option: String,
    pub path: String,
    pub update: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let config = read_to_string(path)?;
        let configs: Vec<Self> = serde_json::from_str(&config)?;
        Ok(configs)
    }

    pub fn sessdata() -> Option<String> {
        // 先从本地读取 sessdata
        if let Ok(sessdata) = read_to_string("config/SESSDATA.txt") {
            Some(sessdata)
        } else if let Some(sessdata) = var_os("SESSDATA") {
            // 如果本地不存在，则从系统环境变量获取 sessdata
            sessdata.into_string().ok()
        } else {
            None
        }
    }
}
