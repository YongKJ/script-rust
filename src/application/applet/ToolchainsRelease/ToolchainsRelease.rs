use crate::application::applet::ToolchainsRelease::pojo::po::ToolchainInfo::ToolchainInfo;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, FileUtil, LogUtil, PromptUtil, RemoteUtil};
use std::collections::HashMap;
use std::path;

struct ToolchainsRelease {
    toolchainInfos: Vec<ToolchainInfo>,
}

impl ToolchainsRelease {
    fn new() -> Self {
        Self {
            toolchainInfos: ToolchainInfo::gets(),
        }
    }

    fn apply(&self) {
        let mut mapData: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
        for toolchainInfo in &self.toolchainInfos {
            let index = toolchainInfo.srcDir().rfind(path::MAIN_SEPARATOR).unwrap();
            let toolchainType = toolchainInfo.srcDir().get(index + 1..toolchainInfo.srcDir().len()).unwrap();
            if !FileUtil::exist(toolchainInfo.desDir()) {
                FileUtil::mkdir(toolchainInfo.desDir());
            }

            let mut lstData: Vec<HashMap<String, String>> = Vec::new();
            let toolchains = FileUtil::list(toolchainInfo.srcDir());
            for toolchain in toolchains {
                if toolchain.ends_with("sha256") {
                    continue;
                }
                let toolchainPath = format!("{}{}{}", toolchainInfo.srcDir(), path::MAIN_SEPARATOR, toolchain);
                let releaseDir = self.release(toolchainPath.as_str(), toolchainInfo.desDir());
                lstData.push(self.getGccPath(releaseDir.as_str()));
            }

            mapData.insert(toolchainType.to_string(), lstData);
        }

        let mut folder = FileUtil::appDir(true);
        if cfg!(windows) {
            folder = FileUtil::appDir(false);
        }
        RemoteUtil::changeWorkFolder(folder.as_str());
        let content = DataUtil::objToJson(&mapData);
        FileUtil::write("toolchains-bin.json", content.as_str());
    }

    fn release(&self, toolchainPath: &str, desDir: &str) -> String {
        let (bin, args) = PromptUtil::releaseToolchain(toolchainPath, desDir);
        RemoteUtil::changeWorkFolder(desDir);
        RemoteUtil::execLocalCmd(bin, args);

        let pointIndex = toolchainPath.find(".tar").unwrap();
        let sepIndex = toolchainPath.rfind(path::MAIN_SEPARATOR).unwrap();
        format!("{}{}{}", desDir, path::MAIN_SEPARATOR, toolchainPath.get(sepIndex + 1..pointIndex).unwrap())
    }

    fn getGccPath(&self, releaseDir: &str) -> HashMap<String, String> {
        let binPath = format!("{}{}{}", releaseDir, path::MAIN_SEPARATOR, "bin");
        let binNames = FileUtil::list(binPath.as_str());
        let mut mapData: HashMap<String, String> = HashMap::new();
        for binName in binNames {
            if binName.ends_with("gcc") {
                let binPath = format!("{}{}{}", binPath, path::MAIN_SEPARATOR, binName);
                mapData.insert(binName, binPath);
                break;
            }
        }
        mapData
    }
}

pub fn run() {
    ToolchainsRelease::new().apply();
}