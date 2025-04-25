use std::fmt::{Display, Formatter};

use crate::ast::{FileAst, FileStatementAst};

impl Display for FileAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Display for FileStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatementAst::Class(class) => write!(f, "{}", class),
            FileStatementAst::Function(function) => write!(f, "{}", function),
            FileStatementAst::Interface(interface) => write!(f, "{}", interface),
        }
    }
}
