use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use nom::Mode;
use nom::branch::alt;
use nom::bytes::complete::{take_till, take_until};
use nom::character::complete::{alphanumeric1, char};
use nom::combinator::{all_consuming, complete, consumed, cut, map, opt, peek, recognize};
use nom::error::ErrorKind;
use nom::multi::{many0, separated_list0};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{Err, Finish, OutputMode, PResult};
use nom::{IResult, Parser, character::complete::multispace0, error::ParseError, sequence::delimited};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};
use nom_language::error::VerboseError;

use crate::ast::{
    BodyAst, ClassDefinitionAst, ClassDefinitionFieldAst, FieldAst, FunctionArgumentAst, FunctionDefinitionAst, FileAst, FileStatementAst, TypeNameAst,
};
use crate::file::SourceFile;
use crate::nom_tools::{CustomErrorContext, Span, State, cleanup, expected};
use nom_locate::{LocatedSpan, position};

pub fn comment<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    preceded(char('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),))).parse(input)
}

pub fn parse<'a>(state: State<'a>) -> IResult<Span<'a>, FileAst<'a>, VerboseError<Span<'a>>> {
    let input = Span::new_extra(state.file.code(), state);
    let (remaining, statements) = many0(alt((cleanup(ClassDefinitionAst::parse), cleanup(FunctionDefinitionAst::parse_file_function)))).parse(input)?;

    if remaining.len() > 0 {
        /*
        let error = Error(remaining.to_range(), "Unparsed input".to_string());
        let (_, data) = cleanup(alphanumeric1).parse(remaining.clone())?;
        remaining.extra.report_error(error);
        */
        let (_, data) = cleanup(alphanumeric1).parse(remaining.clone())?;
        let error = VerboseError::add_error(remaining.clone(), "Syntax issue", VerboseError::from_error_kind(remaining, ErrorKind::Eof));
        return Err(Err::Failure(error));
    }

    Ok((
        remaining,
        FileAst {
            statements,
        },
    ))
}

impl Display for FileAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Display for FileStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatementAst::ClassDefinition(class) => write!(f, "{}", class),
            FileStatementAst::FunctionDefinition(function) => write!(f, "{}", function),
        }
    }
}

impl ClassDefinitionAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, FileStatementAst<'a>, E> {
        let (input, _) = cleanup(tag("class")).parse(input)?;
        let (input, name) =
            expected("Missing class name", cleanup(alt((take_till(|c| c == '{' || c == ' ' || c == '\t' || c == '\r' || c == '\n'), take_until("{")))))
                .parse(input)?;
        let (input, _) = expected("Missing class '{'", peek(cleanup(char('{')))).parse(input)?;
        let (input, fields) = map(
            delimited(
                char('{'),
                cleanup(many0(alt((FieldAst::parse_class_field, FunctionDefinitionAst::parse_class_function)))),
                expected("Missing class '}'", char('}')),
            ),
            |items| items,
        )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::ClassDefinition(ClassDefinitionAst {
                name,
                fields,
            }),
        ))
    }
}

impl Display for ClassDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} {{", self.name.fragment())?;
        for (index, field) in self.fields.iter().enumerate() {
            match field {
                ClassDefinitionFieldAst::ClassField(field) => {
                    write!(f, "{}", field)?;
                }
                ClassDefinitionFieldAst::ClassFunction(function) => {
                    write!(f, "{}", function)?;
                }
            }
        }
        write!(f, "}}")
    }
}

impl TypeNameAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, TypeNameAst<'a>, E> {
        let (input, nullable) = cleanup(opt(char('?'))).parse(input)?;
        let (input, names) = map(separated_list0(char('.'), cleanup(alphanumeric1)), |items| items).parse(input)?;
        Ok((
            input,
            TypeNameAst {
                nullable: nullable.is_some(),
                names,
            },
        ))
    }
}

impl Display for TypeNameAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nullable {
            write!(f, "?")?;
        }

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.fragment())?;
        }
        Ok(())
    }
}

impl FunctionArgumentAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, FunctionArgumentAst<'a>, E> {
        let (input, (name, field_type)) = (cleanup(terminated(alphanumeric1, cleanup(char(':')))), cleanup(TypeNameAst::parse)).parse(input)?;
        Ok((
            input,
            FunctionArgumentAst {
                name,
                field_type,
            },
        ))
    }
}

