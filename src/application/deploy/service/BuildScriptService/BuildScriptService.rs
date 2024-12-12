use crate::application::deploy::pojo::po::OsInfo::OsInfo;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, LogUtil};

pub struct BuildScriptService {
    osInfos: Vec<OsInfo>,
}

impl BuildScriptService {
    pub fn new() -> Self {
        Self {
            osInfos: OsInfo::gets()
        }
    }

    fn test(&self) {
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "self.osInfos.len()", self.osInfos.len()));
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "osInfos", DataUtil::objToJson(&self.osInfos)));
    }
}

pub fn run() {
    BuildScriptService::new().test();
}