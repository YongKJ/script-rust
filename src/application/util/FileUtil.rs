use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use lazy_static::lazy_static;
use mime_guess::from_path;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::string::ToString;
use std::time::SystemTime;
use std::{env, fs, path};

lazy_static! {
    static ref APP_DIR: String = getAppDir();
}

fn getAppDir() -> String {
    let execPath = execPath();
    let sepStr = format!("{}{}{}", "rust", path::MAIN_SEPARATOR, "target");
    if execPath.contains(sepStr.as_str()) {
        return dir(dir(dir(execPath.as_str()).as_str()).as_str());
    }

    dir(execPath.as_str())
}

pub fn appDir() -> String {
    APP_DIR.to_string()
}

pub fn execPath() -> String {
    let curExe = env::current_exe();
    if curExe.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "execPath", "env::current_exe", curExe.unwrap_err()));
        return "".to_string();
    }

    curExe.unwrap().to_str().expect("").to_string()
}

pub fn workFolder() -> String {
    let curDir = env::current_dir();
    if curDir.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "workFolder", "env::current_dir", curDir.unwrap_err()));
        return "".to_string();
    }

    curDir.unwrap().to_str().expect("").to_string()
}

pub fn join(filePath: &str, fileName: &str) -> String {
    PathBuf::from(filePath).join(fileName).to_str().unwrap().to_string()
}

pub fn dir(fileName: &str) -> String {
    PathBuf::from(fileName).parent().unwrap().to_str().unwrap().to_string()
}

pub fn getAbsPath(names: Vec<&str>) -> String {
    let mut dir = PathBuf::from(appDir());
    for name in names {
        dir.push(name);
    }

    dir.to_str().unwrap().to_string()
}

pub fn isTest() -> bool {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return false;
    }

    let flag = args.get(1);
    flag.expect("").as_str() == "test".to_string()
}

pub fn create(fileName: &str) {
    let result = File::create(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "create", "File::create", result.unwrap_err()));
    }
}

pub fn size(fileName: &str) -> u64 {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "size", "fs::metadata", fileInfo.unwrap_err()));
        return 0;
    }

    fileInfo.unwrap().len()
}

pub fn sizeFolder(fileName: &str) -> u64 {
    let mut folderSize = 0;
    let files = list(fileName);
    for file in files.iter() {
        let tempFileName = fileName.to_string() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        if isFolder(tempFileName.as_str()) {
            folderSize += sizeFolder(tempFileName.as_str())
        } else {
            folderSize += size(tempFileName.as_str())
        }
    }

    folderSize
}

pub fn exist(fileName: &str) -> bool {
    fs::metadata(fileName).is_ok()
}

pub fn Type(fileName: &str) -> String {
    from_path(fileName).first_or_octet_stream().to_string()
}

pub fn date(fileName: &str) -> SystemTime {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "date", "fs::metadata", fileInfo.unwrap_err()));
        return SystemTime::now();
    }

    let createTime = fileInfo.unwrap().created();
    if createTime.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "date", "createTime", createTime.unwrap_err()));
        return SystemTime::now();
    }

    createTime.unwrap()
}

pub fn modDate(fileName: &str) -> SystemTime {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "modDate", "fs::metadata", fileInfo.unwrap_err()));
        return SystemTime::now();
    }

    let modTime = fileInfo.unwrap().modified();
    if modTime.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "modDate", "modTime", modTime.unwrap_err()));
        return SystemTime::now();
    }

    modTime.unwrap()
}

pub fn isFolder(fileName: &str) -> bool {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "isFolder", "fs::metadata", fileInfo.unwrap_err()));
        return false;
    }

    fileInfo.unwrap().is_dir()
}

pub fn isFile(fileName: &str) -> bool {
    let fileInfo = fs::metadata(fileName);
    if fileInfo.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "isFolder", "fs::metadata", fileInfo.unwrap_err()));
        return false;
    }

    fileInfo.unwrap().is_file()
}

pub fn mkdir(fileName: &str) {
    let result = fs::create_dir_all(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "mkdir", "fs::create_dir_all", result.unwrap_err()));
    }
}

pub fn list(fileName: &str) -> Vec<String> {
    let files = fs::read_dir(fileName);
    if files.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "list", "fs::read_dir", files.unwrap_err()));
        return Vec::new();
    }

    let mut lstFile = Vec::new();
    for entry in files.unwrap() {
        if entry.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "list", "entry", entry.unwrap_err()));
            continue;
        }

        let entryName = entry.unwrap()
            .file_name().to_str()
            .unwrap().to_string();

        lstFile.push(entryName);
    }

    lstFile
}

