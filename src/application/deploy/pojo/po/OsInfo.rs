use crate::application::deploy::pojo::po::ArchInfo::ArchInfo;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, GenUtil, LogUtil};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct OsInfo {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "arch")]
    arch: Vec<ArchInfo>,
}

impl Debug for OsInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}\n", "Debug: ", "OsInfo", DataUtil::objToJson(&self))
    }
}

impl Display for OsInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "OsInfo", DataUtil::objToJson(&self))
    }
}

impl OsInfo {
    fn new(name: String, arch: Vec<ArchInfo>) -> Self {
        Self { name, arch }
    }

    pub fn of(name: &str, arch: Vec<ArchInfo>) -> OsInfo {
        OsInfo::new(name.to_string(), arch)
    }

    pub fn gets() -> Vec<OsInfo> {
        let mapData = GenUtil::getConfig();
        let osInfoData = mapData.get("os_info").unwrap();
        let content = serde_yaml::to_string(osInfoData).unwrap();
        let lstData = serde_yaml::from_str(content.as_str());
        if lstData.is_err() {
            LogUtil::loggerLine(Log::of("OsInfo", "gets", "err", lstData.unwrap_err()));
            return Vec::new();
        }
        lstData.unwrap()
    }
}

impl OsInfo {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_arch(&mut self, arch: Vec<ArchInfo>) {
        self.arch = arch;
    }
}

impl OsInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arch(&self) -> &Vec<ArchInfo> {
        &self.arch
    }
}

