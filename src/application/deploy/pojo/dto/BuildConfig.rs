use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
use crate::application::deploy::pojo::po::Script::Script;
use crate::application::util::{DataUtil, FileUtil};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(rename = "app_path")]
    appPath: String,
    #[serde(rename = "app_test_path")]
    appTestPath: String,
    #[serde(rename = "cross_build_path")]
    crossBuildPath: String,
    #[serde(rename = "target_path")]
    targetPath: String,
    #[serde(rename = "release_target_path")]
    releaseTargetPath: String,
    #[serde(rename = "debug_target_path")]
    debugTargetPath: String,
    #[serde(rename = "script_run_pattern")]
    scriptRunPattern: String,
    #[serde(rename = "script_run_original")]
    scriptRunOriginal: String,
    #[serde(rename = "package_use_pattern")]
    packageUsePattern: String,
    #[serde(rename = "package_use_original")]
    packageUseOriginal: String,
    #[serde(rename = "build_target_pattern")]
    buildTargetPattern: String,
    #[serde(rename = "build_target_original")]
    buildTargetOriginal: String,
    #[serde(rename = "add_target_pattern")]
    addTargetPattern: String,
    #[serde(rename = "add_target_original")]
    addTargetOriginal: String,
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "BuildConfig", DataUtil::objToJson(&self))
    }
}

impl BuildConfig {
    fn new(appPath: String, appTestPath: String, crossBuildPath: String, targetPath: String, releaseTargetPath: String, debugTargetPath: String, scriptRunPattern: String, scriptRunOriginal: String, packageUsePattern: String, packageUseOriginal: String, buildTargetPattern: String, buildTargetOriginal: String, addTargetPattern: String, addTargetOriginal: String) -> Self {
        Self { appPath, appTestPath, crossBuildPath, targetPath, releaseTargetPath, debugTargetPath, scriptRunPattern, scriptRunOriginal, packageUsePattern, packageUseOriginal, buildTargetPattern, buildTargetOriginal, addTargetPattern, addTargetOriginal }
    }

    pub fn of(appPath: &str, appTestPath: &str, crossBuildPath: &str, targetPath: &str, releaseTargetPath: &str, debugTargetPath: &str, scriptRunPattern: &str, scriptRunOriginal: &str, packageUsePattern: &str, packageUseOriginal: &str, buildTargetPattern: &str, buildTargetOriginal: &str, addTargetPattern: &str, addTargetOriginal: &str) -> Self {
        Self::new(appPath.to_string(), appTestPath.to_string(), crossBuildPath.to_string().to_string(), targetPath.to_string(), releaseTargetPath.to_string(), debugTargetPath.to_string(), scriptRunPattern.to_string(), scriptRunOriginal.to_string(), packageUsePattern.to_string(), packageUseOriginal.to_string(), buildTargetPattern.to_string(), buildTargetOriginal.to_string(), addTargetPattern.to_string(), addTargetOriginal.to_string())
    }

    pub fn get() -> BuildConfig {
        let appTestPath = FileUtil::getAbsPath(false, vec!["src", "application", "ApplicationTest.rs"]);
        let appPath = FileUtil::getAbsPath(false, vec!["src", "application", "Application.rs"]);
        let mut crossBuildPath = FileUtil::getAbsPath(false, vec!["cross_build.sh"]);
        let targetPath = FileUtil::getAbsPath(false, vec!["target"]);
        if cfg!(windows) {
            crossBuildPath = FileUtil::getAbsPath(false, vec!["cross_build.cmd"])
        }
        Self::of(
            appPath.as_str(), appTestPath.as_str(), crossBuildPath.as_str(), targetPath.as_str(),
            "", "", "\\s+(\\S+)::run\\(\\)",
            "Demo", "use\\s+(crate\\S+);",
            "crate::application::applet::Demo::Demo",
            "[\\s\\S]+=(\\S+)\\s--release", "x86_64-pc-windows-msvc",
            "[\\s\\S]+add\\s(\\S+)", "x86_64-pc-windows-msvc"
        )
    }

