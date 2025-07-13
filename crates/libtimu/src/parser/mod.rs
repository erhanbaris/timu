//! Timu language parser implementation using nom combinators.
//!
//! This module provides the complete parsing infrastructure for the Timu programming language.
//! It transforms source code text into Abstract Syntax Tree (AST) nodes using nom parser
//! combinators, handling all language constructs including:
//!
//! - Module imports and exports (`use` statements)
//! - Class and interface definitions
//! - Function definitions and calls
//! - Type annotations and expressions
//! - Primitive values (strings, numbers, booleans)
//! - Comments and whitespace handling
//!
//! The parser is designed to provide rich error reporting with precise source locations
//! and context information for debugging and IDE integration.
//!
//! # Architecture
//!
//! The parser is organized into specialized submodules:
//! - [`expression`] - Expression parsing with operator precedence
//! - [`primitive`] - Primitive value parsing (strings, numbers, booleans)
//! - [`class`], [`interface`], [`extend`] - Object-oriented constructs
//! - [`function_definition`], [`function_call`] - Function handling
//! - [`variable`], [`field`] - Variable and field declarations
//! - [`type_info`], [`ref_info`] - Type system support
//! - [`if_condition`], [`body`] - Control flow and code blocks
//! - [`module_use`] - Module system support
//! - [`splited_path`] - Qualified path handling
//!
//! # Usage
//!
//! The main entry point is the [`parse`] function which takes a [`State`] containing
//! the source file and returns a [`FileAst`] representing the parsed program.

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::combinator::{cut, map, opt, recognize};
use nom::sequence::{pair, preceded, terminated};
use nom::Err;
use nom::branch::alt;
use nom::error::context;
use nom::multi::{many0, many0_count};
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst, UseAst};
use crate::nom_tools::{NomSpan, State, cleanup};

mod body;
mod class;
mod expression;
mod extend;
mod field;
mod file;
mod function_definition;
mod function_call;
mod if_condition;
mod interface;
mod module_use;
mod primitive;
mod ref_info;
mod type_info;
mod variable;
pub mod splited_path;

/// Type alias for parser errors with rich context information
pub type TimuParserError<'base> = VerboseError<NomSpan<'base>>;

/// Parses a complete Timu source file into an Abstract Syntax Tree
/// 
/// This function is the main entry point for parsing Timu source code. It takes
/// parser state containing the source file and attempts to parse all top-level
/// constructs (classes, interfaces, functions, use statements, extensions).
/// 
/// # Arguments
/// * `state` - Parser state containing the source file and indexing information
/// 
/// # Returns
/// * `Ok((remaining, ast))` - Successful parse with remaining input and file AST
/// * `Err(error)` - Parse error with detailed context and location information
/// 
/// # Errors
/// Returns a `TimuParserError` if:
/// - Unknown or invalid syntax is encountered
/// - Required elements are missing (e.g., missing closing braces)
/// - Malformed constructs are found
/// 
/// # Example
/// ```ignore
/// let source_file = SourceFile::new(vec!["main.tim".into()], source_code);
/// let state = State::new(source_file);
/// let (remaining, file_ast) = parse(&state)?;
/// ```
pub fn parse<'base>(state: &'base State) -> IResult<NomSpan<'base>, FileAst<'base>, TimuParserError<'base>> {
    let file = state.file.clone();
    let extra = state.clone();

    let input = NomSpan::new_extra(state.file.code().as_str(), extra);
    let (remaining, statements) =
        many0(alt((
            cleanup(UseAst::parse_for_file),
            cleanup(ClassDefinitionAst::parse),
            cleanup(FunctionDefinitionAst::parse_for_file),
            cleanup(InterfaceDefinitionAst::parse),
            cleanup(ExtendDefinitionAst::parse),
        )))
        .parse(input)?;

    if remaining.len() > 0 {
        let error = VerboseError {
            errors: vec![(remaining, VerboseErrorKind::Context("Unknown syntax"))],
        };
        return Err(Err::Failure(error));
    }

    Ok((
        remaining,
        FileAst {
            file,
            statements,
        },
    ))
}

