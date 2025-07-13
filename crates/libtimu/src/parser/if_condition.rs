//! Conditional statement parsing for the Timu language.
//!
//! This module handles parsing of `if` statements and conditional expressions,
//! which are fundamental control flow constructs in Timu. The parser supports
//! the full range of conditional syntax including if-else chains, else-if
//! clauses, and nested conditionals.
//!
//! # Conditional Statement Syntax
//!
//! Timu supports several forms of conditional statements:
//!
//! ## Simple If Statement
//! ```timu
//! if (condition) {
//!     // code to execute if condition is true
//! }
//! ```
//!
//! ## If-Else Statement
//! ```timu
//! if (condition) {
//!     // true branch
//! } else {
//!     // false branch
//! }
//! ```
//!
//! ## If-Else If-Else Chain
//! ```timu
//! if (condition1) {
//!     // first condition true
//! } else if (condition2) {
//!     // second condition true
//! } else if (condition3) {
//!     // third condition true
//! } else {
//!     // all conditions false
//! }
//! ```
//!
//! ## Nested Conditionals
//! ```timu
//! if (outerCondition) {
//!     if (innerCondition) {
//!         // nested logic
//!     }
//! }
//! ```
//!
//! # Conditional Expressions
//!
//! Conditions can be any valid expression that evaluates to a boolean value:
//! - **Boolean literals**: `true`, `false`
//! - **Comparison operators**: `==`, `!=`, `<`, `>`, `<=`, `>=`
//! - **Logical operators**: `&&` (and), `||` (or), `!` (not)
//! - **Function calls**: `isValid()`, `hasValue()`
//! - **Variable references**: `flag`, `result`
//! - **Complex expressions**: `(a > b) && (c < d)`
//!
//! # Parsing Features
//!
//! ## Expression Handling
//! - Supports all expression types as conditions
//! - Proper operator precedence handling
//! - Parenthesized expressions for clarity
//!
//! ## Code Block Integration
//! - Each branch uses standard code block syntax
//! - Supports empty blocks `{}`
//! - Nested statement parsing within blocks
//!
//! ## Error Recovery
//! - Clear error messages for missing parts
//! - Context-aware error reporting
//! - Helpful suggestions for common mistakes
//!
//! # Integration with Language Features
//!
//! Conditional statements integrate with:
//! - **Variable scope**: Each block creates a new scope
//! - **Type checking**: Condition expressions must evaluate to boolean
//! - **Control flow**: Part of the broader statement parsing system
//! - **Function bodies**: Can appear as statements within functions

use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{IResult, Parser};

use crate::ast::{BodyAst, BodyStatementAst, ExpressionAst, IfConditionAst};
use crate::nom_tools::{cleanup, NomSpan};

use super::TimuParserError;

