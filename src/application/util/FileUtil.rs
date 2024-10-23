use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use std::env;
use std::fs::File;
use std::io::Error;

pub fn workFolder() -> String {
    let curDir = env::current_dir();
    if curDir.is_err() {
        LogUtil::loggerLine::<Error>(Log::of("FileUtil", "workFolder", "env::current_dir", Box::new(curDir.unwrap_err())));
        return "".to_string();
    }

    curDir.unwrap().to_str().expect("").to_string()
}

pub fn create(fileName: String) {
    let result = File::create(fileName);
    if result.is_err() {
        LogUtil::loggerLine::<Error>(Log::of("FileUtil", "create", "File::create", Box::new(result.unwrap_err())));
    }
}