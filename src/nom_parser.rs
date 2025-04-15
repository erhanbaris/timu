use nom::branch::alt;
use nom::bytes::complete::{take_till, take_until};
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::{IResult, Parser, character::complete::multispace0, error::ParseError, sequence::delimited};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};

use crate::span::Spanned;
use nom_locate::{LocatedSpan, position};

type Span<'a> = LocatedSpan<&'a str>;

struct Token<'a> {
    pub position: Span<'a>,
}

fn cleanup_whitespaces(c: char) -> bool {
    (c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

fn cleanup<'a, O, E: ParseError<LocatedSpan<&'a str>>, F: Parser<LocatedSpan<&'a str>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<LocatedSpan<&'a str>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}

pub fn parse(input: Span) -> IResult<Span, TimuFileStatementAst> {
    let (input, _) = cleanup(tag("type")).parse(input)?;
    let (input, position) = cleanup(position).parse(input)?;
    let (input, name) = cleanup(alt((take_till(cleanup_whitespaces), take_until("{")))).parse(input)?;

    let (input, fields) = map(
        delimited(
            char('{'),
            cleanup(separated_list0(
                char(','),
                (
                    cleanup(opt(tag("pub"))),
                    cleanup(alt((take_till(cleanup_whitespaces), take_until(":")))),
                    cleanup(char(':')),
                    cleanup(opt(char('?'))),
                    cleanup(alt((take_till(cleanup_whitespaces), take_until("}"), take_until(",")))),
                ),
            )),
            char('}'),
        ),
        |items| items,
    )
    .parse(input)?;

    let mut values = Vec::new();
    for (is_public, field_name, _, is_nullable, field_type) in &fields {
        values.push(TypeDefinitionField::TypeDefinition(TypeDefinitionVariable {
            is_public: is_public.is_some(),
            name: field_name.fragment().to_string(),
            is_nullable: is_nullable.is_some(),
            type_name: field_type.fragment().to_string(),
        }));
    }

    println!("input: {}", input.is_empty());

    Ok((
        input,
        TimuFileStatementAst::TypeDefinition(TypeDefinition {
            name: String::from(name.to_string()),
            values,
        }),
    ))
}

#[derive(Debug)]
pub enum TimuFileStatementAst {
    TypeDefinition(TypeDefinition),
}

#[derive(Debug)]
pub struct TypeDefinition {
    name: String,
    values: Vec<TypeDefinitionField>,
}

#[derive(Debug)]
pub enum TypeDefinitionField {
    TypeDefinition(TypeDefinitionVariable),
}

#[derive(Debug)]
pub struct TypeDefinitionVariable {
    is_public: bool,
    name: String,
    is_nullable: bool,
    type_name: String,
}
