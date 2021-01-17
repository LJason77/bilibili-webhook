use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::rss::Rss;
use crate::setting::Feed;
use crate::sqlite::{Content, Source};
use crate::{sqlite, writer};

pub fn update(feed: Feed) {
	let connection = sqlite::open();

	loop {
		let rss = Rss::new(&feed.url);

		// 订阅源
		let source = Source::query_where(&connection, &feed.url).unwrap_or_else(|_| {
			let title = &rss.channel.title[0..&rss.channel.title.len() - 20];
			Source::insert(&connection, &feed.url, &title)
		});

		let mut is_update = false;

		// 内容
		for item in &rss.channel.item {
			if Content::query_where(&connection, &item.link).is_err() {
				// 返回错误，说明数据库没有这个内容，所以要更新
				info!("[{}] 更新了一个新视频：{}", &source.title, &item.title);
				// 下载新视频
				match download(&item.link, &feed) {
					Ok(output) => {
						let out = output.wait_with_output().unwrap();
						let out = String::from_utf8_lossy(&out.stdout);
						for line in out.split('\n') {
							writer::bilili(&source.title, line);
						}
						info!("\"{}\" 下载成功", &item.title);
						// 下载成功才在数据库添加内容
						Content::insert(&connection, &source.id, &item.link, &item.title);
						is_update = true;
					}
					Err(error) => {
						error!("{}", error);
					}
				}
			}
		}

		if !is_update {
			info!("[{}] 没有更新！", &source.title);
		}

		// 线程休眠
		let interval = &feed.interval * 60;
		thread::sleep(Duration::from_secs(interval));
	}
}

#[inline(always)]
fn download(url: &str, feed: &Feed) -> std::io::Result<Child> {
	let mut cmd = Command::new("bilili");
	let args = feed.option.split(' ');
	for arg in args {
		cmd.arg(arg);
	}

	// 从系统环境变量获取 sessdata
	let sessdata = std::env::var_os("SESSDATA");
	if sessdata != None {
		cmd.args(&["-c", sessdata.unwrap().to_str().unwrap()]);
	}

	cmd.arg("-y")
		.args(&["-d", &format!("downloads/{}", &feed.path)])
		.arg(url)
		.stdout(Stdio::piped())
		.spawn()
}
