use serde::Serialize;

pub fn objToJson<T: Serialize>(obj: T) -> String {
    serde_json::to_string(&obj).unwrap()
}