use std::env;
use std::fs::File;

pub fn workFolder() -> String {
    env::current_dir().unwrap()
        .to_str().expect("").to_string()
}

pub fn create(fileName: String) {
    let result = File::create(fileName);
    if result.is_err() {
        println!("{}", result.unwrap_err())
    }
}