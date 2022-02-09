use std::{thread::sleep, time::Duration};

use log::{error, info, warn};
use quick_xml::de::from_str;
use reqwest::blocking::{self, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    pub title: String,
    pub description: String,
    #[serde(rename = "pubDate", default)]
    pub pub_date: String,
    pub link: String,
    pub author: String,
}

#[derive(Deserialize)]
pub struct Channel {
    pub title: String,
    pub description: String,
    #[serde(rename = "lastBuildDate", default)]
    pub last_build_date: String,
    pub item: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

fn get(url: &str, mut retry: i8) -> Response {
    blocking::get(url).unwrap_or_else(|error| {
        error!("请求失败，请检查配置和网络!");
        info!("get retry {:?}", retry);
        error!("{:?}", error);
        if retry == 0 {
            error!("源 {} 更新失败，暂停更新！", url);
            panic!()
        } else {
            let interval = 15;
            warn!("{} 秒后进行第 {} 次重试：{}", interval, 6 - retry, url);
            retry -= 1;
            sleep(Duration::from_secs(interval));
            get(url, retry)
        }
    })
}

impl Rss {
    pub fn new(url: &str) -> Self {
        let retry: i8 = 5;
        let res = get(url, retry);
        let body = res.text().unwrap();

        from_str(&body).unwrap_or_else(|error| {
            panic!("xml 解析失败：{:?}\n", error);
        })
    }
}
