use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt};
use nom::error::context;
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{BodyStatementAst, ExpressionAst, TypeNameAst, VariableAssignAst, VariableDefinitionAst, VariableDefinitionType};
use crate::nom_tools::{Span, cleanup};
use crate::parser::{expected_ident, ident};

use super::TimuParserError;

impl VariableDefinitionAst<'_> {
    pub fn parse_body_statement(input: Span<'_>) -> IResult<Span<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableDefinition(variable)))
    }

    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, VariableDefinitionAst<'_>, TimuParserError<'_>> {
        let (input, variable_definition_type) = cleanup(alt((
            map(tag("var"), |_| VariableDefinitionType::Var),
            map(tag("const"), |_| VariableDefinitionType::Const))
        )).parse(input)?;
        let (input, name) = expected_ident("Missing variable name", input)?;
        
        let (input, expected_type, expression) = match cleanup(opt(char(':'))).parse(input)? {
            (input, Some(_)) => {
                let (input, expected_type) = context("Missing variable type", cut(cleanup(TypeNameAst::parse))).parse(input)?;
                
                // If the type is nullable, no need for expression, but if there is a equal sign, expression is required
                let (input, expression) = match cleanup(opt(char('='))).parse(input)? {
                    (input, Some(_)) => {
                        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
                        (input, Some(expression))
                    },
                    (input, None) => (input, None),
                };

                if variable_definition_type == VariableDefinitionType::Const && expression.is_none() {
                    return Err(nom::Err::Failure(VerboseError {
                        errors: vec![(input, VerboseErrorKind::Context("Const variable must have an assignment"))],
                    }));
                }
                
                (input, Some(expected_type), expression)
            }

            // If there is no type, we must have an expression
            (input, None) => {
                let (input, _) = context("Missing '='", cleanup(cut(char('=')))).parse(input)?;
                let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
                (input, None, Some(expression))
            }
        };

        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableDefinitionAst {
                variable_definition_type,
                name,
                expected_type,
                expression,
            },
        ))
    }
}

impl Display for VariableDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.variable_definition_type, self.name.fragment())?;
        if let Some(expected_type) = &self.expected_type {
            write!(f, ": {}", expected_type)?;
        }
        if let Some(expression) = &self.expression {
            write!(f, " = {}", expression)?;
        }
        write!(f, ";")
    }
}

impl VariableAssignAst<'_> {
    pub fn parse_body_statement(input: Span<'_>) -> IResult<Span<'_>, BodyStatementAst<'_>, TimuParserError<'_>> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::VariableAssign(variable)))
    }

    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, VariableAssignAst<'_>, TimuParserError<'_>> {
        let (input, name) = ident().parse(input)?;
        let (input, _) = context("Missing '='", cleanup(char('='))).parse(input)?;
        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableAssignAst {
                name,
                expression,
            },
        ))
    }
}

impl Display for VariableAssignAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {};", self.name.fragment(), self.expression)
    }
}

impl Display for VariableDefinitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableDefinitionType::Var => write!(f, "var"),
            VariableDefinitionType::Const => write!(f, "const"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use nom_language::error::VerboseErrorKind;
    use rstest::rstest;

    use crate::{ast::VariableDefinitionAst, file::SourceFile, nom_tools::{Span, State}};
    
    #[rstest]
    #[case("var a = false;", "var a = false;")]
    #[case("var a = 100;", "var a = 100;")]
    #[case("var a: ?i32;", "var a: ?i32;")]
    #[case("var a = 100;", "var a = 100;")]
    #[case("var _a = 200;", "var _a = 200;")]
    #[case("const a = 100;", "const a = 100;")]
    #[case("const _123a = 100;", "const _123a = 100;")]
    #[case("const a_123 = 100;", "const a_123 = 100;")]
    #[case("const a_123_____ = 100;", "const a_123_____ = 100;")]
    #[case("const a = 1.0;", "const a = 1.0;")]
    #[case("const a = 1.2;", "const a = 1.2;")]
    #[case("const a = -1.2;", "const a = -1.2;")]
    #[case("const a: f64 = -1.2;", "const a: f64 = -1.2;")]
    #[case("const a: ?f64 = -1.2;", "const a: ?f64 = -1.2;")]
    fn custom_variable_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let result = VariableDefinitionAst::parse(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.to_string(), expected);
    }

    #[rstest]
    #[case("var a;", "Missing '='")]
    #[case("var a: ;", "Missing variable type")]
    #[case("const a: ?i32;", "Const variable must have an assignment")]
    fn invalid_variable_test<'a>(#[case] code: &'a str, #[case] expected: &'a str) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let result = VariableDefinitionAst::parse(input);
        let error = result.unwrap_err();

        if let nom::Err::Failure(error) = error {
            if let VerboseErrorKind::Context(ctx) = error.errors[error.errors.len() - 1].1 {
                assert_eq!(ctx, expected, "{}", code);
            } else {
                panic!("Expected an error, but got: {:#?}", error);
            }
        } else {
            panic!("Expected an error, but got: {:#?}", error);
        }
    }
}