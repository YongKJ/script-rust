use std::env;

pub fn workFolder() -> String {
    env::current_dir().unwrap()
        .to_str().expect("").to_string()
}