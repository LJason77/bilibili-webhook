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
    #[must_use]
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        let setting = read_to_string(&path).expect("配置文件不存在！请先创建配置文件！");
        toml::from_str(&setting).expect("配置解析失败！")
    }
}
