use crate::application::config::Global;
use crate::application::pojo::dto::Log::Log;
use std::any::Any;
use std::fmt;
use std::io;

pub fn loggerLine(log: Log) {
    logger(log);
    println!();
}

pub fn logger(log: Log) {
    if !Global::LOG_ENABLE {
        return;
    }

    print!("[{}] {} -> {}: ", log.className(), log.methodName(), log.paramName());
    loggerValue(log.value());
}

fn loggerValue(value: &Box<dyn Any>) {
    if let Some(ioErr) = (*value).downcast_ref::<io::Error>() {
        print!("{}", ioErr);
    } else if let Some(fmtErr) = (*value).downcast_ref::<fmt::Error>() {
        print!("{}", fmtErr);
    } else if let Some(string) = (*value).downcast_ref::<String>() {
        print!("{}", string);
    }
}
