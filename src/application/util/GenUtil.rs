use crate::application::pojo::dto::Log::Log;
use crate::application::util::{FileUtil, LogUtil};
use regex::{Captures, Regex};
use serde_yaml::Value;
use std::collections::HashMap;
use std::io::Write;
use std::{io, path};

pub fn readParams() -> Vec<String> {
    let flushResult = io::stdout().flush();
    if flushResult.is_err() {
        LogUtil::loggerLine(Log::of("GenUtil", "readParams", "io::stdout().flush()", flushResult.unwrap_err()));
    }
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    if result.is_err() {
        LogUtil::loggerLine(Log::of("GenUtil", "readParams", "io::stdin().read_line", result.unwrap_err()));
        return Vec::new();
    }
    input.trim().split(" ").map(String::from).collect()
}

pub fn strToUsize(str: &str) -> usize {
    str.parse().unwrap()
}

pub fn writeConfig(mapData: HashMap<String, Value>) {
    let content = serde_yaml::to_string(&mapData).unwrap();
    let configPath = getConfigPath();
    FileUtil::write(configPath.as_str(), content.as_str());
}

pub fn getValue(key: &str) -> String {
    getConfig().get(key).unwrap().as_str().unwrap().to_string()
}

pub fn getConfig() -> HashMap<String, Value> {
    let configPath = getConfigPath();
    let content = FileUtil::read(configPath.as_str());

    let mapData = serde_yaml::from_str(content.as_str());
    if mapData.is_err() {
        LogUtil::loggerLine(Log::of("GenUtil", "getConfig", "err", mapData.unwrap_err()));
        return HashMap::new();
    }

    mapData.unwrap()
}

pub fn getConfigPath() -> String {
    let config = getYaml();
    let execDir = FileUtil::dir(FileUtil::execPath().as_str());
    let mut path = FileUtil::join(execDir.as_str(), config.clone().as_str());
    if FileUtil::exist(path.clone().as_str()) {
        return path;
    }
    path = FileUtil::getAbsPath(false, vec![config.as_str()]);
    if FileUtil::exist(path.clone().as_str()) {
        return path;
    }
    FileUtil::getAbsPath(false, vec!["src", "assets", config.as_str()])
}

pub fn getYaml() -> String {
    let execYaml = getExecYaml();
    if FileUtil::exist(execYaml.as_str()) {
        return execYaml;
    }
    getYamlByContent()
}

pub fn getExecYaml() -> String {
    let execPath = FileUtil::execPath();
    let mut index = execPath.rfind(path::MAIN_SEPARATOR).unwrap();
    let execName = execPath.get(index + 1..execPath.len()).unwrap();
    if !execName.contains(".") {
        return toLine(execName) + ".yaml";
    }
    index = execName.find(".").unwrap();
    toLine(execName.get(0..index).unwrap()) + ".yaml"
}

pub fn getYamlByContent() -> String {
    let mut appName = "Application.rs";
    if FileUtil::isTest() {
        appName = "ApplicationTest.rs";
    }

    let appPath = FileUtil::getAbsPath(false, vec!["src", "application", appName]);
    let regex = Regex::new("\\s+(\\S+)::run\\(\\)").unwrap();
    let lines = FileUtil::readByLine(appPath.as_str());
    for line in lines {
        if line.contains("//") {
            continue;
        }
        if !regex.is_match(line.as_str()) {
            continue;
        }
        let lstMatch = regex.captures(line.as_str()).unwrap();
        if lstMatch.len() == 1 {
            continue;
        }
        return toLine(lstMatch.get(1).unwrap().as_str()) + ".yaml";
    }
    "".to_string()
}

pub fn toHump(name: &str) -> String {
    let subName = &name[1..name.len()];
    let regex = Regex::new("\\-(\\w)").unwrap();
    let firstUpperLetter = name[0..1].to_uppercase();

    firstUpperLetter + regex.replace_all(subName, |lstMatch: &Captures|
        lstMatch.get(1).unwrap().as_str().replace("-", "").to_uppercase()).as_ref()
}

pub fn toLine(name: &str) -> String {
    let subName = &name[1..name.len()];
    let regex = Regex::new("([A-Z])").unwrap();
    let firstLowerLetter = name[0..1].to_lowercase();

    firstLowerLetter + regex.replace_all(subName, |lstMatch: &Captures|
        "-".to_string() + lstMatch.get(1).unwrap().as_str().to_lowercase().as_str()).as_ref()
}