impl BodyAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, BodyAst<'a>, E> {
        let (input, body) = map(
            delimited(expected("Missing '{'", char('{')), cleanup(separated_list0(char(';'), BodyAst::parse::<E>)), expected("Missing '}'", char('}'))),
            |items| items,
        )
        .parse(input)?;
        Ok((
            input,
            BodyAst {
                statements: vec![],
            },
        ))
    }
}

impl Display for BodyAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, statement) in self.statements.iter().enumerate() {
            write!(f, "{}", statement)?;
            if index < self.statements.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl Display for FunctionArgumentAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.fragment(), self.field_type)
    }
}

impl FunctionDefinitionAst<'_> {
    pub fn parse_file_function<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, FileStatementAst<'a>, E> {
        let (input, function) = Self::parse(input)?;

        Ok((input, FileStatementAst::FunctionDefinition(function)))
    }

    pub fn parse_class_function<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, E> {
        let (input, function) = Self::parse(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassFunction(function)))
    }

    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, FunctionDefinitionAst<'a>, E> {
        let (input, is_public) = cleanup(opt(tag("pub"))).parse(input)?;
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) =
            expected("Missing function name", cleanup(alt((take_till(|c| c == '(' || c == ' ' || c == '\t' || c == '\r' || c == '\n'), take_until("(")))))
                .parse(input)?;
        let (input, _) = expected("Missing '('", peek(cleanup(char('(')))).parse(input)?;
        let (input, arguments_fields) =
            map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse::<E>)), expected("Missing ')'", char(')'))), |items| items)
                .parse(input)?;

        let (input, _) = expected("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = expected("Missing function return type", cleanup(cleanup(TypeNameAst::parse))).parse(input)?;

        let (input, body) = BodyAst::parse::<E>.parse(input)?;
        let mut arguments = Vec::new();
        for argument in arguments_fields.into_iter() {
            arguments.push(argument);
        }

        Ok((
            input,
            FunctionDefinitionAst {
                is_public: is_public.is_some(),
                name,
                arguments,
                body,
                return_type,
            },
        ))
    }
}

impl Display for FunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}func {}(", if self.is_public { "pub " } else { "" }, self.name.fragment())?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {} {}", self.return_type, self.body)
    }
}

impl Display for ClassDefinitionFieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDefinitionFieldAst::ClassField(field) => write!(f, "{}", field),
            ClassDefinitionFieldAst::ClassFunction(function) => write!(f, "{}", function),
        }
    }
}

impl FieldAst<'_> {
    pub fn parse_field<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(input: Span<'a>) -> IResult<Span<'a>, FieldAst<'a>, E> {
        let (input, (is_public, name, field_type, _)) =
            (cleanup(opt(tag("pub"))), cleanup(terminated(alphanumeric1, cleanup(char(':')))), cleanup(TypeNameAst::parse), cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            FieldAst {
                is_public: is_public.is_some(),
                name,
                field_type,
            },
        ))
    }

    pub fn parse_class_field<'a, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, E> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassField(field)))
    }
}

impl Display for FieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {};",
            match self.is_public {
                true => "pub ",
                false => "",
            },
            self.name.fragment(),
            self.field_type
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, vec};

    use nom_language::error::VerboseError;
    use rstest::rstest;

    use crate::{file::SourceFile, nom_parser::State};

    use super::{Span, TypeNameAst};

    #[rstest]
    #[case("string", false, vec!["string"])]
    #[case(" string ", false, vec!["string"])]
    #[case("string.base", false, vec!["string", "base"])]
    #[case("string.base . test", false, vec!["string", "base", "test"])]
    #[case(" string   .        base        . test", false, vec!["string", "base", "test"])]
    #[case(" ? string   .        base        . test", true, vec!["string", "base", "test"])]
    #[case("?string", true, vec!["string"])]
    fn parse_type_name_test<'a>(#[case] code: &'a str, #[case] nullable: bool, #[case] expected: Vec<&str>) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));
        let errors = Rc::new(RefCell::new(vec![]));

        let mut state = State {
            errors: errors.clone(),
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let result = TypeNameAst::parse::<VerboseError<Span>>(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.nullable, nullable, "nullable info does not match expected");

        let parsed: Vec<_> = parsed.names.into_iter().map(|s| s.fragment().to_string()).collect();
        assert_eq!(parsed, expected, "Parsed type name does not match expected");
    }
}
