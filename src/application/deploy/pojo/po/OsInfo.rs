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
    archs: Vec<ArchInfo>,
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
    fn new(name: String, archs: Vec<ArchInfo>) -> Self {
        Self { name, archs }
    }

    pub fn of(name: &str, archs: Vec<ArchInfo>) -> Self {
        Self::new(name.to_string(), archs)
    }

    pub fn gets() -> Vec<OsInfo> {
        let osInfo = GenUtil::getValue("os_info");
        let mapData = GenUtil::getConfig();
        let osInfoKey = format!("{}_{}", osInfo, "os_info");
        let osInfoData = mapData.get(osInfoKey.as_str());
        if osInfoData.is_none() {
            return Vec::new();
        }
        let content = serde_yaml::to_string(osInfoData.unwrap()).unwrap();
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

    pub fn set_archs(&mut self, archs: Vec<ArchInfo>) {
        self.archs = archs;
    }
}

impl OsInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn archs(&self) -> &Vec<ArchInfo> {
        &self.archs
    }
}

