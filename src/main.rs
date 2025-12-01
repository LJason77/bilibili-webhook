#![deny(clippy::pedantic)]

use std::{env::var_os, error::Error, fs::write, path::Path};

use log4rs::config::Deserializers;
use threadpool::ThreadPool;

use crate::config::{Config, Feed};

mod config;
mod models;
mod update;
mod writer;

fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("log.yml", Deserializers::default())?;
    log::info!("RUA！");

    let config = Config::load("config/config.toml")?;

    // 判断是否存在 sessdata 文件
    let sessdata = Path::new("config/SESSDATA.txt");
    // 从系统环境变量获取 sessdata，再保存到本地
    if !sessdata.exists()
        && let Some(sessdata) = var_os("SESSDATA")
        && let Err(error) = write("config/SESSDATA.txt", sessdata.into_encoded_bytes())
    {
        log::error!("写入 SESSDATA 失败: {error}");
    }

    // 提取需要更新的订阅
    let update_feeds: Vec<Feed> = config.feed.into_iter().filter(|feed| feed.update).collect();

    // 根据订阅数量创建线程
    if !update_feeds.is_empty() {
        let pool = ThreadPool::new(update_feeds.len());
        for feed in update_feeds {
            pool.execute(move || update::update(&feed));
        }
        pool.join();
    }
    Ok(())
}
