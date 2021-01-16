use serde::Deserialize;
use serde_xml_rs::{from_reader, Error};

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
	pub fn new(url: &str) -> Result<Self, Error> {
		let res = reqwest::blocking::get(url).unwrap_or_else(|error| {
			if error.is_request() {
				error!("请求错误，请检查配置!");
				panic!(error);
			} else {
				panic!(error);
			}
		});
		let body = res.text().unwrap();
		from_reader(body.as_bytes())
	}
}
