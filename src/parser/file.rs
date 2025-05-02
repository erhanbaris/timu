use std::fmt::{Display, Formatter};


use crate::ast::{FileAst, FileStatementAst, FunctionDefinitionAst, UseAst};

impl FileAst<'_> {
    pub fn get_functions(&self) -> impl Iterator<Item = &FunctionDefinitionAst<'_>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Function(function) = statement {
                    Some(function)
                } else {
                    None
                }
            })
    }

    pub fn get_uses(&self) -> impl Iterator<Item = &UseAst<'_>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Use(import) = statement {
                    Some(import)
                } else {
                    None
                }
            })
    }
}

impl Display for FileAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, statement) in self.statements.iter().enumerate() {
            write!(f, "{}", statement)?;
            if index < self.statements.len() - 1 {
                writeln!(f)?;
            }
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
            FileStatementAst::Extend(extend) => write!(f, "{}", extend),
            FileStatementAst::Use(import) => write!(f, "{}", import),
        }
    }
}