/// Parses block comments in the form `/* ... */`
/// 
/// This parser recognizes multi-line comments that start with `/*` and end with `*/`.
/// Currently unused in the main parser but available for future comment handling.
#[allow(warnings)]
pub fn comment<'base>(input: NomSpan<'base>) -> IResult<NomSpan<'base>, NomSpan<'base>, TimuParserError<'base>> {
    preceded(char('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),))).parse(input)
}

/// Parses the optional `pub` visibility modifier
/// 
/// Returns `Some(span)` if the `pub` keyword is found, `None` otherwise.
/// Used to determine the visibility of classes, functions, and other declarations.
pub fn is_public(input: NomSpan<'_>) -> IResult<NomSpan<'_>, Option<NomSpan<'_>>, TimuParserError<'_>> {
    cleanup(opt(tag("pub"))).parse(input)
}

/// Parses the optional `?` nullable type modifier
/// 
/// Returns `true` if a `?` character is found before a type, indicating
/// that the type is nullable in Timu's type system.
pub fn is_nullable(input: NomSpan<'_>) -> IResult<NomSpan<'_>, bool, TimuParserError<'_>> {
    cleanup(map(opt(char('?')), |item| item.is_some())).parse(input)
}

/// Parses the optional `ref` reference type modifier
/// 
/// Returns `true` if the `ref` keyword is found, indicating that
/// the type is a reference rather than a value type.
pub fn is_reference(input: NomSpan<'_>) -> IResult<NomSpan<'_>, bool, TimuParserError<'_>> {
    cleanup(map(opt(tag("ref")), |item| item.is_some())).parse(input)
}

/// Parses an identifier with a custom error message
/// 
/// This function expects an identifier and provides a custom error context
/// if parsing fails, making error messages more helpful for users.
/// 
/// # Arguments
/// * `message` - The error message to display if identifier parsing fails
/// * `input` - The input span to parse from
pub fn expected_ident<'base>(message: &'static str, input: NomSpan<'base>) -> IResult<NomSpan<'base>, NomSpan<'base>, TimuParserError<'base>> {
    context(message, cut(ident())).parse(input)
}

/// Parses a valid Timu identifier
/// 
/// Identifiers in Timu must start with an alphabetic character or underscore,
/// followed by any number of alphanumeric characters or underscores.
/// Whitespace is automatically cleaned up around the identifier.
/// 
/// # Examples
/// Valid identifiers: `foo`, `_bar`, `hello_world`, `MyClass`, `test123`
pub fn ident<'base>() -> impl Parser<NomSpan<'base>, Output = NomSpan<'base>, Error = TimuParserError<'base>> {
    cleanup(recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_")))))))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::vec;

    use rstest::rstest;

    use crate::{
        ast::{PrimitiveValue, TypeNameAst},
        file::SourceFile, nom_tools::State, parser::primitive::{number, string},
    };

    use super::NomSpan;

    #[rstest]
    #[case(r#""hello""#, PrimitiveValue::String("hello".into()))]
    #[case(r#""hello\nworld""#, PrimitiveValue::String("hello\nworld".into()))]
    #[case(r#""hello\tworld""#, PrimitiveValue::String("hello\tworld".into()))]
    #[case(r#""hello\\world""#, PrimitiveValue::String("hello\\world".into()))]
    #[case(r#""hello\"world""#, PrimitiveValue::String("hello\"world".into()))]
    #[case(r#""hello/world""#, PrimitiveValue::String("hello/world".into()))]
    fn string_test<'base>(#[case] code: &'base str, #[case] expected: PrimitiveValue) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let (_, string) = string(input).unwrap();

        assert_eq!(string, expected, "Parsed string does not match expected");
    }

    #[rstest]
    #[case("true", PrimitiveValue::Bool(true))]
    #[case("false", PrimitiveValue::Bool(false))]
    fn boolean_test<'base>(#[case] code: &'base str, #[case] expected: PrimitiveValue) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let (_, (_, boolean)) = PrimitiveValue::parse(input).unwrap();

        assert_eq!(boolean, expected, "Parsed boolean does not match expected");
    }

    #[rstest]
    #[case("123", PrimitiveValue::I8(123))]
    #[case("-123", PrimitiveValue::I8(-123))]
    #[case("255", PrimitiveValue::U8(255))]
    #[case("32767", PrimitiveValue::I16(32767))]
    #[case("65535", PrimitiveValue::U16(65535))]
    #[case("2147483647", PrimitiveValue::I32(2147483647))]
    #[case("4294967295", PrimitiveValue::U32(4294967295))]
    #[case("9223372036854775807", PrimitiveValue::I64(9223372036854775807))]
    #[case("18446744073709551615", PrimitiveValue::U64(18446744073709551615))]
    fn integer_test<'base>(#[case] code: &'base str, #[case] expected: PrimitiveValue) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, expected, "Parsed integer does not match expected");
    }

    #[rstest]
    #[case("string", false, vec!["string"])]
    #[case(" string ", false, vec!["string"])]
    #[case("string.base", false, vec!["string", "base"])]
    #[case("string.base . test", false, vec!["string", "base", "test"])]
    #[case(" string   .        base        . test", false, vec!["string", "base", "test"])]
    #[case(" ? string   .        base        . test", true, vec!["string", "base", "test"])]
    #[case("?string", true, vec!["string"])]
    fn parse_type_name_test<'base>(#[case] code: &'base str, #[case] nullable: bool, #[case] expected: Vec<&str>) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let result = TypeNameAst::parse(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.nullable, nullable, "nullable info does not match expected");

        let parsed: Vec<_> = parsed.names.into_iter().map(|s| s.text.to_string()).collect();
        assert_eq!(parsed, expected, "Parsed type name does not match expected");
    }

    #[rstest]
    #[case("1.2", 1.2, 1)]
    #[case("2.2", 2.2, 1)]
    #[case("2.20000000000000", 2.2, 14)]
    #[case("1.23", 1.23, 2)]
    #[case("1024.0", 1024.0, 1)]
    #[case("-1024.0", -1024.0, 1)]
    #[case("1.0e-7", 1.0e-7, 1)]
    #[case("123456789.0e+7", 1234567890000000.0, 1)]
    fn float_test<'base>(#[case] code: &'base str, #[case] expected: f64, #[case] dot_place: u8) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, PrimitiveValue::Float(expected, dot_place), "Parsed type name does not match expected");
    }

    #[rstest]
    #[case("1.7976931348623157E+300", 1797693134862315647938267463293564874600617718166104931943772918675666340832537361829116717802808644459281636809871223917508254623303542508952824391223228755068260245991425339269180741930617451225745000201898803634683406373476746438518757597828943183163861984879702567874510145974570799930947550576640.0000000000000000, 16)]
    fn double_test<'base>(#[case] code: &'base str, #[case] expected: f64, #[case] dot_place: u8) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, PrimitiveValue::Double(expected, dot_place), "Parsed type name does not match expected");
    }
}
