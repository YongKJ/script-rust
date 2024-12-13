use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
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
    #[serde(rename = "cross_build_content")]
    crossBuildContent: String,
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
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "BuildConfig", DataUtil::objToJson(&self))
    }
}

impl BuildConfig {
    fn new(appPath: String, appTestPath: String, crossBuildPath: String, crossBuildContent: String, targetPath: String, releaseTargetPath: String, debugTargetPath: String, scriptRunPattern: String, scriptRunOriginal: String, packageUsePattern: String, packageUseOriginal: String) -> Self {
        Self { appPath, appTestPath, crossBuildPath, crossBuildContent, targetPath, releaseTargetPath, debugTargetPath, scriptRunPattern, scriptRunOriginal, packageUsePattern, packageUseOriginal }
    }

    pub fn of(appPath: &str, appTestPath: &str, crossBuildPath: &str, crossBuildContent: &str, targetPath: &str, releaseTargetPath: &str, debugTargetPath: &str, scriptRunPattern: &str, scriptRunOriginal: &str, packageUsePattern: &str, packageUseOriginal: &str) -> Self {
        Self::new(appPath.to_string(), appTestPath.to_string(), crossBuildPath.to_string(), crossBuildContent.to_string(), targetPath.to_string(), releaseTargetPath.to_string(), debugTargetPath.to_string(), scriptRunPattern.to_string(), scriptRunOriginal.to_string(), packageUsePattern.to_string(), packageUseOriginal.to_string())
    }

    pub fn get() -> BuildConfig {
        let appTestPath = FileUtil::getAbsPath(vec!["src", "application", "ApplicationTest.rs"]);
        let appPath = FileUtil::getAbsPath(vec!["src", "application", "Application.rs"]);
        let crossBuildPath = FileUtil::getAbsPath(vec!["cross_build.cmd"]);
        let crossBuildContent = FileUtil::read(crossBuildPath.as_str());
        let targetPath = FileUtil::getAbsPath(vec!["target"]);
        Self::of(
            appPath.as_str(), appTestPath.as_str(), crossBuildPath.as_str(), crossBuildContent.as_str(),
            targetPath.as_str(), "", "", "\\s+(\\S+)::run\\(\\)",
            "Demo", "use\\s+(crate\\S+);",
            "crate::application::applet::Demo::Demo",
        )
    }

    pub fn setBinTargetPath(mut buildConfig: BuildConfig, compilationTypeInfo: CompilationTypeInfo) {
        let mut binName = "script_rust".to_string();
        if cfg!(windows) {
            binName = binName + ".exe";
        }
        let debugTargetBin = FileUtil::getAbsPath(vec!["target", compilationTypeInfo.target(), "debug", binName.as_str()]);
        let releaseTargetBin = FileUtil::getAbsPath(vec!["target", compilationTypeInfo.target(), "release", binName.as_str()]);
        buildConfig.set_releaseTargetPath(releaseTargetBin);
        buildConfig.set_debugTargetPath(debugTargetBin);
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

    pub fn set_crossBuildContent(&mut self, crossBuildContent: String) {
        self.crossBuildContent = crossBuildContent;
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

    pub fn crossBuildContent(&self) -> &str {
        &self.crossBuildContent
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
}