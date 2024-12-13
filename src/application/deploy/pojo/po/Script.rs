use crate::application::util::{DataUtil, FileUtil, GenUtil};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path;

#[derive(Serialize, Deserialize)]
pub struct Script {
    #[serde(rename = "rust_name")]
    rustName: String,
    #[serde(rename = "rust_path")]
    rustPath: String,
    #[serde(rename = "yaml_config")]
    yamlConfig: String,
    #[serde(rename = "script_name")]
    scriptName: String,
    #[serde(rename = "script_path")]
    scriptPath: String,
    #[serde(rename = "script_config")]
    scriptConfig: String,
    #[serde(rename = "script_run")]
    scriptRun: String,
    #[serde(rename = "script_use")]
    scriptUse: String,
    #[serde(rename = "script_project")]
    scriptProject: String,
    #[serde(rename = "dist_path")]
    distPath: String,
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "Script", DataUtil::objToJson(&self))
    }
}

impl Script {
    fn new(rustName: String, rustPath: String, yamlConfig: String, scriptName: String, scriptPath: String, scriptConfig: String, scriptRun: String, scriptUse: String, scriptProject: String, distPath: String) -> Self {
        Self { rustName, rustPath, yamlConfig, scriptName, scriptPath, scriptConfig, scriptRun, scriptUse, scriptProject, distPath }
    }

    pub fn of(rustName: &str, rustPath: &str, yamlConfig: &str, scriptName: &str, scriptPath: &str, scriptConfig: &str, scriptRun: &str, scriptUse: &str, scriptProject: &str, distPath: &str) -> Self {
        Self::new(rustName.to_string(), rustPath.to_string(), yamlConfig.to_string(), scriptName.to_string(), scriptPath.to_string(), scriptConfig.to_string(), scriptRun.to_string(), scriptUse.to_string(), scriptProject.to_string(), distPath.to_string())
    }

    pub fn gets() -> Vec<Script> {
        let path = FileUtil::getAbsPath(vec!["src", "application", "deploy", "service"]);
        let mut lstScript = Self::getListByDir("".to_string());
        lstScript.extend(Self::getListByDir(path));
        lstScript
    }

    pub fn setDistPath(mut script: Script, os: &str, arch: &str) {
        let mut distPath = script.distPath.clone();
        let mut scriptPath = script.scriptPath.clone();
        if !(os == "windows" && arch == "x86_64") {
            distPath = format!("{}-{}-{}", distPath, os, arch);
            scriptPath = format!("{}-{}-{}", scriptPath, os, arch);
        }
        if os == "windows" {
            distPath = distPath + ".exe";
            scriptPath = scriptPath + ".exe";
        }
        script.set_distPath(distPath);
        script.set_scriptPath(scriptPath);
    }

    fn getListByDir(mut appletDir: String) -> Vec<Script> {
        if appletDir.len() == 0 {
            appletDir = FileUtil::getAbsPath(vec!["src", "application", "applet"]);
        }
        let assetsDir = FileUtil::getAbsPath(vec!["src", "assets"]);
        let scriptDir = FileUtil::getAbsPath(vec!["script"]);
        let distDir = FileUtil::getAbsPath(vec!["dist"]);
        let lstFile = FileUtil::list(appletDir.as_str());

        let mut lstScript: Vec<Script> = Vec::new();
        for file in lstFile {
            if file == "mod.rs" {
                continue;
            }
            let mut rustPath = FileUtil::join(appletDir.as_str(), file.as_str());
            if FileUtil::isFolder(rustPath.as_str()) {
                rustPath = Self::getScript(rustPath.as_str());
            }
            let index = rustPath.rfind(path::MAIN_SEPARATOR).unwrap();
            let rustName = rustPath.get(index + 1..rustPath.len()).unwrap();
            let name = rustName.trim_end_matches(".rs");

            let scriptRun = name;
            let scriptName = GenUtil::toLine(scriptRun);
            let projectName = GenUtil::toLine(scriptRun);
            let scriptUse = Self::getUsePath(rustPath.clone());
            let yamlName = GenUtil::toLine(name) + ".yaml";
            let mut distPath = FileUtil::join(distDir.as_str(), scriptName.as_str());
            let yamlConfig = FileUtil::join(assetsDir.as_str(), yamlName.as_str());
            let scriptProject = FileUtil::join(scriptDir.as_str(), projectName.as_str());
            let scriptConfig = FileUtil::join(scriptProject.as_str(), yamlName.as_str());
            let scriptPath = FileUtil::join(scriptProject.as_str(), scriptName.as_str());
            distPath = distPath.replace(path::MAIN_SEPARATOR, "/");

            lstScript.push(Self::of(
                rustName, rustPath.as_str(), yamlConfig.as_str(), scriptName.as_str(), scriptPath.as_str(),
                scriptConfig.as_str(), scriptRun, scriptUse.as_str(), scriptProject.as_str(), distPath.as_str(),
            ))
        }
        lstScript
    }

    fn getUsePath(mut path: String) -> String {
        let index = path.find("src").unwrap();
        path = path.trim_end_matches(".rs").to_string();
        path = path.get(index..path.len()).unwrap().to_string();
        path = path.replace("src", "crate").to_string();
        let names: Vec<&str> = path.split(path::MAIN_SEPARATOR).collect();
        names.join("::")
    }

    fn getScript(folder: &str) -> String {
        let lstFile = FileUtil::list(folder);
        for file in lstFile {
            if file == "mod.rs" {
                continue;
            }
            if file.ends_with(".rs") {
                return FileUtil::join(folder, file.as_str());
            }
        }
        "".to_string()
    }
}

impl Script {
    pub fn set_rustName(&mut self, rustName: String) {
        self.rustName = rustName;
    }

    pub fn set_rustPath(&mut self, rustPath: String) {
        self.rustPath = rustPath;
    }

    pub fn set_yamlConfig(&mut self, yamlConfig: String) {
        self.yamlConfig = yamlConfig;
    }

    pub fn set_scriptName(&mut self, scriptName: String) {
        self.scriptName = scriptName;
    }

    pub fn set_scriptPath(&mut self, scriptPath: String) {
        self.scriptPath = scriptPath;
    }

    pub fn set_scriptConfig(&mut self, scriptConfig: String) {
        self.scriptConfig = scriptConfig;
    }

    pub fn set_scriptRun(&mut self, scriptRun: String) {
        self.scriptRun = scriptRun;
    }

    pub fn set_scriptUse(&mut self, scriptUse: String) {
        self.scriptUse = scriptUse;
    }

    pub fn set_scriptProject(&mut self, scriptProject: String) {
        self.scriptProject = scriptProject;
    }

    pub fn set_distPath(&mut self, distPath: String) {
        self.distPath = distPath;
    }
}

impl Script {
    pub fn rustName(&self) -> &str {
        &self.rustName
    }

    pub fn rustPath(&self) -> &str {
        &self.rustPath
    }

    pub fn yamlConfig(&self) -> &str {
        &self.yamlConfig
    }

    pub fn scriptName(&self) -> &str {
        &self.scriptName
    }

    pub fn scriptPath(&self) -> &str {
        &self.scriptPath
    }

    pub fn scriptConfig(&self) -> &str {
        &self.scriptConfig
    }

    pub fn scriptRun(&self) -> &str {
        &self.scriptRun
    }

    pub fn scriptUse(&self) -> &str {
        &self.scriptUse
    }

    pub fn scriptProject(&self) -> &str {
        &self.scriptProject
    }

    pub fn distPath(&self) -> &str {
        &self.distPath
    }
}