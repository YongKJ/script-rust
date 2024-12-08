use crate::application::pojo::dto::Log::Log;
use crate::application::util::LogUtil;
use std::env::set_current_dir;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn changeWorkFolder(home: &str) {
    let result = set_current_dir(Path::new(home));
    if result.is_err() {
        LogUtil::loggerLine(Log::of("RemoteUtil", "changeWorkFolder", "err", result.unwrap_err()))
    }
}

pub fn execLocalCmd(bin: &str, args: Vec<&str>) {
    let mut cmd = Command::new(bin);
    cmd.stdout(Stdio::piped());
    for arg in args {
        cmd.arg(arg);
    }

    let terminal = cmd.spawn();
    if terminal.is_err() {
        LogUtil::loggerLine(Log::of("RemoteUtil", "execLocalCmd", "terminal", terminal.unwrap_err()));
        return;
    }

    let stdout = terminal.unwrap().stdout.unwrap();
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        if line.is_err() {
            continue;
        }
        println!("{}", line.unwrap());
    }
}