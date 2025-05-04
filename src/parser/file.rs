use std::{fmt::{Display, Formatter}, rc::Rc};


use crate::ast::{ClassDefinitionAst, FileAst, FileStatementAst, FunctionDefinitionAst, InterfaceDefinitionAst, UseAst};

impl<'a> FileAst<'a> {
    pub fn get_uses(&self) -> impl Iterator<Item = &UseAst<'a>> {
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

    pub fn get_classes(&self) -> impl Iterator<Item = Rc<ClassDefinitionAst<'a>>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Class(klass) = statement {
                    Some(klass.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_functions(&self) -> impl Iterator<Item = Rc<FunctionDefinitionAst<'a>>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Function(func) = statement {
                    Some(func.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_interfaces(&self) -> impl Iterator<Item = &InterfaceDefinitionAst<'a>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Interface(interface) = statement {
                    Some(interface)
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
