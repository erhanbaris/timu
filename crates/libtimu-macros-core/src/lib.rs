use std::fmt::{Display, Formatter};

pub mod traits;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCode {
    pub source: String,
    pub name: String,
}

impl Display for SourceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}