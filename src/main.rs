use std::{
    env::var_os,
    error::Error,
    fs::{create_dir_all, write},
    path::Path,
    thread,
};

use log::{error, info};
use log4rs::config::Deserializers;

use crate::{config::Config, update::FeedUpdater};

mod config;
mod sqlite;
mod update;

// 避免使用 musl 默认配置器
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance/
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("log.yml", Deserializers::default())?;
    info!("RUA！");

    let configs = Config::load("config/config.json")?;

    // 判断是否存在 sessdata 文件
    let sessdata = Path::new("config/SESSDATA.txt");
    if !sessdata.exists()
        // 从系统环境变量获取 sessdata，再保存到本地
        && let Some(sessdata) = var_os("SESSDATA")
        && let Err(error) = write("config/SESSDATA.txt", sessdata.into_encoded_bytes())
    {
        error!("写入 SESSDATA 失败: {error}");
    }

    // 创建所需目录
    create_dir_all("config/log")?;

    // 提取需要更新的订阅
    let update_feeds: Vec<Config> = configs.into_iter().filter(|feed| feed.update).collect();

    // 根据订阅数量创建线程
    let mut handles = vec![];
    for feed in update_feeds {
        // 为每个feed创建独立的数据库连接
        let handle = thread::spawn(move || match FeedUpdater::new("config/date.db", feed) {
            Ok(mut updater) => {
                if let Err(e) = updater.run() {
                    log::error!("{e}");
                }
            }
            Err(e) => {
                error!("无法创建 FeedUpdater: {e:?}");
            }
        });

        handles.push(handle);
    }

    // 等待所有线程完成（实际上会一直运行）
    for handle in handles {
        if let Err(e) = handle.join() {
            error!("线程错误: {e:?}");
        }
    }

    Ok(())
}
