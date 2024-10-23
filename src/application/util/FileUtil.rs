use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use std::env;
use std::fs::File;
use std::io::Error;

pub fn workFolder() -> String {
    env::current_dir().unwrap()
        .to_str().expect("").to_string()
}

pub fn create(fileName: String) {
    let result = File::create(fileName);
    if result.is_err() {
        LogUtil::loggerLine::<Error>(Log::of("FileUtil", "create", "File::create", Box::new(result.unwrap_err())));
    }
}