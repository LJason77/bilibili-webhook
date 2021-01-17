use std::thread;
use std::time::Duration;

use reqwest::blocking::Response;
use serde::Deserialize;
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
pub struct Item {
	pub title: String,
	pub description: String,
	#[serde(rename = "pubDate", default)]
	pub pub_date: String,
	pub link: String,
	pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
	pub title: String,
	pub description: String,
	#[serde(rename = "lastBuildDate", default)]
	pub last_build_date: String,
	pub item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Rss {
	pub channel: Channel,
}

impl Rss {
	#[inline(always)]
	pub fn new(url: &str) -> Self {
		let retry: i8 = 5;
		let res = get(url, retry);
		let body = res.text().unwrap();
		from_reader(body.as_bytes()).unwrap_or_else(|error| {
			error!("xml 解析失败：{:?}", error);
			panic!();
		})
	}
}

#[inline(always)]
fn get(url: &str, mut retry: i8) -> Response {
	reqwest::blocking::get(url).unwrap_or_else(|error| {
		error!("请求失败，请检查配置和网络!");
		info!("get retry{:?}", retry);
		error!("{:?}", error);
		if retry == 0 {
			error!("源 {} 更新失败，暂停更新！", url);
			panic!()
		} else {
			let interval = 15;
			warn!("{} 秒后进行第 {} 次重试：{}", interval, 6 - retry, url);
			retry -= 1;
			thread::sleep(Duration::from_secs(interval));
			get(url, retry)
		}
	})
}
