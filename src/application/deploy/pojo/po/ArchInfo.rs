use crate::application::deploy::pojo::po::CompilationTypeInfo::CompilationTypeInfo;
use crate::application::util::DataUtil;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchInfo {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "compilation_type")]
    compilationTypes: Vec<CompilationTypeInfo>,
}

impl Display for ArchInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}\n", "ArchInfo", DataUtil::objToJson(&self))
    }
}

impl ArchInfo {
    fn new(name: String, compilationTypes: Vec<CompilationTypeInfo>) -> Self {
        Self { name, compilationTypes }
    }

    pub fn of(name: &str, compilationTypes: Vec<CompilationTypeInfo>) -> Self {
        Self::new(name.to_string(), compilationTypes)
    }
}

impl ArchInfo {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_compilationTypes(&mut self, compilationTypes: Vec<CompilationTypeInfo>) {
        self.compilationTypes = compilationTypes;
    }
}

impl ArchInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn compilationTypes(&self) -> &Vec<CompilationTypeInfo> {
        &self.compilationTypes
    }
}