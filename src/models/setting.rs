use std::io::{self, Read};

use log::error;
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
	#[inline(always)]
	pub fn new(path: &str) -> Settings {
		let path = std::path::Path::new(path);
		let display = path.display();

		// 用只读方式打开文件
		let mut file = std::fs::File::open(&path).unwrap_or_else(|error| {
			if error.kind() == io::ErrorKind::NotFound {
				error!("配置文件不存在！请先创建：{}", display);
				panic!("{}", error);
			} else {
				error!("无法打开 {}：{}", display, error);
				panic!();
			}
		});

		// 读取文件内容到字符串
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap_or_else(|error| {
			error!("无法读取 {}：{}", display, error);
			panic!();
		});

		// 从字符串解析配置
		let mut config = config::Config::new();
		config
			.merge(config::File::from_str(&content, config::FileFormat::Toml))
			.unwrap_or_else(|error| {
				error!("解析配置文件错误：{}", error);
				panic!()
			});

		// 转换为 Settings
		config.try_into().unwrap_or_else(|error| {
			error!("转换配置文件错误：{}", error);
			panic!();
		})
	}
}
