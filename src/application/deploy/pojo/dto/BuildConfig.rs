use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
use crate::application::deploy::pojo::po::OsInfo::OsInfo;
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
    #[serde(rename = "cargo_config_path")]
    cargoConfigPath: String,
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
    #[serde(rename = "config_target_pattern")]
    configTargetPattern: String,
    #[serde(rename = "config_target_original")]
    configTargetOriginal: String,
    #[serde(rename = "config_flags_pattern")]
    configFlagsPattern: String,
    #[serde(rename = "config_flags_original")]
    configFlagsOriginal: String,
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "BuildConfig", DataUtil::objToJson(&self))
    }
}

impl BuildConfig {
    fn new(appPath: String, appTestPath: String, crossBuildPath: String, cargoConfigPath: String, targetPath: String, releaseTargetPath: String, debugTargetPath: String, scriptRunPattern: String, scriptRunOriginal: String, packageUsePattern: String, packageUseOriginal: String, buildTargetPattern: String, buildTargetOriginal: String, addTargetPattern: String, addTargetOriginal: String, configTargetPattern: String, configTargetOriginal: String, configFlagsPattern: String, configFlagsOriginal: String) -> Self {
        Self { appPath, appTestPath, crossBuildPath, cargoConfigPath, targetPath, releaseTargetPath, debugTargetPath, scriptRunPattern, scriptRunOriginal, packageUsePattern, packageUseOriginal, buildTargetPattern, buildTargetOriginal, addTargetPattern, addTargetOriginal, configTargetPattern, configTargetOriginal, configFlagsPattern, configFlagsOriginal }
    }

    pub fn of(appPath: &str, appTestPath: &str, crossBuildPath: &str, cargoConfigPath: &str, targetPath: &str, releaseTargetPath: &str, debugTargetPath: &str, scriptRunPattern: &str, scriptRunOriginal: &str, packageUsePattern: &str, packageUseOriginal: &str, buildTargetPattern: &str, buildTargetOriginal: &str, addTargetPattern: &str, addTargetOriginal: &str, configTargetPattern: &str, configTargetOriginal: &str, configFlagsPattern: &str, configFlagsOriginal: &str) -> Self {
        Self::new(appPath.to_string(), appTestPath.to_string(), crossBuildPath.to_string().to_string(), cargoConfigPath.to_string(), targetPath.to_string(), releaseTargetPath.to_string(), debugTargetPath.to_string(), scriptRunPattern.to_string(), scriptRunOriginal.to_string(), packageUsePattern.to_string(), packageUseOriginal.to_string(), buildTargetPattern.to_string(), buildTargetOriginal.to_string(), addTargetPattern.to_string(), addTargetOriginal.to_string(), configTargetPattern.to_string(), configTargetOriginal.to_string(), configFlagsPattern.to_string(), configFlagsOriginal.to_string())
    }

    pub fn get() -> BuildConfig {
        let appTestPath = FileUtil::getAbsPath(false, vec!["src", "application", "ApplicationTest.rs"]);
        let appPath = FileUtil::getAbsPath(false, vec!["src", "application", "Application.rs"]);
        let cargoConfigPath = FileUtil::getAbsPath(false, vec![".cargo", "config.toml"]);
        let mut crossBuildPath = FileUtil::getAbsPath(false, vec!["cross_build.sh"]);
        let targetPath = FileUtil::getAbsPath(false, vec!["target"]);
        if cfg!(windows) {
            crossBuildPath = FileUtil::getAbsPath(false, vec!["cross_build.cmd"])
        }
        Self::of(
            appPath.as_str(), appTestPath.as_str(), crossBuildPath.as_str(), cargoConfigPath.as_str(),
            targetPath.as_str(), "", "", "\\s+(\\S+)::run\\(\\)",
            "Demo", "use\\s+(crate\\S+);", "crate::application::applet::Demo::Demo",
            "[\\s\\S]+=(\\S+)\\s--release", "x86_64-pc-windows-msvc",
            "[\\s\\S]+add\\s(\\S+)", "x86_64-pc-windows-msvc",
            "[\\s\\S]+\\.(\\S+)\\]", "x86_64-pc-windows-gnu",
            "[\\s\\S]+=\\s(\\S+)", "[\"-C\", \"linker=x86_64-w64-mingw32-gcc.exe\"]"
        )
    }

    pub fn getBinTargetPath(osInfo: &OsInfo, compilationTypeInfo: &CompilationTypeInfo) -> (String, String) {
        let mut binName = "script_rust".to_string();
        if osInfo.name() == "windows" {
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

    pub fn set_cargoConfigPath(&mut self, cargoConfigPath: String) {
        self.cargoConfigPath = cargoConfigPath;
    }

    pub fn set_configTargetPattern(&mut self, configTargetPattern: String) {
        self.configTargetPattern = configTargetPattern;
    }

    pub fn set_configTargetOriginal(&mut self, configTargetOriginal: String) {
        self.configTargetOriginal = configTargetOriginal;
    }

    pub fn set_configFlagsPattern(&mut self, configFlagsPattern: String) {
        self.configFlagsPattern = configFlagsPattern;
    }

    pub fn set_configFlagsOriginal(&mut self, configFlagsOriginal: String) {
        self.configFlagsOriginal = configFlagsOriginal;
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

    pub fn cargoConfigPath(&self) -> &str {
        &self.cargoConfigPath
    }

    pub fn configTargetPattern(&self) -> &str {
        &self.configTargetPattern
    }

    pub fn configTargetOriginal(&self) -> &str {
        &self.configTargetOriginal
    }

    pub fn configFlagsPattern(&self) -> &str {
        &self.configFlagsPattern
    }

    pub fn configFlagsOriginal(&self) -> &str {
        &self.configFlagsOriginal
    }
}