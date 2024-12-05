use regex::{Captures, Regex};

pub fn toHump(name: String) -> String {
    let regex = Regex::new("\\-(\\w)").unwrap();
    let subName = name.clone()[1..name.len()].to_string();
    let firstUpperLetter = name[0..1].to_string().to_uppercase();

    firstUpperLetter + regex.replace_all(subName.as_str(), |lstMatch: &Captures|
        lstMatch.get(1).unwrap().as_str().replace("-", "").to_uppercase()).as_ref()
}

pub fn toLine(name: String) -> String {
    let regex = Regex::new("([A-Z])").unwrap();
    let subName = name.clone()[1..name.len()].to_string();
    let firstLowerLetter = name[0..1].to_string().to_lowercase();

    firstLowerLetter + regex.replace_all(subName.as_str(), |lstMatch: &Captures|
        "-".to_string() + lstMatch.get(1).unwrap().as_str().to_lowercase().as_str()).as_ref()
}