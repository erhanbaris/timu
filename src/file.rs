use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile<'a> {
    path: PathBuf,
    code: &'a str,
}

impl<'a> SourceFile<'a> {
    pub fn new(path: PathBuf, code: &'a str) -> Self {
        Self {
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
}
