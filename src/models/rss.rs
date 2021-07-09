use log::error;
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

fn get(url: &str, mut retry: i8) -> reqwest::blocking::Response {
    reqwest::blocking::get(url).unwrap_or_else(|error| {
        error!("请求失败，请检查配置和网络!");
        log::info!("get retry{:?}", retry);
        error!("{:?}", error);
        if retry == 0 {
            error!("源 {} 更新失败，暂停更新！", url);
            panic!()
        } else {
            let interval = 15;
            log::warn!("{} 秒后进行第 {} 次重试：{}", interval, 6 - retry, url);
            retry -= 1;
            std::thread::sleep(std::time::Duration::from_secs(interval));
            get(url, retry)
        }
    })
}

impl Rss {
    #[inline(always)]
    pub fn new(url: &str) -> Self {
        let retry: i8 = 5;
        let res = get(url, retry);
        let body = res.text().unwrap();
        serde_xml_rs::from_reader(body.as_bytes()).unwrap_or_else(|error| {
            error!("xml 解析失败：{:?}", error);
            panic!();
        })
    }
}
