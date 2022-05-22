#![deny(clippy::pedantic)]

use log4rs::config::Deserializers;
use threadpool::ThreadPool;

use models::setting::Settings;

use crate::models::Feed;

mod models;
mod update;
mod writer;

fn main() {
    log4rs::init_file("log.yml", Deserializers::default()).unwrap();
    log::info!("RUA！");

    let settings = Settings::new("config/config.toml");

    // 提取需要更新的订阅
    let update_feeds: Vec<Feed> = settings.feed.into_iter().filter(|feed| feed.update).collect();

    // 根据订阅数量创建线程
    if !update_feeds.is_empty() {
        let pool = ThreadPool::new(update_feeds.len());
        for feed in update_feeds {
            pool.execute(move || update::update(&feed));
        }
        pool.join();
    }
}
