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

    fn run_test(&self) {
        ApplicationTest::run();
    }
}

fn main() {
    let script_rust = ScriptRust::new();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        script_rust.run();
        return;
    }

    let flag = args.get(1);
    if *flag.expect("") == "test".to_string() {
        script_rust.run_test();
    }
}
