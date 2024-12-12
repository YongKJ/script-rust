use crate::application::deploy::pojo::po::OsInfo::OsInfo;
use crate::application::deploy::pojo::po::Script::Script;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, LogUtil};

pub struct BuildScriptService {
    osInfos: Vec<OsInfo>,
    scripts: Vec<Script>
}

impl BuildScriptService {
    pub fn new() -> Self {
        Self {
            osInfos: OsInfo::gets(),
            scripts: Script::gets()
        }
    }

    fn test(&self) {
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "self.osInfos.len()", self.osInfos.len()));
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "osInfos", DataUtil::objToJson(&self.osInfos)));
        println!();
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "self.scripts.len()", self.scripts.len()));
        LogUtil::loggerLine(Log::of("BuildScriptService", "test", "scripts", DataUtil::objToJson(&self.scripts)));
    }
}

pub fn run() {
    BuildScriptService::new().test();
}