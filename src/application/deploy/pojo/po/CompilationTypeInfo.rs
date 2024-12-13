use crate::application::util::DataUtil;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct CompilationTypeInfo {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "target")]
    target: String,
}

impl Display for CompilationTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "CompilationTypeInfo", DataUtil::objToJson(&self))
    }
}

impl CompilationTypeInfo {
    fn new(name: String, target: String) -> Self {
        Self { name, target }
    }

    pub fn of(name: &str, target: &str) -> Self {
        Self::new(name.to_string(), target.to_string())
    }
}

impl CompilationTypeInfo {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }
}

impl CompilationTypeInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> &str {
        &self.target
    }
}
