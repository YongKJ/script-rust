use crate::application::applet::ToolchainsRelease::pojo::po::ToolchainInfo::ToolchainInfo;
use crate::application::util::{DataUtil, FileUtil, PromptUtil, RemoteUtil};
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
        let mut mapData: HashMap<String, Vec<String>> = HashMap::new();
        for toolchainInfo in self.toolchainInfos {
            let index = toolchainInfo.srcDir().rfind("/").unwrap();
            let toolchainType = toolchainInfo.srcDir().get(index + 1..toolchainInfo.srcDir().len()).unwrap();
            if !FileUtil::exist(toolchainInfo.desDir()) {
                FileUtil::mkdir(toolchainInfo.desDir());
            }

            let mut lstData: Vec<String> = Vec::new();
            let toolchains = FileUtil::list(toolchainInfo.srcDir());
            for toolchain in toolchains {
                if toolchain.ends_with("sha256") {
                    continue;
                }
                let toolchainPath = format!("{}{}{}", toolchainInfo.srcDir(), path::MAIN_SEPARATOR, toolchain);
                let releaseDir = Self.release(toolchainPath.as_str(), toolchainInfo.desDir());
                lstData.push(self.getGccPath(releaseDir.as_str()));
            }

            mapData.insert(toolchainType.to_string(), lstData);
        }

        let content = DataUtil::objToJson(&mapData);
        RemoteUtil::changeWorkFolder(FileUtil::appDir(true).as_str());
        FileUtil::write("toolchains-bin.json", content.as_str());
    }

    fn release(&self, toolchainPath: &str, desDir: &str) -> String {
        let (bin, args) = PromptUtil::releaseToolchain(toolchainPath, desDir);
        RemoteUtil::changeWorkFolder(desDir);
        RemoteUtil::execLocalCmd(bin, args);

        let sepIndex = toolchainPath.rfind("/").unwrap();
        let pointIndex = toolchainPath.rfind(".").unwrap();
        desDir.to_string() + toolchainPath.get(sepIndex + 1..pointIndex).unwrap()
    }

    fn getGccPath(&self, releaseDir: &str) -> String {
        let binPath = format!("{}{}{}", releaseDir, path::MAIN_SEPARATOR, "bin");
        let binNames = FileUtil::list(binPath.as_str());
        for binName in binNames {
            if binName.ends_with("gcc") {
                return binName;
            }
        }
        "".to_string()
    }
}

pub fn run() {
    ToolchainsRelease::new().apply();
}