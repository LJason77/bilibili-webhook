use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};

#[inline(always)]
pub fn bilili(output: &str) {
	let mut file = OpenOptions::new()
		.append(true)
		.open("config/bilili.log")
		.unwrap_or_else(|error| {
			if error.kind() == ErrorKind::NotFound {
				File::create("config/bilili.log").unwrap_or_else(|error| {
					panic!("{:?}", error);
				})
			} else {
				panic!("{:?}", error);
			}
		});
	file.write_all(output.as_bytes()).expect("写入失败");
	file.write_all("\n".as_bytes()).expect("写入失败");
}
