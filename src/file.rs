use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile<'a> {
    path: Vec<Cow<'a, str>>,
    code: &'a str,
}

impl<'a> SourceFile<'a> {
    pub fn new(path: Vec<Cow<'a, str>>, code: &'a str) -> Self {
        Self {
            path,
            code,
        }
    }

    pub fn path(&self) -> &Vec<Cow<'a, str>> {
        &self.path
    }

    pub fn code(&self) -> &'a str {
        self.code
    }
}
