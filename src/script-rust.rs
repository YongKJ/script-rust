#![allow(
    dead_code,
    unused_imports,
    non_snake_case
)]

mod application;

use crate::application::{
    Application,
    ApplicationTest,
};
use std::env;

struct ScriptRust {}

impl ScriptRust {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        Application::run();
    }

    fn runTest(&self) {
        ApplicationTest::run();
    }
}

fn main() {
    let scriptRust = ScriptRust::new();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        scriptRust.run();
        return;
    }

    let flag = args.get(1);
    if *flag.expect("") == "test".to_string() {
        scriptRust.runTest();
    }
}
