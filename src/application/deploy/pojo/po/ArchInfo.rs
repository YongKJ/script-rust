use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
use crate::application::util::DataUtil;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct ArchInfo {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "compilation_type")]
    compilationType: Vec<CompilationTypeInfo>,
}

impl Display for ArchInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "ArchInfo", DataUtil::objToJson(&self))
    }
}

impl ArchInfo {
    fn new(name: String, compilationType: Vec<CompilationTypeInfo>) -> Self {
        Self { name, compilationType }
    }

    pub fn of(name: &str, compilationType: Vec<CompilationTypeInfo>) -> Self {
        Self::new(name.to_string(), compilationType)
    }
}

impl ArchInfo {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_compilationType(&mut self, compilationType: Vec<CompilationTypeInfo>) {
        self.compilationType = compilationType;
    }
}

impl ArchInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn compilationType(&self) -> &Vec<CompilationTypeInfo> {
        &self.compilationType
    }
}