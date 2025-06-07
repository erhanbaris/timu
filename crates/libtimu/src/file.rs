use std::{error::Error, path::PathBuf, sync::Arc};

use libtimu_macros_core::SourceCode;
use miette::NamedSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub path: Arc<Vec<String>>,
    pub code: Arc<String>,
}

impl From<SourceFile> for NamedSource<String> {
    fn from(file: SourceFile) -> Self {
        let pathbuffer = PathBuf::from_iter(file.path.iter());
        let path = pathbuffer.to_string_lossy();
        NamedSource::new(path, file.code.to_string())
    }
}

impl From<SourceFile> for SourceCode {
    fn from(file: SourceFile) -> Self {
        let pathbuffer = PathBuf::from_iter(file.path.iter());
        let path = pathbuffer.to_string_lossy();
        
        SourceCode {
            source: file.code.to_string(),
            name: path.to_string()   
        }
    }
}

impl Error for SourceFile {}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceFile: {:?}", self.path)
    }
}

impl SourceFile {
    pub fn new(path: Vec<String>, code: String) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn code(&self) -> &String {
        self.code.as_ref()
    }
}
