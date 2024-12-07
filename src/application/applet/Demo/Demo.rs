use crate::application::pojo::dto::Log::Log;
use crate::application::util::{FileUtil, GenUtil, LogUtil};
use regex::{Captures, Regex};
use serde_yaml::Value;
use std::collections::HashMap;
use std::env;

struct Demo {
    msg: String,
}

impl Demo {
    fn new() -> Self {
        Self {
            msg: GenUtil::getValue("msg")
        }
    }

    fn test(&self) {
        println!("msg: {}", self.msg)
    }

    fn test1(&self) {
        let dir = FileUtil::workFolder();
        println!("msg: {}", dir)
    }

    fn test2(&self) {
        let fileName = "C:\\$WINRE_BACKUP_PARTITION.MARKER";
        FileUtil::create(fileName.to_string());
    }

    fn test3(&self) {
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", "Hello world!"));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", "世界，你好！".to_string()));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", 12));
    }

    fn test4(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let size = FileUtil::size(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", size));
    }

    fn test5(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let isFolder = FileUtil::isFolder(fileName.to_string());
        let isFile = FileUtil::isFile(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFolder", isFolder));
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFile", isFile));
    }

    fn test6(&self) {
        // let fileName = "C:\\Users\\admin\\Desktop\\apk";
        let fileName = "C:\\Users\\admin\\Desktop\\api-go";
        // let fileName = "C:\\Users\\Admin\\Desktop\\Database-Backup";
        let folderSize = FileUtil::sizeFolder(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test6", "folderSize", folderSize));
    }

    fn test7(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\rust-test\\rust-demo";
        FileUtil::mkdir(fileName.to_string());
    }

    fn test8(&self) {
        // let fileName = "C:\\Users\\Admin\\Desktop\\busybox-df.json";
        let fileName = "C:\\Users\\Admin\\Desktop\\api-ts.code-workspace";
        let lines = FileUtil::readByLine(fileName.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test8", "lines.len()", lines.len()));
        // let content = FileUtil::read(fileName.to_string());
        // LogUtil::loggerLine(Log::of("Demo", "test8", "content.len()", content.len()));
    }

    fn test9(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\test.code-workspace";
        FileUtil::write(fileName.to_string(), "Hello world!".to_string());
    }

    fn test10(&self) {
        let regexStr = "\\sworld";
        let regex = Regex::new(regexStr).unwrap();
        let str = "Hello world!";
        let flag = regex.is_match(str);
        LogUtil::loggerLine(Log::of("Demo", "test10", "flag", flag));
    }

    fn test11(&self) {
        let regexStr = "\\s(wor)ld";
        let regex = Regex::new(regexStr).unwrap();
        let str = "Hello world!";
        let lstMatch = regex.captures(str).unwrap();
        LogUtil::loggerLine(Log::of("Demo", "test11", "lstMatch.len()", lstMatch.len()));
        LogUtil::loggerLine(Log::of("Demo", "test11", "lstMatch.get(0)", lstMatch.get(0).unwrap().as_str()));
        LogUtil::loggerLine(Log::of("Demo", "test11", "lstMatch.get(1)", lstMatch.get(1).unwrap().as_str()));
    }

    fn test12(&self) {
        let regexStr = "\\s(wor)ld";
        let regex = Regex::new(regexStr).unwrap();
        let str = "Hello world!";
        let replaceStr = regex.replace(str, "ggg");
        LogUtil::loggerLine(Log::of("Demo", "test12", "replaceStr", replaceStr));
    }

    fn test13(&self) {
        let regexStr = "\\s(wor)ld";
        let regex = Regex::new(regexStr).unwrap();
        let str = "Hello world!";
        let lstMatch = regex.captures(str).unwrap();
        let matchStr = lstMatch.get(1).unwrap().as_str();
        let replaceStr = str.to_string().replace(matchStr, "ggg");
        LogUtil::loggerLine(Log::of("Demo", "test13", "replaceStr", replaceStr));
    }

    fn test14(&self) {
        let regexStr = "([A-Z])";
        let regex = Regex::new(regexStr).unwrap();
        let str = "HelloWorld";
        let replaceStr = str[0..1].to_string().to_lowercase() + regex.replace_all(&str[1..str.len()], |lstMatch: &Captures| -> String {
            return "-".to_string() + lstMatch.get(1).unwrap().as_str().to_lowercase().as_str();
        }).to_string().as_str();
        LogUtil::loggerLine(Log::of("Demo", "test14", "replaceStr", replaceStr));
    }

    fn test15(&self) {
        let regexStr = "\\-(\\w)";
        let regex = Regex::new(regexStr).unwrap();
        let str = "hello-world";
        let replaceStr = str[0..1].to_string().to_uppercase() + regex.replace_all(&str[1..str.len()], |lstMatch: &Captures| -> String {
            return lstMatch.get(1).unwrap().as_str().replace("-", "").to_uppercase();
        }).to_string().as_str();
        LogUtil::loggerLine(Log::of("Demo", "test15", "replaceStr", replaceStr));
    }

    fn test16(&self) {
        let str = "hello-world";
        let replaceStr = GenUtil::toHump(str.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test16", "replaceStr", replaceStr));
    }

    fn test17(&self) {
        let str = "HelloWorld";
        let replaceStr = GenUtil::toLine(str.to_string());
        LogUtil::loggerLine(Log::of("Demo", "test17", "replaceStr", replaceStr));
    }

    fn test18(&self) {
        let regStr = "redirect:\\s\\{[\\s\\S]*?name:\\s\"(\\w+)\"[\\s\\S]*?\\}";
        let path = "D:\\Document\\MyCodes\\Gitea\\api-web-vue3\\src\\router\\index.ts";
        let value = "login";
        FileUtil::modFile(path.to_string(), regStr.to_string(), false, value.to_string());
    }

    fn test19(&self) {
        let regStr = "\\s+.+LOG_ENABLE = (\\S+);";
        let path = "D:\\Document\\MyCodes\\Gitea\\api-web-vue3\\src\\common\\config\\Global.ts";
        let value = "true";
        FileUtil::modContent(path.to_string(), regStr.to_string(), false, value.to_string());
    }

    fn test20(&self) {
        let curExe = env::current_exe();
        if curExe.is_err() {
            LogUtil::loggerLine(Log::of("Demo", "test20", "env::current_exe()", curExe.unwrap_err()));
            return;
        }

        let curExePath = curExe.unwrap();
        let execPath = curExePath.to_str().expect("");
        LogUtil::loggerLine(Log::of("Demo", "test20", "execPath", execPath));
    }

    fn test21(&self) {
        let appDir = FileUtil::appDir();
        let execPath = FileUtil::execPath();
        LogUtil::loggerLine(Log::of("Demo", "test20", "appDir", appDir));
        LogUtil::loggerLine(Log::of("Demo", "test20", "execPath", execPath));
    }

    fn test22(&self) {
        let yamlName = GenUtil::getYaml();
        let yamlPath = GenUtil::getConfigPath();
        LogUtil::loggerLine(Log::of("Demo", "test22", "yamlName", yamlName));
        LogUtil::loggerLine(Log::of("Demo", "test22", "yamlPath", yamlPath.clone()));

        let content = FileUtil::read(yamlPath);
        let mapValue: HashMap<String, Value> = serde_yaml::from_str(content.as_str()).unwrap();
        // let flag  = matches!(mapValue, Value::Mapping(_));
        // let value = mapValue.get("msg").unwrap().as_str().unwrap().to_string();
        // LogUtil::loggerLine(Log::of("Demo", "test22", "flag", flag));
        let value = mapValue.get("msg").unwrap().as_str().unwrap().to_string();
        LogUtil::loggerLine(Log::of("Demo", "test22", "value", value));
    }

    fn test23(&self) {
        let value = GenUtil::getValue("msg");
        LogUtil::loggerLine(Log::of("Demo", "test23", "value", value));

        let mut mapData = GenUtil::getConfig();
        mapData.insert("msg".to_string(), Value::from("世界，你好！"));
        GenUtil::writeConfig(mapData);
    }

}

pub fn run() {
    let demo = Demo::new();

    demo.test23();
    // demo.test22();
    // demo.test21();
    // demo.test20();
    // demo.test19();
    // demo.test18();
    // demo.test17();
    // demo.test16();
    // demo.test15();
    // demo.test14();
    // demo.test13();
    // demo.test12();
    // demo.test11();
    // demo.test10();
    // demo.test9();
    // demo.test8();
    // demo.test7();
    // demo.test6();
    // demo.test5();
    // demo.test4();
    // demo.test3();
    // demo.test2();
    // demo.test1();
    // demo.test();
}