impl IfConditionAst<'_> {
    /// Parses a complete conditional statement with if, else-if, and else clauses
    /// 
    /// This is the main parser for conditional statements in Timu. It handles the
    /// full conditional syntax including the initial if condition, any number of
    /// else-if clauses, and an optional final else clause. Each clause consists
    /// of a condition expression (except the final else) and a code block body.
    /// 
    /// # Parsing Logic
    /// 1. Parse `if` keyword and condition expression
    /// 2. Parse the true branch code block
    /// 3. Parse zero or more `else if` clauses (condition + body)
    /// 4. Optionally parse final `else` clause (body only)
    /// 5. Build complete conditional AST
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, if_ast))` - Successfully parsed conditional statement
    /// * `Err(error)` - Parse error with context information
    /// 
    /// # Syntax Examples
    /// ```timu
    /// if (condition) { statements }
    /// 
    /// if (condition) {
    ///     statements
    /// } else {
    ///     other_statements  
    /// }
    /// 
    /// if (condition1) {
    ///     branch1
    /// } else if (condition2) {
    ///     branch2
    /// } else {
    ///     default_branch
    /// }
    /// ```
    /// 
    /// # Errors
    /// Returns errors for:
    /// - Missing condition expression after `if`
    /// - Missing code block for if branch
    /// - Missing condition expression after `else if`
    /// - Missing code block for any branch
    /// - Invalid expression syntax in conditions
    /// 
    /// # Expression Types
    /// Condition expressions can be:
    /// - Boolean literals: `true`, `false`
    /// - Comparison expressions: `a == b`, `x > 10`
    /// - Logical expressions: `flag && other_flag`
    /// - Function calls: `isValid(input)`
    /// - Variable references: `condition_flag`
    /// - Complex expressions: `(a > b) && (c == d)`
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, IfConditionAst<'_>, TimuParserError<'_>> {
        let (input, _) = cleanup(tag("if")).parse(input)?;
        let (input, expression) = context("Missing if expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, true_body) = context("Missing true body", cut(BodyAst::parse)).parse(input)?;

        let (input, else_ifs) = many0(
            preceded(
                (cleanup(tag("else")), cleanup(tag("if"))), 
                (context("Missing 'else if' expression", cut(ExpressionAst::parse)), context("Missing 'else if' body", cut(BodyAst::parse)))
            )
        ).parse(input)?;
        let (input, false_body) = match cleanup(opt(tag("else"))).parse(input)? {
            (input, Some(_)) => {
                let (input, false_body) = context("Missing false body", cut(BodyAst::parse)).parse(input)?;
                (input, Some(false_body))
            },
            (input, None) => (input, None),
        };

        Ok((
            input,
            IfConditionAst {
                expression,
                true_body,
                else_ifs,
                false_body,
            },
        ))
    }

    /// Parses a conditional statement for use within code blocks
    /// 
    /// This parser variant wraps the main conditional statement parser for
    /// integration with the body statement parsing system. Conditional statements
    /// are valid statements that can appear within function bodies, other
    /// conditional blocks, and any other code block context.
    /// 
    /// # Arguments
    /// * `input` - The input span to parse from
    /// 
    /// # Returns
    /// * `Ok((remaining, statement))` - Successfully parsed conditional statement
    /// * `Err(error)` - Parse error from the underlying conditional parser
    /// 
    /// # Usage Context
    /// This function is called by the body parser when processing statements
    /// within code blocks. It enables conditional statements to be used as
    /// first-class statements alongside variable declarations, assignments,
    /// and function calls.
    pub fn parse_body_statement(input: NomSpan<'_>) -> IResult<NomSpan<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, if_condition) = Self::parse(input)?;
        Ok((input, BodyStatementAst::IfCondition(if_condition)))
    }
}

impl Display for IfConditionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.expression, self.true_body)?;
        for (expression, body) in self.else_ifs.iter() {
            write!(f, " else if {expression} {body}")?;
        }
        if let Some(false_body) = &self.false_body {
            write!(f, " else {false_body}")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{
        ast::IfConditionAst,
        file::SourceFile, nom_tools::State,
    };

    use super::NomSpan;

    #[rstest]
    #[case(r#"if true {}"#, r#"if true {}"#)]
    #[case(r#"if true {if true {if true {if true {}}}}"#, r#"if true {if true {if true {if true {}}}}"#)]
    #[case(r#"if call(true) {}"#, r#"if call(true) {}"#)]
    #[case(r#"if call(true) { var a = 1;var a = 1;var a = 1;}"#, r#"if call(true) {var a = 1; var a = 1; var a = 1;}"#)]
    #[case(r#"if call(true) + 20 == 100 {}"#, r#"if ((call(true) + 20) == 100) {}"#)]
    #[case(r#"if true || false {}"#, r#"if (true || false) {}"#)]
    #[case(r#"if true || false {} else {}"#, r#"if (true || false) {} else {}"#)]
    #[case(r#"if true || false {var a = 20;} else {var b = 30;}"#, r#"if (true || false) {var a = 20;} else {var b = 30;}"#)]
    #[case(r#"if true || false {}
else if false {}
"#, r#"if (true || false) {} else if false {}"#)]
    #[case(r#"if true || false {}
    else if false {}
    else if false {}
    else if false {}
    else {}
    "#, r#"if (true || false) {} else if false {} else if false {} else if false {} else {}"#)]
    fn if_condition_test<'base>(#[case] code: &'base str, #[case] expected: &'base str) {
        let source_file = SourceFile::new(vec!["<memory>".into()], code.to_string());

        let state = State {
            file: source_file.clone(),
            indexer: Default::default(),
        };

        let input = NomSpan::new_extra(source_file.code().as_str(), state);
        let (_, response) = IfConditionAst::parse(input).unwrap();
        assert_eq!(response.to_string(), expected, "{}", code);
    }
}
