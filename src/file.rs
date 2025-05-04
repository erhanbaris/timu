use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile<'a> {
    name: &'a str,
    path: PathBuf,
    code: &'a str,
}

impl<'a> SourceFile<'a> {
    pub fn new(name: &'a str, path: PathBuf, code: &'a str) -> Self {
        Self {
            name,
            path,
            code,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn code(&self) -> &'a str {
        self.code
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}
