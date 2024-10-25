use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use std::fs::File;
use std::{env, fs, path};

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

pub fn sizeFolder(fileName: String) -> u64 {
    let mut folderSize = 0;
    let files = list(fileName.clone());
    for file in files.iter() {
        let tempFileName = fileName.clone() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        if isFolder(tempFileName.clone()) {
            folderSize += sizeFolder(tempFileName)
        } else {
            folderSize += size(tempFileName)
        }
    }

    folderSize
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

pub fn mkdir(fileName: String) {
    let result = fs::create_dir_all(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "mkdir", "fs::create_dir_all", Box::new(result.unwrap_err())));
    }
}

pub fn list(fileName: String) -> Vec<String> {
    let files = fs::read_dir(fileName);
    if files.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "list", "fs::read_dir", Box::new(files.unwrap_err())));
        return Vec::new();
    }

    let mut lstFile = Vec::new();
    for entry in files.unwrap() {
        if entry.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "list", "entry", Box::new(entry.unwrap_err())));
            continue;
        }

        let entryName = entry.unwrap()
            .file_name().to_str()
            .unwrap().to_string();

        lstFile.push(entryName);
    }

    lstFile
}

pub fn Move(srcFileName: String, desFileName: String) {
    let result = fs::rename(srcFileName, desFileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "Move", "fs::rename", Box::new(result.unwrap_err())));
    }
}

pub fn copy(srcFileName: String, desFileName: String) {
    let result = fs::copy(srcFileName, desFileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "copy", "fs::copy", Box::new(result.unwrap_err())));
    }
}

pub fn copyFolder(srcFolderName: String, desFolderName: String) {
    let files = list(srcFolderName.clone());
    for file in files.iter() {
        let srcNewFileName = srcFolderName.clone() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        let desNewFileName = desFolderName.clone() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        if isFolder(srcNewFileName.clone()) {
            mkdir(desNewFileName.clone());
            copyFolder(srcNewFileName, desNewFileName);
        } else {
            copy(srcNewFileName, desNewFileName);
        }
    }
}

pub fn delete(fileName: String) {
    if !exist(fileName.clone()) {
        return;
    }

    if isFolder(fileName.clone()) {
        let result = fs::remove_dir_all(fileName);
        if result.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "delete", "fs::remove_dir_all", Box::new(result.unwrap_err())));
        }
    } else {
        let result = fs::remove_file(fileName);
        if result.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "delete", "fs::remove_file", Box::new(result.unwrap_err())));
        }
    }
}
