use crate::application::deploy::pojo::po::Script::Script;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{FileUtil, GenUtil, LogUtil, RemoteUtil};
use regex::{Captures, Regex};
use serde_yaml::Value;
use std::collections::HashMap;
use std::{env, io};

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
        FileUtil::create(fileName);
    }

    fn test3(&self) {
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", "Hello world!"));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", "世界，你好！".to_string()));
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", 12));
    }

    fn test4(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let size = FileUtil::size(fileName);
        LogUtil::loggerLine(Log::of("Demo", "test3", "msg", size));
    }

    fn test5(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\顾客浏览路径聚合-1729481289990.xlsx";
        let isFolder = FileUtil::isFolder(fileName);
        let isFile = FileUtil::isFile(fileName);
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFolder", isFolder));
        LogUtil::loggerLine(Log::of("Demo", "test3", "isFile", isFile));
    }

    fn test6(&self) {
        // let fileName = "C:\\Users\\admin\\Desktop\\apk";
        let fileName = "C:\\Users\\admin\\Desktop\\api-go";
        // let fileName = "C:\\Users\\Admin\\Desktop\\Database-Backup";
        let folderSize = FileUtil::sizeFolder(fileName);
        LogUtil::loggerLine(Log::of("Demo", "test6", "folderSize", folderSize));
    }

    fn test7(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\rust-test\\rust-demo";
        FileUtil::mkdir(fileName);
    }

    fn test8(&self) {
        // let fileName = "C:\\Users\\Admin\\Desktop\\busybox-df.json";
        let fileName = "C:\\Users\\Admin\\Desktop\\api-ts.code-workspace";
        let lines = FileUtil::readByLine(fileName);
        LogUtil::loggerLine(Log::of("Demo", "test8", "lines.len()", lines.len()));
        // let content = FileUtil::read(fileName.to_string());
        // LogUtil::loggerLine(Log::of("Demo", "test8", "content.len()", content.len()));
    }

    fn test9(&self) {
        let fileName = "C:\\Users\\Admin\\Desktop\\test.code-workspace";
        FileUtil::write(fileName, "Hello world!");
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
        let replaceStr = GenUtil::toHump(str);
        LogUtil::loggerLine(Log::of("Demo", "test16", "replaceStr", replaceStr));
    }

    fn test17(&self) {
        let str = "HelloWorld";
        let replaceStr = GenUtil::toLine(str);
        LogUtil::loggerLine(Log::of("Demo", "test17", "replaceStr", replaceStr));
    }

    fn test18(&self) {
        let regStr = "redirect:\\s\\{[\\s\\S]*?name:\\s\"(\\w+)\"[\\s\\S]*?\\}";
        let path = "D:\\Document\\MyCodes\\Gitea\\api-web-vue3\\src\\router\\index.ts";
        let value = "login";
        FileUtil::modFile(path, regStr, false, value);
    }

    fn test19(&self) {
        let regStr = "\\s+.+LOG_ENABLE = (\\S+);";
        let path = "D:\\Document\\MyCodes\\Gitea\\api-web-vue3\\src\\common\\config\\Global.ts";
        let value = "true";
        FileUtil::modContent(path, regStr, false, value);
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
        let execPath = FileUtil::execPath();
        let appDir = FileUtil::appDir(false);
        LogUtil::loggerLine(Log::of("Demo", "test20", "appDir", appDir));
        LogUtil::loggerLine(Log::of("Demo", "test20", "execPath", execPath));
    }

    fn test22(&self) {
        let yamlName = GenUtil::getYaml();
        let yamlPath = GenUtil::getConfigPath();
        LogUtil::loggerLine(Log::of("Demo", "test22", "yamlName", yamlName));
        LogUtil::loggerLine(Log::of("Demo", "test22", "yamlPath", yamlPath.clone()));

        let content = FileUtil::read(yamlPath.as_str());
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

    fn test24(&self) {
        RemoteUtil::execLocalCmd("D:/Software/scoop/shims/neofetch.cmd", vec![]);
    }

    fn test25(&self) {
        let lstScript = Script::gets();
        for script in lstScript {
            LogUtil::loggerLine(Log::of("Demo", "test25", "script", script));
        }
    }

    fn test26(&self) {
        LogUtil::loggerLine(Log::of("Demo", "test26", "&self.msg", &self.msg));
        LogUtil::loggerLine(Log::of("Demo", "test26", "FileUtil::appDir()", FileUtil::appDir(false)));
    }

    fn test27(&self) {
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        if result.is_err() {
            LogUtil::loggerLine(Log::of("Demo", "test27", "io::stdin().read_line", result.unwrap_err()));
        }
        LogUtil::loggerLine(Log::of("Demo", "test27", "input", input));
    }

    fn test28(&self) {
        let path = "D:\\Document\\MyCodes\\Github\\script_rust\\src\\application\\Application.rs";
        let value = "crate::application::applet::BuildScriptService::BuildScriptService";
        let regStr = "use\\s+(crate\\S+);";

        FileUtil::modContent(path, regStr, false, value);
    }

    fn test29(&self) {
        let path = "D:\\Document\\MyCodes\\Github\\script_rust\\.cargo\\config.toml";
        // let value = "i686-pc-windows-gnu";
        // let regStr = "[\\s\\S]+\\.(\\S+)\\]";
        let value = "[\"-C\", \"linker=i686-w64-mingw32-gcc.exe\"]";
        let regStr = "[\\s\\S]+=\\s([\\s\\S]+)";

        FileUtil::modContent(path, regStr, false, value);
    }
}

pub fn run() {
    let demo = Demo::new();

    demo.test29();
    // demo.test28();
    // demo.test27();
    // demo.test26();
    // demo.test25();
    // demo.test24();
    // demo.test23();
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
