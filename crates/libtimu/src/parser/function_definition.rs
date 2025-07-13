//! Function definition parsing for the Timu language.
//!
//! This module handles parsing of function definitions in various contexts within Timu.
//! Functions can be defined at the module level, within classes, or as part of interface
//! extensions. The parser handles all the different syntax variations and contexts.
//!
//! # Function Definition Syntax
//!
//! ## Basic Function Definition
//! ```timu
//! func functionName(arg1: Type1, arg2: Type2): ReturnType {
//!     // function body
//! }
//! ```
//!
//! ## Public Function (Module Level)
//! ```timu
//! pub func publicFunction(): void {
//!     // function body
//! }
//! ```
//!
//! ## Method Definition (Class Context)
//! ```timu
//! func methodName(this, arg: Type): ReturnType {
//!     // method body with access to this
//! }
//! ```
//!
//! ## Function Arguments
//! - **Regular arguments**: `name: Type`
//! - **This parameter**: `this` (for methods)
//! - **Multiple arguments**: Comma-separated list
//!
//! # Parsing Contexts
//!
//! Functions can be parsed in different contexts:
//! - **File/Module level**: Top-level functions with optional `pub` visibility
//! - **Class methods**: Functions defined within class bodies
//! - **Interface extensions**: Functions added to existing types
//!
//! # Return Types
//!
//! All functions must specify a return type:
//! - `void` for functions that don't return a value
//! - Primitive types: `i32`, `string`, `bool`, etc.
//! - Complex types: Class names, qualified types
//! - Nullable types: `?Type` for optional returns

use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{consumed, cut, map, opt, peek};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::{IResult, Parser, sequence::delimited};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{
    AstIndex, BodyAst, ClassDefinitionFieldAst, ExtendDefinitionFieldAst, FileStatementAst, FunctionArgumentAst, FunctionDefinitionAst, FunctionDefinitionLocationAst, TypeNameAst
};
use crate::nom_tools::{NomSpan, cleanup};
use crate::parser::{expected_ident, ident, is_public};

use super::TimuParserError;

impl<'base> FunctionDefinitionAst<'base> {
    /// Parses a function definition for use at file/module level
    /// 
    /// This parser variant handles function definitions that appear at the top level
    /// of a source file. These functions can optionally be marked as `pub` for
    /// external visibility and become part of the module's public API.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed file-level function
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Examples
    /// ```timu
    /// func privateFunction(): void { }
    /// pub func publicFunction(arg: i32): string { }
    /// ```
    pub fn parse_for_file(input: NomSpan<'base>) -> IResult<NomSpan<'base>, FileStatementAst<'base>, TimuParserError<'base>> {
        let (input, (_, function)) = Self::parse(input)?;
        Ok((input, FileStatementAst::Function(function.into())))
    }

    /// Parses a function definition within a class context
    /// 
    /// This parser variant handles method definitions inside class bodies. The parsed
    /// function is automatically tagged with the containing class name for proper
    /// semantic analysis and method resolution.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// * `class_name` - The name of the containing class
    /// 
    /// # Returns
    /// * `Ok((remaining, field))` - Successfully parsed class method
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Examples
    /// ```timu
    /// class MyClass {
    ///     func method(this): void { }
    ///     func calculate(this, x: i32): i32 { }
    /// }
    /// ```
    pub fn parse_class_function(input: NomSpan<'base>, class_name: NomSpan<'base>) -> IResult<NomSpan<'base>, ClassDefinitionFieldAst<'base>, TimuParserError<'base>> {
        let (input, (_, mut function)) = Self::parse(input)?;
        function.location = FunctionDefinitionLocationAst::Class(class_name.into()).into();
        Ok((input, ClassDefinitionFieldAst::Function(function)))
    }

