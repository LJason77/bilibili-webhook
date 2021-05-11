mod models;
mod update;
mod writer;

fn main() {
	log4rs::init_file("log.yml", Default::default()).unwrap();
	log::info!("RUA！");

	let settings = crate::models::setting::Settings::new("config/config.toml");

	// 提取需要更新的订阅
	let mut update_feeds = Vec::new();
	for feed in settings.feed {
		if feed.update {
			update_feeds.push(feed);
		}
	}

	// 根据订阅数量创建线程
	let num_threads = update_feeds.len();
	if num_threads > 0 {
		let pool = threadpool::ThreadPool::new(num_threads);
		for feed in update_feeds {
			pool.execute(move || update::update(feed));
		}
		pool.join();
	}
}
