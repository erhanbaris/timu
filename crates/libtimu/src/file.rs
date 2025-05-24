use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile<'base> {
    path: Vec<Cow<'base, str>>,
    code: &'base str,
}

impl<'base> SourceFile<'base> {
    pub fn new(path: Vec<Cow<'base, str>>, code: &'base str) -> Self {
        Self {
            path,
            code,
        }
    }

    pub fn path(&self) -> &Vec<Cow<'base, str>> {
        &self.path
    }

    pub fn code(&self) -> &'base str {
        self.code
    }
}
