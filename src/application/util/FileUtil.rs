use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use mime_guess::from_path;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::SystemTime;
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

pub fn Type(fileName: String) -> String {
    from_path(fileName).first_or_octet_stream().to_string()
}

pub fn date(fileName: String) -> SystemTime {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "date", "fs::metadata", Box::new(fileInfo.unwrap_err())));
        return SystemTime::now();
    }

    let createTime = fileInfo.unwrap().created();
    if createTime.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "date", "createTime", Box::new(createTime.unwrap_err())));
        return SystemTime::now();
    }

    createTime.unwrap()
}

pub fn modDate(fileName: String) -> SystemTime {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "modDate", "fs::metadata", Box::new(fileInfo.unwrap_err())));
        return SystemTime::now();
    }

    let modTime = fileInfo.unwrap().modified();
    if modTime.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "modDate", "modTime", Box::new(modTime.unwrap_err())));
        return SystemTime::now();
    }

    modTime.unwrap()
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

pub fn read(fileName: String) -> String {
    let result = fs::read_to_string(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "read", "fs::read_to_string", Box::new(result.unwrap_err())));
        return "".to_string();
    }

    result.unwrap()
}

pub fn readByLine(fileName: String) -> Vec<String> {
    let file = File::open(fileName);
    if file.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "readByLine", "File::open", Box::new(file.unwrap_err())));
        return Vec::new();
    }

    let reader = BufReader::new(file.unwrap());
    let mut lstLine = Vec::new();
    for line in reader.lines() {
        if line.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "readByLine", "line", Box::new(line.unwrap_err())));
            continue;
        }

        lstLine.push(line.unwrap())
    }

    lstLine
}

pub fn write(fileName: String, content: String) {
    let file = File::create(fileName);
    if file.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "write", "File::open", Box::new(file.unwrap_err())));
        return;
    }

    let result = file.unwrap().write_all(content.as_bytes());
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "write", "file.unwrap().write", Box::new(result.unwrap_err())));
    }
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

pub fn modContent(path: String, regStr: String, isAll: bool, value: String) {
    modifyContent(path, regStr, isAll, &|_lstMatch: Captures| value.clone())
}

pub fn modifyContent(path: String, regStr: String, isAll: bool, valueFunc: &dyn Fn(Captures) -> String) {
    let content = read(path.clone());
    let mut contentBreak = "\n";
    if content.contains("\r\n") {
        contentBreak = "\r\n";
    }
    let mut updateFlag = false;
    let mut lstLine: Vec<String> = Vec::new();
    let regex = Regex::new(regStr.as_str()).unwrap();
    let lines: Vec<&str> = content.split(contentBreak).collect();
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        if (!isAll && updateFlag) || !regex.is_match(line) {
            lstLine.push(lines.get(i).unwrap().to_string());
            continue;
        }
        let lstMatch = regex.captures(line).unwrap();
        if lstMatch.len() == 1 {
            continue;
        }
        updateFlag = true;
        let matchStr = lstMatch.get(1).unwrap().as_str();
        lstLine.push(line.replace(matchStr, valueFunc(lstMatch).as_str()));
    }
    write(path, lstLine.join(contentBreak))
}
