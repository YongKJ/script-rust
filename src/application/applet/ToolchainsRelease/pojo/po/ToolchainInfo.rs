use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, GenUtil, LogUtil};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolchainInfo {
    #[serde(rename = "src_dir")]
    srcDir: String,
    #[serde(rename = "des_dir")]
    desDir: String,
}

impl Display for ToolchainInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "ToolchainInfo", DataUtil::objToJson(&self))
    }
}

impl ToolchainInfo {
    fn new(srcDir: String, desDir: String) -> Self {
        Self { srcDir, desDir }
    }

    pub fn of(srcDir: &str, desDir: &str) -> Self {
        Self::new(srcDir.to_string(), desDir.to_string())
    }

    pub fn gets() -> Vec<ToolchainInfo> {
        let mapData = GenUtil::getConfig();
        let toolchainInfoData = mapData.get("toolchain_info");
        if toolchainInfoData.is_none() {
            return Vec::new();
        }
        let content = serde_yaml::to_string(toolchainInfoData.unwrap()).unwrap();
        let lstData = serde_yaml::from_str(content.as_str());
        if lstData.is_err() {
            LogUtil::loggerLine(Log::of("OsInfo", "gets", "err", lstData.unwrap_err()));
            return Vec::new();
        }
        lstData.unwrap()
    }
}

impl ToolchainInfo {
    pub fn set_srcDir(&mut self, srcDir: String) {
        self.srcDir = srcDir;
    }

    pub fn set_desDir(&mut self, desDir: String) {
        self.desDir = desDir;
    }
}

impl ToolchainInfo {
    pub fn srcDir(&self) -> &str {
        &self.srcDir
    }

    pub fn desDir(&self) -> &str {
        &self.desDir
    }
}