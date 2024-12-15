use crate::application::deploy::pojo::dto::BuildConfig::BuildConfig;
use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
use crate::application::deploy::pojo::po::OsInfo::OsInfo;
use crate::application::deploy::pojo::po::Script::Script;
use crate::application::pojo::dto::Log::Log;
use crate::application::util::{DataUtil, FileUtil, GenUtil, LogUtil, PromptUtil, RemoteUtil};

struct BuildScriptService {
    osInfos: Vec<OsInfo>,
    scripts: Vec<Script>,
    buildConfig: BuildConfig,
}

impl BuildScriptService {
    pub fn new() -> Self {
        Self {
            osInfos: OsInfo::gets(),
            scripts: Script::gets(),
            buildConfig: BuildConfig::get(),
        }
    }

    fn apply(&self) {
        println!();
        for (i, script) in self.scripts.iter().enumerate() {
            println!("{}. {}", i + 1, script.rustName());
        }
        print!("Please enter one or more numbers corresponding to the script: ");
        let scriptNums = GenUtil::readParams();
        if scriptNums.len() == 0 {
            return;
        }
        println!();

        for (i, osInfo) in self.osInfos.iter().enumerate() {
            println!("{}. {}", i + 1, osInfo.name());
        }
        print!("Please enter one or more numbers corresponding to the os: ");
        let osNums = GenUtil::readParams();
        let osNum = GenUtil::strToUsize(osNums.get(0).expect("0"));
        let osInfo = self.osInfos.get(osNum - 1).unwrap();
        println!();

        for (i, arch) in osInfo.archs().iter().enumerate() {
            println!("{}. {}", i + 1, arch.name());
        }
        print!("Please enter one or more numbers corresponding to the arch: ");
        let archNums = GenUtil::readParams();
        let archNum = GenUtil::strToUsize(archNums.get(0).expect("0"));
        let archInfo = osInfo.archs().get(archNum - 1).unwrap();
        println!();

        for (i, compilationType) in archInfo.compilationTypes().iter().enumerate() {
            println!("{}. {}", i + 1, compilationType.name());
        }
        print!("Please enter one or more numbers corresponding to the compilation type: ");
        let compilationNums = GenUtil::readParams();
        let compilationNum = GenUtil::strToUsize(compilationNums.get(0).expect("0"));
        let compilationInfo = archInfo.compilationTypes().get(compilationNum - 1).unwrap();
        println!();

        for scriptNum in scriptNums {
            let index = GenUtil::strToUsize(scriptNum.as_str()) - 1;
            if 0 <= index && index < self.scripts.len() {
                let script = self.scripts.get(index).unwrap();
                BuildConfig::setBinTargetPath(&self.buildConfig, compilationInfo);
                Script::setDistPath(script, &self.buildConfig, osInfo.name(), archInfo.name());
                self.build(script, compilationInfo);
            }
        }
    }

    fn build(&self, script: &Script, compilationTypeInfo: &CompilationTypeInfo) {
        self.changeCrossBuild(compilationTypeInfo, true);
        self.changeBuildConfig(script, true);

        let (bin, args) = PromptUtil::packageRustScript(self.buildConfig.crossBuildPath());
        RemoteUtil::changeWorkFolder(FileUtil::appDir().as_str());
        RemoteUtil::execLocalCmd(bin, args);

        self.updateScript(script);
        self.changeBuildConfig(script, false);
        self.changeCrossBuild(compilationTypeInfo, false);
    }

    fn updateScript(&self, script: &Script) {
        if !FileUtil::exist(script.scriptProject()) {
            FileUtil::mkdir(script.scriptProject());
        }
        if FileUtil::exist(script.scriptPath()) &&
            FileUtil::exist(script.targetPath()) &&
            script.rustName() != "BuildScriptService.rs" {
            FileUtil::delete(script.scriptPath());
        }
        if FileUtil::exist(script.scriptPath()) &&
            FileUtil::exist(script.yamlConfig()) {
            FileUtil::delete(script.scriptConfig());
        }
        if FileUtil::exist(script.targetPath()) &&
            script.rustName() != "BuildScriptService.rs" {
            FileUtil::copy(script.targetPath(), script.scriptPath());
        }
        if FileUtil::exist(script.yamlConfig()) {
            FileUtil::copy(script.yamlConfig(), script.scriptConfig());
        }
    }

    fn changeBuildConfig(&self, script: &Script, isBefore: bool) {
        let mut scriptRun = script.scriptRun();
        let mut scriptUse = script.scriptUse();
        if !isBefore {
            scriptRun = self.buildConfig.scriptRunOriginal();
            scriptUse = self.buildConfig.packageUseOriginal();
        }
        FileUtil::modContent(self.buildConfig.appPath(), self.buildConfig.scriptRunPattern(), false, scriptRun);
        FileUtil::modContent(self.buildConfig.appPath(), self.buildConfig.packageUsePattern(), false, scriptUse);
    }

    fn changeCrossBuild(&self, compilationTypeInfo: &CompilationTypeInfo, isBefore: bool) {
        let mut buildTarget = compilationTypeInfo.target();
        if !isBefore {
            buildTarget = self.buildConfig.buildTargetOriginal();
        }
        FileUtil::modFile(self.buildConfig.crossBuildPath(), self.buildConfig.buildTargetPattern(), true, buildTarget);
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
    BuildScriptService::new().apply();
}