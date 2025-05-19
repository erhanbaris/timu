use std::{fmt::{Display, Formatter}, rc::Rc};


use crate::ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FileStatementAst, FunctionDefinitionAst, InterfaceDefinitionAst, UseAst};

impl<'base> FileAst<'base> {
    pub fn get_uses(&self) -> impl Iterator<Item = Rc<UseAst<'base>>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Use(import) = statement {
                    Some(import.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_classes(&self) -> impl Iterator<Item = Rc<ClassDefinitionAst<'base>>> {
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

    pub fn get_functions(&self) -> impl Iterator<Item = Rc<FunctionDefinitionAst<'base>>> {
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

    pub fn get_interfaces(&self) -> impl Iterator<Item = Rc<InterfaceDefinitionAst<'base>>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Interface(interface) = statement {
                    Some(interface.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_extends(&self) -> impl Iterator<Item = Rc<ExtendDefinitionAst<'base>>> {
        self.statements
            .iter()
            .filter_map(|statement| {
                if let FileStatementAst::Extend(extend) = statement {
                    Some(extend.clone())
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
