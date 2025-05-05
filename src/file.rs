#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile<'a> {
    path: Vec<String>,
    code: &'a str,
}

impl<'a> SourceFile<'a> {
    pub fn new(path: Vec<String>, code: &'a str) -> Self {
        Self {
            path,
            code,
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn code(&self) -> &'a str {
        self.code
    }
}
