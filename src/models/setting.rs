use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Feed {
    pub url: String,
    pub interval: u64,
    pub option: String,
    pub path: String,
    pub update: bool,
}

#[derive(Deserialize)]
pub struct Settings {
    pub feed: Vec<Feed>,
}

impl Settings {
    pub fn new(path: &str) -> Settings {
        let path = Path::new(path);
        let setting = read_to_string(&path).expect("配置文件不存在！请先创建");
        toml::from_str(&setting).unwrap()
    }
}
