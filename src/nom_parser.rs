use nom::Finish;
use nom::branch::alt;
use nom::bytes::complete::{take_till, take_until};
use nom::character::complete::{alphanumeric1, char};
use nom::combinator::{all_consuming, complete, consumed, map, opt};
use nom::multi::{many0, separated_list0};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{IResult, Parser, character::complete::multispace0, error::ParseError, sequence::delimited};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};

use crate::span::Spanned;
use nom_locate::{LocatedSpan, position};

pub type Span<'a> = LocatedSpan<&'a str>;

fn cleanup_whitespaces(c: char) -> bool {
    (c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

fn cleanup<'a, O, E: ParseError<LocatedSpan<&'a str>>, F: Parser<LocatedSpan<&'a str>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<LocatedSpan<&'a str>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}

pub fn parse_type_name<'a>(input: Span<'a>) -> IResult<Span<'a>, TypeName<'a>> {
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

pub fn parse_field(input: Span) -> IResult<Span, ClassField> {
    let (input, (is_public, field_name, field_type)) =
        (cleanup(opt(tag("pub"))), cleanup(terminated(alphanumeric1, cleanup(char(':')))), cleanup(parse_type_name)).parse(input)?;

    Ok((
        input,
        ClassField {
            is_public: is_public.is_some(),
            name: field_name.fragment(),
            field_type,
        },
    ))
}

pub fn parse_class(input: Span) -> IResult<Span, TimuFileStatementAst> {
    let (input, _) = cleanup(tag("class")).parse(input)?;
    let (input, position) = cleanup(position).parse(input)?;
    let (input, name) = cleanup(alt((take_till(cleanup_whitespaces), take_until("{")))).parse(input)?;
    let (input, fields) = map(delimited(char('{'), cleanup(separated_list0(char(','), parse_field)), char('}')), |items| items).parse(input)?;

    let mut values = Vec::new();
    for field in fields.into_iter() {
        values.push(ClassDefinitionField::ClassField(field));
    }

    Ok((
        input,
        TimuFileStatementAst::ClassDefinition(ClassDefinition {
            name: name.fragment(),
            values,
        }),
    ))
}

pub fn parse(input: Span) -> IResult<Span, Vec<TimuFileStatementAst>> {
    let (input, items) = all_consuming(many0(alt((parse_class,)))).parse(input)?;
    Ok((input, items))
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
    name: &'a str,
    values: Vec<ClassDefinitionField<'a>>,
}

#[derive(Debug)]
pub enum ClassDefinitionField<'a> {
    ClassField(ClassField<'a>),
}

#[derive(Debug)]
pub struct ClassField<'a> {
    is_public: bool,
    name: &'a str,
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
