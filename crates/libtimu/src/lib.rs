//! Timu Language Compiler Library
//!
//! This is the core library for the Timu programming language compiler. It provides
//! a complete compilation pipeline from source code to a type-checked intermediate
//! representation (TIR).
//!
//! # Overview
//!
//! The Timu compiler follows a multi-stage compilation process:
//!
//! 1. **Parsing**: Convert source code into an Abstract Syntax Tree (AST)
//! 2. **Type Resolution**: Build Type Intermediate Representation (TIR) with full type information
//! 3. **Error Reporting**: Collect and report compilation errors with source location information
//!
//! # Architecture
//!
//! The library is organized into several key modules:
//!
//! - [`parser`]: Converts source code text into structured AST nodes
//! - [`ast`]: Abstract Syntax Tree definitions for all language constructs
//! - [`tir`]: Type Intermediate Representation and type checking system
//! - [`error`]: Error handling and reporting infrastructure
//! - [`file`]: Source file management and location tracking
//! - [`nom_tools`]: Parser combinator utilities built on nom
//! - [`map`]: Specialized data structures for compiler use
//!
//! # Usage
//!
//! The main entry points are [`process_code`] for parsing and [`process_ast`] for type checking:
//!
//! ```ignore
//! use libtimu::{process_code, process_ast, nom_tools::State, file::SourceFile};
//!
//! // Parse source code
//! let source = SourceFile::new(vec!["main".into()], "class Test {}".to_string());
//! let state = State::new(source);
//! let ast = process_code(&state)?;
//!
//! // Type check and build TIR
//! let tir_context = process_ast(vec![ast.into()])?;
//! ```
//!
//! # Error Handling
//!
//! All compilation errors are represented as [`TirError`] which provides rich
//! diagnostic information including source locations and suggested fixes.

use std::rc::Rc;

use ast::FileAst;
use error::handle_parser;
use nom::Finish;
use nom_tools::State;
use tir::{TirContext, TirError};

// Public modules that form the compiler's public API
pub mod ast;
pub mod file;

#[rustfmt::skip]
pub mod parser;
pub mod error;
pub mod nom_tools;
pub mod tir;
pub mod map;

#[cfg(test)]
mod tests;

/// Parses Timu source code into an Abstract Syntax Tree (AST).
///
/// This is the first stage of the compilation pipeline. It takes a parser state
/// containing source code and produces a structured AST representation.
///
/// # Arguments
///
/// * `state` - Parser state containing the source code and file information
///
/// # Returns
///
/// * `Ok(FileAst)` - The parsed AST representing the entire source file
/// * `Err(TirError)` - Parsing error with location information
///
/// # Examples
///
/// ```ignore
/// use libtimu::{process_code, nom_tools::State, file::SourceFile};
///
/// let source = SourceFile::new(vec!["example".into()], "class Hello {}".to_string());
/// let state = State::new(source);
/// let ast = process_code(&state)?;
/// ```
///
/// # Errors
///
/// Returns a [`TirError`] if the source code contains syntax errors, including:
/// - Invalid token sequences
/// - Malformed declarations
/// - Unmatched delimiters
/// - Invalid expressions
#[allow(clippy::result_unit_err)]
pub fn process_code<'base>(state: &'base State) -> Result<FileAst<'base>, TirError> {
    let response = parser::parse(state).finish();
    handle_parser(response)
}

/// Performs type checking and builds the Type Intermediate Representation (TIR).
///
/// This is the second stage of the compilation pipeline. It takes one or more
/// ASTs and performs type resolution, building a complete type-checked
/// representation of the program.
///
/// # Arguments
///
/// * `files` - Vector of parsed ASTs to process together
///
/// # Returns
///
/// * `Ok(TirContext)` - Complete TIR context with all type information
/// * `Err(TirError)` - Type checking error with diagnostic information
///
/// # Examples
///
/// ```ignore
/// use libtimu::{process_code, process_ast, nom_tools::State, file::SourceFile};
/// use std::rc::Rc;
///
/// // Parse multiple files
/// let ast1 = process_code(&state1)?;
/// let ast2 = process_code(&state2)?;
///
/// // Type check together
/// let tir_context = process_ast(vec![ast1.into(), ast2.into()])?;
/// ```
///
/// # Type Checking Process
///
/// 1. **Module Registration**: Register all modules and their relationships
/// 2. **Signature Building**: Create type signatures for all declarations
/// 3. **Scope Construction**: Build hierarchical scope tree
/// 4. **Type Resolution**: Resolve all type references and expressions
/// 5. **Error Collection**: Gather any type checking errors
///
/// # Errors
///
/// Returns a [`TirError`] if type checking fails, including:
/// - Type mismatches
/// - Undefined variables or types
/// - Circular dependencies
/// - Import errors
/// - Scope resolution failures
#[allow(clippy::result_unit_err)]
pub fn process_ast(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, TirError> {
    crate::tir::build(files)
}
