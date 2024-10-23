use crate::application::config::Global;
use crate::application::pojo::dto::Log::Log;
use std::fmt::Display;

pub fn loggerLine(log: Log) {
    if !Global::LOG_ENABLE {
        return;
    }

    println!("[{}] {} -> {}: {}", log.className(), log.methodName(), log.paramName(), log.value().as_ref());
}

pub fn logger(log: Log) {
    if !Global::LOG_ENABLE {
        return;
    }

    print!("[{}] {} -> {}: {}", log.className(), log.methodName(), log.paramName(), log.value().as_ref());
}