    pub fn getBinTargetPath(compilationTypeInfo: &CompilationTypeInfo) -> (String, String) {
        let mut binName = "script_rust".to_string();
        if cfg!(windows) {
            binName = binName + ".exe";
        }
        let debugTargetBin = FileUtil::getAbsPath(false, vec!["target", compilationTypeInfo.target(), "debug", binName.as_str()]);
        let releaseTargetBin = FileUtil::getAbsPath(false, vec!["target", compilationTypeInfo.target(), "release", binName.as_str()]);
        (debugTargetBin, releaseTargetBin)
    }

}

impl BuildConfig {
    pub fn set_appPath(&mut self, appPath: String) {
        self.appPath = appPath;
    }

    pub fn set_appTestPath(&mut self, appTestPath: String) {
        self.appTestPath = appTestPath;
    }

    pub fn set_crossBuildPath(&mut self, crossBuildPath: String) {
        self.crossBuildPath = crossBuildPath;
    }

    pub fn set_targetPath(&mut self, targetPath: String) {
        self.targetPath = targetPath;
    }

    pub fn set_releaseTargetPath(&mut self, releaseTargetPath: String) {
        self.releaseTargetPath = releaseTargetPath;
    }

    pub fn set_debugTargetPath(&mut self, debugTargetPath: String) {
        self.debugTargetPath = debugTargetPath;
    }

    pub fn set_scriptRunPattern(&mut self, scriptRunPattern: String) {
        self.scriptRunPattern = scriptRunPattern;
    }

    pub fn set_scriptRunOriginal(&mut self, scriptRunOriginal: String) {
        self.scriptRunOriginal = scriptRunOriginal;
    }

    pub fn set_packageUsePattern(&mut self, packageUsePattern: String) {
        self.packageUsePattern = packageUsePattern;
    }

    pub fn set_packageUseOriginal(&mut self, packageUseOriginal: String) {
        self.packageUseOriginal = packageUseOriginal;
    }

    pub fn set_buildTargetPattern(&mut self, buildTargetPattern: String) {
        self.buildTargetPattern = buildTargetPattern;
    }

    pub fn set_buildTargetOriginal(&mut self, buildTargetOriginal: String) {
        self.buildTargetOriginal = buildTargetOriginal;
    }

    pub fn set_addTargetPattern(&mut self, addTargetPattern: String) {
        self.addTargetPattern = addTargetPattern;
    }

    pub fn set_addTargetOriginal(&mut self, addTargetOriginal: String) {
        self.addTargetOriginal = addTargetOriginal;
    }
}

impl BuildConfig {
    pub fn appPath(&self) -> &str {
        &self.appPath
    }

    pub fn appTestPath(&self) -> &str {
        &self.appTestPath
    }

    pub fn crossBuildPath(&self) -> &str {
        &self.crossBuildPath
    }

    pub fn targetPath(&self) -> &str {
        &self.targetPath
    }

    pub fn releaseTargetPath(&self) -> &str {
        &self.releaseTargetPath
    }

    pub fn debugTargetPath(&self) -> &str {
        &self.debugTargetPath
    }

    pub fn scriptRunPattern(&self) -> &str {
        &self.scriptRunPattern
    }

    pub fn scriptRunOriginal(&self) -> &str {
        &self.scriptRunOriginal
    }

    pub fn packageUsePattern(&self) -> &str {
        &self.packageUsePattern
    }

    pub fn packageUseOriginal(&self) -> &str {
        &self.packageUseOriginal
    }

    pub fn buildTargetPattern(&self) -> &str {
        &self.buildTargetPattern
    }

    pub fn buildTargetOriginal(&self) -> &str {
        &self.buildTargetOriginal
    }

    pub fn addTargetPattern(&self) -> &str {
        &self.addTargetPattern
    }

    pub fn addTargetOriginal(&self) -> &str {
        &self.addTargetOriginal
    }

}