    /// Parses a function definition within an extend block
    /// 
    /// This parser variant handles function definitions that extend existing types
    /// with additional methods. Extended functions are implicitly public and cannot
    /// be explicitly marked with `pub` (which would be redundant).
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, field))` - Successfully parsed extension function
    /// * `Err(error)` - Parse error, including attempt to use `pub` modifier
    /// 
    /// # Examples
    /// ```timu
    /// extend string {
    ///     func reverse(this): string { }
    ///     func isEmpty(this): bool { }
    /// }
    /// ```
    pub fn parse_extend_function(input: NomSpan<'base>) -> IResult<NomSpan<'base>, ExtendDefinitionFieldAst<'base>, TimuParserError<'base>> {
        let (input, (is_public, function)) = Self::parse(input)?;
        if let Some(is_public) = is_public {
            let error = VerboseError {
                errors: vec![(is_public, VerboseErrorKind::Context("All extended functions already public"))],
            };
            return Err(nom::Err::Failure(error));
        }
        Ok((input, ExtendDefinitionFieldAst::Function(function)))
    }

    /// Main function definition parser
    /// 
    /// This is the core parser that handles the complete function definition syntax.
    /// It parses the visibility modifier, function keyword, name, parameters, return
    /// type, and body. The parser ensures all required components are present and
    /// properly formatted.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// A tuple containing:
    /// * `Option<NomSpan>` - The `pub` keyword span if present
    /// * `FunctionDefinitionAst` - The complete function definition
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing function name
    /// - Missing or malformed parameter list
    /// - Missing return type
    /// - Missing function body
    /// - Invalid syntax in any component
    /// 
    /// # Syntax
    /// ```timu
    /// [pub] func name(arg1: Type1, arg2: Type2): ReturnType {
    ///     // body
    /// }
    /// ```
    pub fn parse(
        input: NomSpan<'base>,
    ) -> IResult<NomSpan<'base>, (Option<NomSpan<'base>>, FunctionDefinitionAst<'base>), TimuParserError<'base>> {
        let (input, is_public) = is_public(input)?;
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) = expected_ident("Missing function name", input)?;
        let (input, _) = context("Missing '('", cut(peek(cleanup(char('('))))).parse(input)?;
        let (input, (arguments_span, arguments)) =
            consumed(map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse)), context("Missing ')'", cut(char(')')))), |items| {
                items
            }))
            .parse(input)?;

        let (input, _) = context("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = context("Missing function return type", cut(cleanup(cleanup(TypeNameAst::parse)))).parse(input)?;

        let (input, body) = BodyAst::parse(input)?;
        let index = AstIndex(input.extra.indexer.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let original_is_public = is_public.clone();

        Ok((
            input,
            (original_is_public, FunctionDefinitionAst {
                is_public: is_public.map(|item| item.into()),
                name: name.into(),
                arguments,
                arguments_span: arguments_span.into(),
                body: body.into(),
                return_type,
                location: FunctionDefinitionLocationAst::Module.into(),
                index
            },
        )))
    }
}

impl Display for FunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}func {}(", if self.is_public.is_some() { "pub " } else { "" }, self.name.text)?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{arg}")?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {} {}", self.return_type, self.body)
    }
}

impl FunctionArgumentAst<'_> {
    /// Parses a single function argument
    /// 
    /// Function arguments can be either regular parameters with name and type,
    /// or the special `this` parameter for methods. The parser handles both
    /// cases and returns the appropriate AST variant.
    /// 
    /// # Argument Types
    /// - **This parameter**: `this` - Special parameter for method access
    /// - **Regular parameter**: `name: Type` - Named parameter with type annotation
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, argument))` - Successfully parsed function argument
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Examples
    /// ```timu
    /// this                    // Method this parameter
    /// name: string           // Regular typed parameter
    /// count: i32             // Integer parameter
    /// callback: ?Function    // Optional function parameter
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing colon after parameter name
    /// - Invalid type specification
    /// - Malformed parameter syntax
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, FunctionArgumentAst<'_>, TimuParserError<'_>> {
        let (input, this) = cleanup(opt(tag("this"))).parse(input)?;

        if let Some(this) = this {
            return Ok((input, FunctionArgumentAst::This(this.into())));
        }

        let (input, (name, field_type)) = (cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse)).parse(input)?;
        Ok((
            input,
            FunctionArgumentAst::Argument {
                name: name.into(),
                field_type,
            },
        ))
    }
}

impl Display for FunctionArgumentAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionArgumentAst::This(_) => write!(f, "this"),
            FunctionArgumentAst::Argument { name, field_type } => write!(f, "{}: {}", name.text, field_type),
        }
    }
}