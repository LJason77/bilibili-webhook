use std::{
    error::Error,
    fs::OpenOptions,
    io::{self, Write},
    process::Command,
    sync::OnceLock,
    time::Duration,
};

use bili_core::{
    fav::{FavContent, FavContentDetailList},
    BiliCore,
};
use log::info;
use tokio::{runtime::Runtime, time::sleep};

use crate::{
    config::Config,
    sqlite::{Content, Database, Source},
};

pub struct FeedUpdater {
    db: Database,
    core: BiliCore,
    feed: Config,
    fav_title: String,
    source_id: OnceLock<u32>,
}

impl FeedUpdater {
    pub fn new(db_path: &str, feed: Config) -> Result<Self, Box<dyn Error>> {
        let db = Database::new(db_path)?;
        let core = BiliCore::new();
        let fav_title = String::new();
        let source_id = OnceLock::new();
        Ok(FeedUpdater { db, core, feed, fav_title, source_id })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async move {
            loop {
                if let Err(e) = self.update().await {
                    log::error!("获取收藏夹 {} 失败：{e}", self.feed.media_id);
                }

                sleep(Duration::from_secs(self.feed.interval * 60)).await;
            }
        });

        Ok(())
    }
}

impl FeedUpdater {
    async fn update(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(sessdata) = Config::sessdata() {
            self.core.set_sessdata(sessdata);
        }

        let fav = self.fetch(self.feed.media_id).await?;
        self.fav_title = fav.info.title;

        let mut is_update = false;
        for content in fav.medias {
            // 检查是否失效
            if let Err(e) = content.validate() {
                log::error!("{e}");
                continue;
            }

            match self.process_item(&content) {
                Ok(success) if success => is_update = true,
                Ok(_) => {}
                Err(e) => log::error!("处理 {} 失败：{e}", content.bvid),
            }
        }

        info!("{} {}", &self.fav_title, if is_update { "更新完成" } else { "没有更新" });

        Ok(())
    }

    async fn fetch(&self, media_id: u64) -> Result<FavContentDetailList, Box<dyn Error>> {
        let fav = self.core.fav_resource_list(media_id, None, None).await?;
        self.source_id.get_or_init(|| Source::insert(&self.db.conn, &fav.info.id.to_string(), &fav.info.title).unwrap_or(0));
        Ok(fav)
    }

    fn process_item(&self, content: &FavContent) -> Result<bool, Box<dyn Error>> {
        let link = format!("https://www.bilibili.com/video/{}", &content.bvid);
        // 检查是否已存在
        if Content::exists(&self.db.conn, &link)? {
            return Ok(false);
        }

        info!("更新了一个新视频：{}", &content.title);

        // 执行外部命令
        let success = self.execute_command(&link)?;
        if success {
            info!("\"{}\" 下载成功", &content.title);
            // 插入数据库
            Content::insert(&self.db.conn, *self.source_id.get().unwrap_or(&0), &link, &content.title)?;

            return Ok(true);
        }

        Err(format!("\"{link}\" 下载失败").into())
    }

    /// 执行外部命令
    fn execute_command(&self, link: &str) -> Result<bool, Box<dyn Error>> {
        let mut cmd = Command::new("yutto");

        // 添加参数
        let args = self.feed.option.split(' ');
        for arg in args {
            cmd.arg(arg);
        }
        // 添加 sessdata
        if let Some(sessdata) = Config::sessdata() {
            cmd.arg("-c").arg(sessdata);
        }
        // 添加下载目录
        cmd.args(["-d", &format!("downloads/{}", self.feed.path)]);
        // bvid
        cmd.arg(link);

        let output = cmd.output()?;
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        log_to_file(&self.fav_title, &format!("{}: {link}", self.fav_title), &stdout, &stderr)?;

        Ok(success)
    }
}

/// 记录日志到文件
fn log_to_file(fav_title: &str, link: &str, stdout: &str, stderr: &str) -> io::Result<()> {
    for (path, out) in [("stdout", stdout), ("stderr", stderr)] {
        if !out.is_empty() {
            let mut file = OpenOptions::new().create(true).append(true).open(format!("config/log/{path}_{fav_title}.log"))?;
            file.write_all(format!("{link}\n{out}\n").as_bytes())?;
        }
    }

    Ok(())
}
