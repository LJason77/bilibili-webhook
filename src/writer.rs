use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{ErrorKind, Write},
};

use log::error;

pub fn bilili(source: &str, output: &str) {
    let log_file = format!("config/log/{source}.log");
    create_dir_all("config/bilili").unwrap_or_else(|error| {
        error!("{:?}", error);
    });
    create_dir_all("config/log").unwrap_or_else(|error| {
        error!("{:?}", error);
    });
    let mut file =
        OpenOptions::new().create(true).append(true).open(&log_file).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(&log_file).unwrap_or_else(|error| {
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
