use std::{error::Error, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    path: Rc<Vec<String>>,
    code: Rc<String>,
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
