use std::{
    env::var_os,
    fs::read_to_string,
    io::Result,
    process::{Child, Command, Stdio},
};

use log::{error, info};

use crate::{
    config::Feed,
    models::{sqlite, Content, Rss, Source},
    writer,
};

pub fn update(feed: &Feed) {
    let connection = match sqlite::open() {
        Ok(conn) => conn,
        Err(err) => {
            error!("{err}");
            panic!()
        }
    };

    loop {
        let url = &feed.url;

        let rss = match Rss::new(url) {
            Ok(rss) => rss,
            Err(error) => {
                error!("获取订阅源 {url} 失败：{error}");
                panic!()
            }
        };

        let channel = &rss.channel;
        // 订阅源
        let source = Source::query_where(&connection, url).unwrap_or_else(|_| match Source::insert(&connection, url, &channel.title) {
            Ok(source) => source,
            Err(error) => {
                error!("插入订阅源失败：{error}");
                panic!();
            }
        });

        let mut is_update = false;
        let mut is_error = false;

        // 内容
        for item in &channel.item {
            // 返回 OK，说明数据库有这个内容，跳过
            if Content::query_where(&connection, &item.link).is_ok() {
                continue;
            }

            // 返回错误，说明数据库没有这个内容，更新
            info!("[{}] 更新了一个新视频：{}", &source.title, &item.title);

            // 提前处理错误情况
            let child = match download(&item.link, feed) {
                Ok(output) => output,
                Err(error) => {
                    error!("{error}");
                    is_error = true;
                    continue;
                }
            };

            writer::bilili(&source.title, &item.link);
            let output = match child.wait_with_output() {
                Ok(output) => output,
                Err(error) => {
                    error!("{error}");
                    is_error = true;
                    continue;
                }
            };

            if !output.status.success() {
                error!("{}", String::from_utf8_lossy(&output.stderr));
                is_error = true;
                continue;
            }

            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.split('\n') {
                writer::bilili(&source.title, line);
            }

            info!("\"{}\" 下载成功", &item.title);
            // 下载成功才在数据库添加内容
            if let Err(e) = Content::insert(&connection, source.id, &item.link, &item.title) {
                error!("数据库插入内容失败: {e}");
                continue;
            }
            is_update = true;
        }

        if is_error {
            error!("[{}] 存在错误，请检查！", &source.title);
        } else {
            info!("[{}] {}！", &source.title, if is_update { "已更新" } else { "没有更新" });
        }

        // 线程休眠
        let interval = &feed.interval * 60;
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}

fn download(url: &str, feed: &Feed) -> Result<Child> {
    let mut cmd = Command::new("yutto");
    let args = feed.option.split(' ');
    for arg in args {
        cmd.arg(arg);
    }

    // 先从本地读取 sessdata
    if let Ok(sessdata) = read_to_string("config/SESSDATA.txt") {
        cmd.arg("-c").arg(sessdata);
        // 如果本地不存在，则从系统环境变量获取 sessdata
    } else if let Some(sessdata) = var_os("SESSDATA") {
        cmd.arg("-c").arg(sessdata);
    }

    let download_dir = format!("downloads/{}", &feed.path);
    cmd.args(["-d", &download_dir]).arg(url).stdout(Stdio::piped()).spawn()
}
