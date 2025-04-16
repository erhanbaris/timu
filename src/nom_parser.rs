use std::cell::RefCell;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::{take_till, take_until};
use nom::character::complete::{alphanumeric1, char};
use nom::combinator::{all_consuming, complete, consumed, cut, map, opt, recognize};
use nom::error::{ContextError, context};
use nom::multi::{many0, separated_list0};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{Err, Finish, OutputMode, PResult};
use nom::{IResult, Parser, character::complete::multispace0, error::ParseError, sequence::delimited};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};
use nom_language::error::VerboseError;

use crate::file::SourceFile;
use crate::span::Spanned;
use nom_locate::{LocatedSpan, position};

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub errors: &'a RefCell<Vec<Error>>,
    pub file: Rc<SourceFile<'a>>,
}

impl<'a> State<'a> {
    pub fn report_error(&self, error: Error) {
        self.errors.borrow_mut().push(error);
    }
}

pub type Span<'a> = LocatedSpan<&'a str, State<'a>>;

fn cleanup_whitespaces(c: char) -> bool {
    (c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

fn cleanup<'a, O, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>, F: Parser<Span<'a>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<Span<'a>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}

pub fn expected2<'a, O, F: Parser<Span<'a>, Output = O, Error = nom::error::Error<Span<'a>>>>(
    mut f: F, m: &str,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, nom::error::Error<Span<'a>>> {
    move |input: Span<'a>| {
        match f.parse(input) {
            Ok((remaining, out)) => Ok((remaining, out)),
            Err(nom::Err::Error(input)) | Err(nom::Err::Failure(input)) => {
                let err = Error(input.input.to_range(), m.to_string());
                input.input.extra.report_error(err); // Push error onto stack.
                Err(nom::Err::Failure(input))
            }
            Err(err) => Err(err),
        }
    }
}

pub fn comment<'a, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    preceded(char('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),))).parse(input)
}

pub fn parse_type_name<'a, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, TypeName<'a>, E> {
    let (input, nullable) = cleanup(opt(char('?'))).parse(input)?;
    let (input, names) = map(separated_list0(char('.'), cleanup(alphanumeric1)), |items| items).parse(input)?;
    Ok((
        input,
        TypeName {
            nullable: nullable.is_some(),
            names,
        },
    ))
}

pub fn parse_field<'a, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, ClassField<'a>, E> {
    let (input, (is_public, name, field_type)) =
        (cleanup(opt(tag("pub"))), cleanup(terminated(alphanumeric1, cleanup(char(':')))), cleanup(parse_type_name)).parse(input)?;

    Ok((
        input,
        ClassField {
            is_public: is_public.is_some(),
            name,
            field_type,
        },
    ))
}

#[derive(Debug)]
pub struct Error(std::ops::Range<usize>, String);

trait ToRange {
    fn to_range(&self) -> std::ops::Range<usize>;
}

impl<'a> ToRange for Span<'a> {
    fn to_range(&self) -> std::ops::Range<usize> {
        let start = self.location_offset();
        let end = start + self.fragment().len();
        start..end
    }
}

pub fn parse_class<'a, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, TimuFileStatementAst<'a>, E> {
    let (input, (_, name, fields)) = (
        cleanup(tag("class")),
        cleanup(alt((take_till(cleanup_whitespaces), take_until("{")))),
        map(delimited(char('{'), cleanup(separated_list0(char(','), parse_field::<E>)), char('}')), |items| items),
    )
        .parse(input.clone())?;

    let mut values = Vec::new();
    for field in fields.into_iter() {
        values.push(ClassDefinitionField::ClassField(field));
    }

    Ok((
        input,
        TimuFileStatementAst::ClassDefinition(ClassDefinition {
            name,
            values,
        }),
    ))
}

pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + ContextError<Span<'a>>>(state: State<'a>) -> IResult<Span<'a>, Vec<TimuFileStatementAst<'a>>, E> {
    let input = Span::new_extra(state.file.code(), state);
    let parse_result = all_consuming(many0(alt((parse_class,)))).parse(input);

    /*let (input, items) = match parse_result {
        Ok((input, items)) => (input, items),
        Err(error) => {
            let error = error.map(|error| {
                println!("Error: {:?}", &error);
                let (_, data) = cleanup(alphanumeric1::<Span, ()>).parse(error.input).unwrap();
                data
            });
            return Err(error);
        }
    };*/
    parse_result
}

#[derive(Debug)]
pub struct TypeName<'a> {
    pub nullable: bool,
    pub names: Vec<Span<'a>>,
}

#[derive(Debug)]
pub enum TimuFileStatementAst<'a> {
    ClassDefinition(ClassDefinition<'a>),
}

#[derive(Debug)]
pub struct ClassDefinition<'a> {
    name: Span<'a>,
    values: Vec<ClassDefinitionField<'a>>,
}

#[derive(Debug)]
pub enum ClassDefinitionField<'a> {
    ClassField(ClassField<'a>),
}

#[derive(Debug)]
pub struct ClassField<'a> {
    is_public: bool,
    name: Span<'a>,
    field_type: TypeName<'a>,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rstest::rstest;

    use super::{Span, parse_type_name};

    #[rstest]
    #[case("string", false, vec!["string"])]
    #[case(" string ", false, vec!["string"])]
    #[case("string.base", false, vec!["string", "base"])]
    #[case("string.base . test", false, vec!["string", "base", "test"])]
    #[case(" string   .        base        . test", false, vec!["string", "base", "test"])]
    #[case(" ? string   .        base        . test", true, vec!["string", "base", "test"])]
    #[case("?string", true, vec!["string"])]
    fn parse_type_name_test<'a>(#[case] code: &'a str, #[case] nullable: bool, #[case] expected: Vec<&str>) {
        let input = Span::new(code);
        let result = parse_type_name(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.nullable, nullable, "nullable info does not match expected");

        let parsed: Vec<_> = parsed.names.into_iter().map(|s| s.fragment().to_string()).collect();
        assert_eq!(parsed, expected, "Parsed type name does not match expected");
    }
}
