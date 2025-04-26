use std::fmt::{Display, Formatter};

use nom::Err;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of, one_of};
use nom::combinator::{opt, recognize, value};
use nom::multi::{fold, many0, many1};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser, sequence::delimited};
use nom_language::error::VerboseErrorKind;

use crate::ast::PrimitiveType;
use crate::nom_tools::{cleanup, Between, Span};

use super::TimuParserError;

static I8_RANGE: std::ops::Range<i128> = (i8::MIN as i128)..(i8::MAX as i128);
static U8_RANGE: std::ops::Range<i128> = (u8::MIN as i128)..(u8::MAX as i128);

static I16_RANGE: std::ops::Range<i128> = (i16::MIN as i128)..(i16::MAX as i128);
static U16_RANGE: std::ops::Range<i128> = (u16::MIN as i128)..(u16::MAX as i128);

static I32_RANGE: std::ops::Range<i128> = (i32::MIN as i128)..(i32::MAX as i128);
static U32_RANGE: std::ops::Range<i128> = (u32::MIN as i128)..(u32::MAX as i128);

static I64_RANGE: std::ops::Range<i128> = (i64::MIN as i128)..(i64::MAX as i128);
static U64_RANGE: std::ops::Range<i128> = (u64::MIN as i128)..(u64::MAX as i128);

static FLOAT_RANGE: std::ops::Range<f64> = (f32::MIN as f64)..(f32::MAX as f64);


fn character(input: Span<'_>) -> IResult<Span<'_>, char, TimuParserError<'_>> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((value('\n', char('n')), value('\r', char('r')), value('\t', char('t')), value('\\', char('\\')), value('"', char('"')), value('/', char('/'))))
            .parse(input)
    } else {
        Ok((input, c))
    }
}

pub fn string(input: Span<'_>) -> IResult<Span<'_>, PrimitiveType, TimuParserError<'_>> {
    let (input, string) = delimited(
        char('"'),
        fold(0.., character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        char('"'),
    )
    .parse(input)?;

    Ok((input, PrimitiveType::String(string)))
}

pub fn number<'a>(input: Span<'a>) -> IResult<Span<'a>, PrimitiveType, TimuParserError<'a>> {
    let (input, (representing, (number, floating))) = (
        opt(one_of("+-")),
        (
            recognize::<Span<'a>, TimuParserError<'a>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            opt(preceded(
                char('.'),
                (
                    recognize::<Span<'a>, TimuParserError<'a>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
                    opt(preceded(
                        one_of("Ee"),
                        (
                            opt(alt((value(true, char('-')), value(false, char('+'))))),
                            recognize::<Span<'a>, TimuParserError<'a>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
                        ),
                    )),
                ),
            )),
        ),
    )
        .parse(input)?;

    let number = number.replace("_", "");

    let number = if let Some((floating, e_info)) = floating {
        let dot_place = floating.len();
        let floating = floating.replace("_", "");

        let number = if let Some((is_minus, exponent)) = e_info {
            let mut exponent = exponent.replace("_", "").parse::<i32>().unwrap_or(0);
            if let Some(true) = is_minus {
                exponent = -exponent
            };

            let number: f64 = minimal_lexical::parse_float(number.as_bytes().iter(), floating.as_bytes().iter(), exponent);
            number
        } else {
            minimal_lexical::parse_float(number.as_bytes().iter(), floating.as_bytes().iter(), 0)
        };

        let number = match representing {
            Some('-') => -number,
            _ => number,
        };

        match FLOAT_RANGE.between(number) {
            true => PrimitiveType::Float(number, dot_place as u8),
            false => PrimitiveType::Double(number, dot_place as u8) 
        }
    } else {
        let number = match number.replace("_", "").parse::<i128>() {
            Ok(number) => number,
            Err(_) => {
                return Err(Err::Failure(TimuParserError {
                    errors: vec![(input, VerboseErrorKind::Context("Invalid number length"))],
                }));
            }
        };

        let number = match representing {
            Some('-') => -number,
            _ => number,
        };

        if I8_RANGE.between(number) {
            PrimitiveType::I8(number as i8)
        } else if U8_RANGE.between(number) {
            PrimitiveType::U8(number as u8)
        } else if I16_RANGE.between(number) {
            PrimitiveType::I16(number as i16)
        } else if U16_RANGE.between(number) {
            PrimitiveType::U16(number as u16)
        } else if I32_RANGE.between(number) {
            PrimitiveType::I32(number as i32)
        } else if U32_RANGE.between(number) {
            PrimitiveType::U32(number as u32)
        } else if I64_RANGE.between(number) {
            PrimitiveType::I64(number as i64)
        } else if U64_RANGE.between(number) {
            PrimitiveType::U64(number as u64)
        } else {
            return Err(Err::Failure(TimuParserError {
                errors: vec![(input, VerboseErrorKind::Context("Invalid number length"))],
            }));
        }
    };

    Ok((input, number))
}

impl PrimitiveType {
    pub fn parse(input: Span<'_>) -> IResult<Span<'_>, PrimitiveType, TimuParserError<'_>> {
        let (input, value) =
            cleanup(alt((
                number, 
                string, 
                value(PrimitiveType::Bool(true), tag("true")), 
                value(PrimitiveType::Bool(false), tag("false"))
            ))).parse(input)?;

        Ok((input, value))
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::String(value) => write!(f, "{}", value),
            PrimitiveType::Bool(value) => write!(f, "{}", value),
            PrimitiveType::I8(value) => write!(f, "{}", value),
            PrimitiveType::U8(value) => write!(f, "{}", value),
            PrimitiveType::I16(value) => write!(f, "{}", value),
            PrimitiveType::U16(value) => write!(f, "{}", value),
            PrimitiveType::I32(value) => write!(f, "{}", value),
            PrimitiveType::U32(value) => write!(f, "{}", value),
            PrimitiveType::I64(value) => write!(f, "{}", value),
            PrimitiveType::U64(value) => write!(f, "{}", value),
            PrimitiveType::Float(value, len) => write!(f, "{:.*}", *len as usize, value),
            PrimitiveType::Double(value, len) => write!(f, "{:.*}", *len as usize, value),
        }
    }
}
