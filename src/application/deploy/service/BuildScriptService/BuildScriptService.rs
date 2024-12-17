use crate::application::deploy::pojo::dto::BuildConfig::BuildConfig;
use crate::application::deploy::pojo::po::ArchInfo::ArchInfo;
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
    fn new() -> Self {
        Self {
            osInfos: OsInfo::gets(),
            scripts: Script::gets(),
            buildConfig: BuildConfig::get(),
        }
    }

    fn apply(&mut self) {
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
        print!("Please enter one number corresponding to the os: ");
        let osNums = GenUtil::readParams();
        let osNum = GenUtil::strToUsize(osNums.get(0).expect("0"));
        let osInfo = self.osInfos.get(osNum - 1).unwrap();
        println!();

        for (i, arch) in osInfo.archs().iter().enumerate() {
            println!("{}. {}", i + 1, arch.name());
        }
        print!("Please enter one number corresponding to the arch: ");
        let archNums = GenUtil::readParams();
        let archNum = GenUtil::strToUsize(archNums.get(0).expect("0"));
        let archInfo = osInfo.archs().get(archNum - 1).unwrap();
        println!();

        for (i, compilationType) in archInfo.compilationTypes().iter().enumerate() {
            println!("{}. {}", i + 1, compilationType.name());
        }
        print!("Please enter one number corresponding to the compilation type: ");
        let compilationNums = GenUtil::readParams();
        let compilationNum = GenUtil::strToUsize(compilationNums.get(0).expect("0"));
        let compilationInfo = archInfo.compilationTypes().get(compilationNum - 1).unwrap();
        println!();

        for scriptNum in scriptNums {
            let index = GenUtil::strToUsize(scriptNum.as_str()) - 1;
            if index >= self.scripts.len() {
                continue;
            }

            let (debugTargetBin, releaseTargetBin) = BuildConfig::getBinTargetPath(osInfo, compilationInfo);
            self.buildConfig.set_debugTargetPath(debugTargetBin);
            self.buildConfig.set_releaseTargetPath(releaseTargetBin);

            let (scriptPath, targetPath) = Script::getDistPath(&self.scripts[index], &self.buildConfig, osInfo, archInfo, compilationInfo);
            self.scripts[index].set_scriptPath(scriptPath);
            self.scripts[index].set_targetPath(targetPath);

            self.build(&self.scripts[index], osInfo, archInfo, compilationInfo);
        }
    }

    fn build(&self, script: &Script, osInfo: &OsInfo, archInfo: &ArchInfo, compilationTypeInfo: &CompilationTypeInfo) {
        self.changeCrossBuild(compilationTypeInfo, true);
        self.changeBuildConfig(script, true);

        let (bin, args) = PromptUtil::packageRustScript(self.buildConfig.crossBuildPath());
        RemoteUtil::changeWorkFolder(FileUtil::appDir(false).as_str());
        RemoteUtil::execLocalCmd(bin, args);

        self.changeBuildConfig(script, false);
        self.changeCrossBuild(compilationTypeInfo, false);
        self.updateScript(script, osInfo, archInfo, compilationTypeInfo);
    }

    fn updateScript(&self, script: &Script, osInfo: &OsInfo, archInfo: &ArchInfo, compilationTypeInfo: &CompilationTypeInfo) {
        if !FileUtil::exist(script.scriptProject()) {
            FileUtil::mkdir(script.scriptProject());
        }
        if FileUtil::exist(script.scriptPath()) &&
            FileUtil::exist(script.targetPath()) &&
            !(script.rustName() == "BuildScriptService.rs" && osInfo.name() == "windows" &&
                archInfo.name() == "x86_64" && compilationTypeInfo.name() == "msvc") {
            FileUtil::delete(script.scriptPath());
        }
        if FileUtil::exist(script.scriptPath()) &&
            FileUtil::exist(script.yamlConfig()) {
            FileUtil::delete(script.scriptConfig());
        }
        if FileUtil::exist(script.targetPath()) &&
            !(script.rustName() == "BuildScriptService.rs" && osInfo.name() == "windows" &&
                archInfo.name() == "x86_64" && compilationTypeInfo.name() == "msvc") {
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
        let mut addTarget = compilationTypeInfo.target();
        let mut buildTarget = compilationTypeInfo.target();
        if !isBefore {
            addTarget = self.buildConfig.addTargetOriginal();
            buildTarget = self.buildConfig.buildTargetOriginal();
        }
        FileUtil::modContent(self.buildConfig.crossBuildPath(), self.buildConfig.addTargetPattern(), false, addTarget);
        FileUtil::modContent(self.buildConfig.crossBuildPath(), self.buildConfig.buildTargetPattern(), false, buildTarget);

        if compilationTypeInfo.flags().is_empty() {
            return;
        }

        let mut configTarget = compilationTypeInfo.target();
        let mut configFlags = DataUtil::objToJson(compilationTypeInfo.flags());
        if !isBefore {
            configTarget = self.buildConfig.configTargetOriginal();
            configFlags = self.buildConfig.configFlagsOriginal().to_string();
        }
        FileUtil::modContent(self.buildConfig.cargoConfigPath(), self.buildConfig.configTargetPattern(), false, configTarget);
        FileUtil::modContent(self.buildConfig.cargoConfigPath(), self.buildConfig.configFlagsPattern(), false, configFlags.as_str());
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