pub fn read(fileName: &str) -> String {
    let result = fs::read_to_string(fileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "read", "fs::read_to_string", result.unwrap_err()));
        return "".to_string();
    }

    result.unwrap()
}

pub fn readByLine(fileName: &str) -> Vec<String> {
    let file = File::open(fileName);
    if file.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "readByLine", "File::open", file.unwrap_err()));
        return Vec::new();
    }

    let reader = BufReader::new(file.unwrap());
    let mut lstLine = Vec::new();
    for line in reader.lines() {
        if line.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "readByLine", "line", line.unwrap_err()));
            continue;
        }

        lstLine.push(line.unwrap())
    }

    lstLine
}

pub fn write(fileName: &str, content: &str) {
    let file = File::create(fileName);
    if file.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "write", "File::open", file.unwrap_err()));
        return;
    }

    let result = file.unwrap().write_all(content.as_bytes());
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "write", "file.unwrap().write", result.unwrap_err()));
    }
}

pub fn Move(srcFileName: &str, desFileName: &str) {
    let result = fs::rename(srcFileName, desFileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "Move", "fs::rename", result.unwrap_err()));
    }
}

pub fn copy(srcFileName: &str, desFileName: &str) {
    let result = fs::copy(srcFileName, desFileName);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("FileUtil", "copy", "fs::copy", result.unwrap_err()));
    }
}

pub fn copyFolder(srcFolderName: &str, desFolderName: &str) {
    let files = list(srcFolderName);
    for file in files.iter() {
        let srcNewFileName = srcFolderName.to_string() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        let desNewFileName = desFolderName.to_string() + path::MAIN_SEPARATOR.to_string().as_str() + file;
        if isFolder(srcNewFileName.as_str()) {
            mkdir(desNewFileName.as_str());
            copyFolder(srcNewFileName.as_str(), desNewFileName.as_str());
        } else {
            copy(srcNewFileName.as_str(), desNewFileName.as_str());
        }
    }
}

pub fn delete(fileName: &str) {
    if !exist(fileName) {
        return;
    }

    if isFolder(fileName) {
        let result = fs::remove_dir_all(fileName);
        if result.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "delete", "fs::remove_dir_all", result.unwrap_err()));
        }
    } else {
        let result = fs::remove_file(fileName);
        if result.is_err() {
            LogUtil::loggerLine(Log::of("FileUtil", "delete", "fs::remove_file", result.unwrap_err()));
        }
    }
}

pub fn modFile(path: &str, regStr: &str, isAll: bool, value: &str) {
    modifyFile(path, regStr, isAll, |lstMatch: &Captures|
        lstMatch.get(0).unwrap().as_str().replace(
            lstMatch.get(1).unwrap().as_str(), value))
}

pub fn modifyFile(path: &str, regStr: &str, isAll: bool, valueFunc: impl Fn(&Captures) -> String) {
    let mut content = read(path);
    let regex = Regex::new(regStr).unwrap();
    if isAll {
        content = regex.replace_all(content.as_str(), |lstMatch: &Captures| valueFunc(lstMatch)).to_string();
    } else {
        content = regex.replace(content.as_str(), |lstMatch: &Captures| valueFunc(lstMatch)).to_string();
    }
    write(path, content.as_str());
}

pub fn modContent(path: &str, regStr: &str, isAll: bool, value: &str) {
    modifyContent(path, regStr, isAll, |lstMatch: &Captures|
        lstMatch.get(0).unwrap().as_str().replace(
            lstMatch.get(1).unwrap().as_str(), value))
}

pub fn modifyContent(path: &str, regStr: &str, isAll: bool, valueFunc: impl Fn(&Captures) -> String) {
    let content = read(path);
    let mut contentBreak = "\n";
    if content.contains("\r\n") {
        contentBreak = "\r\n";
    }
    let mut updateFlag = false;
    let mut lstLine: Vec<String> = Vec::new();
    let regex = Regex::new(regStr).unwrap();
    let lines: Vec<&str> = content.split(contentBreak).collect();
    for line in lines {
        if (!isAll && updateFlag) || !regex.is_match(line) {
            lstLine.push(line.to_string());
            continue;
        }
        let lstMatch = regex.captures(line).unwrap();
        if lstMatch.len() == 1 {
            continue;
        }
        updateFlag = true;
        lstLine.push(regex.replace_all(line, valueFunc(&lstMatch)).to_string());
    }
    write(path, lstLine.join(contentBreak).as_str())
}
