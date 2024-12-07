use crate::application::config::Global;
use crate::application::pojo::dto::Log::Log;
use std::fmt::Display;

pub fn loggerLine<T: Display>(log: Log<T>) {
    if !Global::LOG_ENABLE {
        return;
    }

    println!("[{}] {} -> {}: {}", log.className(), log.methodName(), log.paramName(), log.value());
}

pub fn logger<T: Display>(log: Log<T>) {
    if !Global::LOG_ENABLE {
        return;
    }

    print!("[{}] {} -> {}: {}", log.className(), log.methodName(), log.paramName(), log.value());
}
