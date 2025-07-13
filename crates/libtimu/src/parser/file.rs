//! File AST utility methods and display implementations.
//!
//! This module provides utility methods for the `FileAst` structure, which represents
//! a complete parsed Timu source file. It includes iterators for extracting different
//! types of declarations (classes, functions, interfaces, extends, uses) and display
//! implementations for formatting the AST back to source code.
//!
//! # File Structure
//!
//! A Timu file can contain the following top-level declarations:
//! - **Use statements**: Import declarations for accessing external modules
//! - **Class definitions**: Object-oriented type definitions with fields and methods
//! - **Interface definitions**: Contract specifications for implementing types
//! - **Function definitions**: Standalone function declarations
//! - **Extend definitions**: Extensions that add functionality to existing types
//!
//! # Iterator Methods
//!
//! The module provides convenient iterator methods for filtering file statements by type:
//! - `get_uses()`: Returns all use/import statements
//! - `get_classes()`: Returns all class definitions
//! - `get_functions()`: Returns all standalone function definitions
//! - `get_interfaces()`: Returns all interface definitions
//! - `get_extends()`: Returns all extend definitions
//!
//! These methods return iterators that yield `Rc<T>` references to the respective AST nodes,
//! allowing efficient shared access to the parsed declarations.

use std::{fmt::{Display, Formatter}, rc::Rc};


use crate::ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FileStatementAst, FunctionDefinitionAst, InterfaceDefinitionAst, UseAst};

impl<'base> FileAst<'base> {
    /// Returns an iterator over all use/import statements in the file
    /// 
    /// This method filters the file's top-level statements and returns only
    /// the use statements, which handle module imports and dependencies.
    /// 
    /// # Returns
    /// An iterator yielding `Rc<UseAst>` references to each use statement
    /// 
    /// # Example Usage
    /// Used during semantic analysis to resolve module dependencies and
    /// build the import graph for the compilation unit.
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

    /// Returns an iterator over all class definitions in the file
    /// 
    /// This method extracts all class declarations from the file's statements,
    /// providing access to object-oriented type definitions with their fields
    /// and methods.
    /// 
    /// # Returns
    /// An iterator yielding `Rc<ClassDefinitionAst>` references to each class definition
    /// 
    /// # Example Usage
    /// Used during type resolution to register class types and analyze
    /// inheritance relationships and method signatures.
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

    /// Returns an iterator over all standalone function definitions in the file
    /// 
    /// This method filters for function declarations that are defined at the file level,
    /// excluding methods that are part of class or interface definitions.
    /// 
    /// # Returns
    /// An iterator yielding `Rc<FunctionDefinitionAst>` references to each function definition
    /// 
    /// # Example Usage
    /// Used during function resolution to register callable functions and
    /// analyze their signatures for type checking and call resolution.
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

    /// Returns an iterator over all interface definitions in the file
    /// 
    /// This method extracts interface declarations which define contracts
    /// that classes and other types can implement.
    /// 
    /// # Returns
    /// An iterator yielding `Rc<InterfaceDefinitionAst>` references to each interface definition
    /// 
    /// # Example Usage
    /// Used during type resolution to register interface contracts and
    /// validate that implementing types satisfy all interface requirements.
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

    /// Returns an iterator over all extend definitions in the file
    /// 
    /// This method extracts extend declarations which add new functionality
    /// to existing types by implementing additional interfaces or adding methods.
    /// 
    /// # Returns
    /// An iterator yielding `Rc<ExtendDefinitionAst>` references to each extend definition
    /// 
    /// # Example Usage
    /// Used during type resolution to process type extensions and validate
    /// that extended types can support the additional interface requirements.
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
            write!(f, "{statement}")?;
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
            FileStatementAst::Class(class) => write!(f, "{class}"),
            FileStatementAst::Function(function) => write!(f, "{function}"),
            FileStatementAst::Interface(interface) => write!(f, "{interface}"),
            FileStatementAst::Extend(extend) => write!(f, "{extend}"),
            FileStatementAst::Use(import) => write!(f, "{import}"),
        }
    }
}
