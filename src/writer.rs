use std::{
	fs,
	io::{self, Write},
};

use log::error;

#[inline(always)]
pub fn bilili(source: &str, output: &str) {
	let log_file = format!("config/bilili/{}.log", source);
	fs::create_dir_all("config/bilili").unwrap_or_else(|error| {
		error!("{:?}", error);
	});
	let mut file = fs::OpenOptions::new()
		.append(true)
		.open(&log_file)
		.unwrap_or_else(|error| {
			if error.kind() == io::ErrorKind::NotFound {
				fs::File::create(&log_file).unwrap_or_else(|error| {
					error!("{:?}", error);
					panic!();
				})
			} else {
				error!("{:?}", error);
				panic!();
			}
		});
	file.write_all(output.as_bytes()).expect("写入失败");
	file.write_all("\n".as_bytes()).expect("写入失败");
}
