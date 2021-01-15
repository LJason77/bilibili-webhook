use quick_xml::de::from_str;
use quick_xml::DeError;
use serde::Deserialize;

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
	pub fn new(url: &str) -> Result<Self, DeError> {
		let res = reqwest::blocking::get(url).unwrap_or_else(|error| {
			if error.is_request() {
				error!("请求错误，请检查配置!");
				panic!(error);
			} else {
				panic!(error);
			}
		});
		let body = res.text().unwrap();
		from_str(&body)
	}
}
