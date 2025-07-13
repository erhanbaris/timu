//! Module import statement parsing for the Timu language.
//!
//! This module handles parsing of `use` statements, which are the primary mechanism for
//! importing functionality from other modules in Timu. Import statements enable code
//! organization and namespace management by allowing modules to access functionality
//! defined in other source files.
//!
//! # Import Statement Syntax
//!
//! Timu supports several forms of import statements:
//!
//! ## Basic Import
//! ```timu
//! use module_name;
//! ```
//! Imports a module, making it accessible by its original name.
//!
//! ## Qualified Import
//! ```timu
//! use std.collections.HashMap;
//! use ui.components.Button;
//! ```
//! Imports specific items from nested module hierarchies using dot notation.
//!
//! ## Aliased Import
//! ```timu
//! use std.collections.HashMap as Map;
//! use very.long.module.name as Short;
//! ```
//! Imports a module with a custom alias for convenience or to avoid naming conflicts.
//!
//! # Module Resolution
//!
//! Module paths in import statements correspond to the file system structure:
//! - `std.collections` maps to `std/collections.tim` or `std/collections/mod.tim`
//! - Nested paths create module hierarchies for namespace organization
//! - The final component is the specific item being imported
//!
//! # Import Processing
//!
//! The parser extracts several key components from import statements:
//! - **Import path** - The dot-separated module path
//! - **Optional alias** - A custom name for the imported module
//! - **Source location** - Position information for error reporting
//!
//! # Integration with Module System
//!
//! Parsed import statements are processed by the TIR (Type Intermediate Representation)
//! system to:
//! - Resolve module paths to actual source files
//! - Check import validity and circular dependency detection
//! - Build symbol tables for name resolution
//! - Enable qualified access to imported functionality

use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{consumed, cut, opt};
use nom::error::context;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

use crate::ast::{FileStatementAst, UseAst};
use crate::nom_tools::{NomSpan, cleanup};
use crate::parser::ident;

use super::splited_path::SplitedPath;
use super::TimuParserError;

impl UseAst<'_> {
    /// Parses a complete `use` statement with optional aliasing
    /// 
    /// This is the main parser for import statements. It handles the full syntax
    /// including the `use` keyword, qualified module path, optional `as` alias,
    /// and terminating semicolon. The parser extracts both the import path and
    /// any alias for later module resolution.
    /// 
    /// # Parsing Logic
    /// 1. Parse the `use` keyword
    /// 2. Parse the dot-separated module path (required)
    /// 3. Optionally parse `as` keyword followed by alias identifier
    /// 4. Require terminating semicolon
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, ast))` - Successfully parsed use statement
    /// * `Err(error)` - Parse error with detailed context
    /// 
    /// # Syntax Examples
    /// ```timu
    /// use module;                    // Basic import
    /// use std.collections.HashMap;   // Qualified import
    /// use long.module.name as Short; // Aliased import
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing module path after `use`
    /// - Missing alias after `as` keyword
    /// - Missing terminating semicolon
    /// - Invalid identifier syntax in path or alias
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, UseAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("use")).parse(input)?;
        let (input, (import_span, splited_import)) = context("Module path missing", cut(consumed(cleanup(separated_list1(char('.'), ident()))))).parse(input)?;
        let import = SplitedPath::new(import_span.into(), splited_import.into_iter().map(|item| item.into()).collect::<Vec<_>>());
        
        let (input, alias) = match opt(cleanup(tag("as"))).parse(input)? {
            (input, Some(_)) => {
                let (input, alias) = context("Module alias missing", cut(cleanup(ident()))).parse(input)?;
                (input, Some(alias))
            }
            (input, None) => (input, None),
        };
        
        let (input, _) = context("Missing ';'", cut(cleanup(char(';')))).parse(input)?;

        Ok((
            input,
            UseAst {
                import,
                alias: alias.map(|item| item.into()),
            },
        ))
    }

    /// Parses a use statement for inclusion in file-level statement lists
    /// 
    /// This parser variant wraps the main use statement parser for integration
    /// with the file parsing system. Import statements are top-level constructs
    /// that appear alongside classes, functions, and other module-level declarations.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed file statement
    /// * `Err(error)` - Parse error from the underlying use parser
    /// 
    /// # Usage Context
    /// This function is called by the file parser when processing the sequence
    /// of top-level statements in a source file. Import statements typically
    /// appear at the beginning of files but can be placed anywhere among
    /// top-level declarations.
    pub fn parse_for_file(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FileStatementAst<'_>, TimuParserError<'_>> {
        let (input, import) = Self::parse(input)?;
        Ok((input, FileStatementAst::Use(import.into())))
    }
}

impl Display for UseAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "use ")?;
        write!(f, "{}", self.import.text)?;
        if let Some(alias) = &self.alias {
            write!(f, " as {}", alias.text)?;
        }
        write!(f, ";")
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use rstest::rstest;

    use crate::{file::SourceFile, nom_tools::State};

    #[rstest]
    #[case("use test;", "use test;")]
    #[case("use test as a;", "use test as a;")]
    #[case(" use test ; ", "use test;")]
    #[case("use test1.test2;", "use test1.test2;")]
    #[case("use test1.test2.test3;", "use test1.test2.test3;")]
    #[case(r#"use foo1.foo2.foo3;
use bar1.bar2.bar3;"#, r#"use foo1.foo2.foo3;
use bar1.bar2.bar3;"#)]
    fn module_use_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let (_, response) = crate::parser::parse(&state).finish().unwrap();
        assert_eq!(response.to_string(), expected, "{code}");
    }
}