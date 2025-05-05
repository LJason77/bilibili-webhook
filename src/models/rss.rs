use std::{thread::sleep, time::Duration};

use log::{error, info, warn};
use quick_xml::de::from_str;
use reqwest::blocking::{self, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub description: String,
    #[serde(rename = "pubDate", default)]
    pub pub_date: String,
    pub link: String,
    pub author: String,
}

#[derive(Deserialize, Serialize)]
pub struct Channel {
    pub title: String,
    pub description: String,
    #[serde(rename = "lastBuildDate", default)]
    pub last_build_date: String,
    pub item: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct Rss {
    pub channel: Channel,
}

fn get(url: &str, mut retry: i8) -> Response {
    blocking::get(url).unwrap_or_else(|error| {
        error!("请求失败，请检查配置和网络!");
        info!("get retry {retry:?}");
        warn!("{error:?}");
        if retry == 0 {
            error!("源 {url} 更新失败，暂停更新！");
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
    #[must_use]
    pub fn new(url: &str) -> Self {
        let retry: i8 = 5;
        let res = get(url, retry);
        let body = res.text().expect("body 解析错误");

        from_str(&body).expect("xml 解析失败")
    }
}
