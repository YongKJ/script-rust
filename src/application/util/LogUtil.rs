use crate::application::config::Global;
use crate::application::pojo::dto::Log::Log;
use std::any::Any;
use std::fmt;

pub fn loggerLine<T: Any + fmt::Display>(log: Log) {
    logger::<T>(log);
    println!();
}

pub fn logger<T: Any + fmt::Display>(log: Log) {
    if !Global::LOG_ENABLE {
        return;
    }

    print!("[{}] {} -> {}: ", log.className(), log.methodName(), log.paramName());
    loggerValue::<T>(log.value());
}

fn loggerValue<T: Any + fmt::Display>(value: &Box<dyn Any>) {
    if !isType::<T>(value) {
        return;
    }

    if let Some(msg) = (*value).downcast_ref::<T>() {
        print!("{}", msg);
    }
}

fn isType<T: Any>(value: &Box<dyn Any>) -> bool {
    value.downcast_ref::<T>().is_some()
}
