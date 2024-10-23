use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use std::fs::File;
use std::{env, fs};

pub fn workFolder() -> String {
    let curDir = env::current_dir();
    if curDir.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "workFolder", "env::current_dir", Box::new(curDir.unwrap_err())));
        return "".to_string();
    }

    curDir.unwrap().to_str().expect("").to_string()
}

pub fn create(fileName: String) {
    let result = File::create(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "create", "File::create", Box::new(result.unwrap_err())));
    }
}

pub fn size(fileName: String) -> u64 {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "size", "fs::metadata", Box::new(fileInfo.unwrap_err())));
        return 0;
    }

    fileInfo.unwrap().len()
}

pub fn exist(fileName: String) -> bool {
    fs::metadata(fileName).is_ok()
}

pub fn isFolder(fileName: String) -> bool {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "isFolder", "fs::metadata", Box::new(fileInfo.unwrap_err())));
        return false;
    }

    fileInfo.unwrap().is_dir()
}

pub fn isFile(fileName: String) -> bool {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "isFolder", "fs::metadata", Box::new(fileInfo.unwrap_err())));
        return false;
    }

    fileInfo.unwrap().is_file()
}