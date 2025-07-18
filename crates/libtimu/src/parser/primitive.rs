//! Primitive value parsing for the Timu language.
//!
//! This module handles parsing of all primitive data types in Timu including:
//!
//! - **Strings**: Double-quoted strings with escape sequence support
//! - **Numbers**: Integers and floating-point numbers with automatic type inference
//! - **Booleans**: `true` and `false` literals
//!
//! # String Parsing
//!
//! Strings are enclosed in double quotes and support the following escape sequences:
//! - `\n` - Newline
//! - `\r` - Carriage return  
//! - `\t` - Tab
//! - `\\` - Backslash
//! - `\"` - Double quote
//! - `\/` - Forward slash
//!
//! # Number Parsing
//!
//! Numbers are parsed with automatic type inference based on value ranges:
//! - **Integers**: Automatically choose the smallest fitting type (i8, u8, i16, u16, i32, u32, i64, u64)
//! - **Floating-point**: Support for float and double precision with scientific notation
//! - **Underscores**: Allowed as digit separators (e.g., `1_000_000`)
//! - **Scientific notation**: Supported for floating-point numbers (e.g., `1.23e-4`)
//!
//! The parser intelligently selects the most appropriate numeric type based on the
//! value's range, preferring signed types when possible and choosing the smallest
//! type that can represent the value.

use std::fmt::{Display, Formatter};

use nom::Err;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of, one_of};
use nom::combinator::{consumed, opt, recognize, value};
use nom::multi::{fold, many0, many1};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser, sequence::delimited};
use nom_language::error::VerboseErrorKind;

use crate::ast::{ExpressionAst, PrimitiveValue};
use crate::nom_tools::{cleanup, Between, NomSpan};

use super::TimuParserError;

// Numeric type range constants for automatic type inference

/// Range for 8-bit signed integers
static I8_RANGE: std::ops::Range<i128> = (i8::MIN as i128)..((i8::MAX as i128) + 1);
/// Range for 8-bit unsigned integers
static U8_RANGE: std::ops::Range<i128> = (u8::MIN as i128)..((u8::MAX as i128) + 1);

/// Range for 16-bit signed integers
static I16_RANGE: std::ops::Range<i128> = (i16::MIN as i128)..((i16::MAX as i128) + 1);
/// Range for 16-bit unsigned integers
static U16_RANGE: std::ops::Range<i128> = (u16::MIN as i128)..((u16::MAX as i128) + 1);

/// Range for 32-bit signed integers
static I32_RANGE: std::ops::Range<i128> = (i32::MIN as i128)..((i32::MAX as i128) + 1);
/// Range for 32-bit unsigned integers
static U32_RANGE: std::ops::Range<i128> = (u32::MIN as i128)..((u32::MAX as i128) + 1);

/// Range for 64-bit signed integers
static I64_RANGE: std::ops::Range<i128> = (i64::MIN as i128)..((i64::MAX as i128) + 1);
/// Range for 64-bit unsigned integers
static U64_RANGE: std::ops::Range<i128> = (u64::MIN as i128)..((u64::MAX as i128) + 1);

/// Range for 32-bit floating-point numbers
static FLOAT_RANGE: std::ops::Range<f64> = (f32::MIN as f64)..(f32::MAX as f64);


/// Parses a single character within a string, handling escape sequences
/// 
/// This function handles both regular characters and escape sequences within strings.
/// Supported escape sequences include: `\n`, `\r`, `\t`, `\\`, `\"`, `\/`.
fn character(input: NomSpan<'_>) -> IResult<NomSpan<'_>, char, TimuParserError<'_>> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((value('\n', char('n')), value('\r', char('r')), value('\t', char('t')), value('\\', char('\\')), value('"', char('"')), value('/', char('/'))))
            .parse(input)
    } else {
        Ok((input, c))
    }
}

/// Parses a string literal enclosed in double quotes
/// 
/// Supports escape sequences and builds the final string by processing
/// each character through the `character` parser. The string content
/// is stored as a `Cow<'base, str>` for efficient memory usage when
/// the parsed string can borrow from the source.
/// 
/// # Examples
/// - `"hello"` → `PrimitiveValue::String("hello")`
/// - `"hello\nworld"` → `PrimitiveValue::String("hello\nworld")`
/// - `"path/to/file"` → `PrimitiveValue::String("path/to/file")`
pub fn string(input: NomSpan<'_>) -> IResult<NomSpan<'_>, PrimitiveValue, TimuParserError<'_>> {
    let (input, string) = delimited(
        char('"'),
        fold(0.., character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        char('"'),
    )
    .parse(input)?;

    Ok((input, PrimitiveValue::String(string.into())))
}

/// Parses numeric literals with automatic type inference
/// 
/// This function parses both integer and floating-point numbers, automatically
/// selecting the most appropriate type based on the value's range and format.
/// 
/// # Integer Type Selection
/// For integers, the parser chooses the smallest type that can represent the value:
/// - Positive values: Prefers unsigned types when they fit
/// - Negative values: Uses signed types
/// - Type hierarchy: i8 → u8 → i16 → u16 → i32 → u32 → i64 → u64
/// 
/// # Floating-Point Type Selection
/// For decimal numbers:
/// - Values within float range → `PrimitiveValue::Float`
/// - Values requiring double precision → `PrimitiveValue::Double`
/// 
/// # Supported Formats
/// - Integers: `123`, `-456`, `1_000_000`
/// - Floats: `3.14`, `-2.5`, `1.23e-4`, `6.02e+23`
/// - Scientific notation: Supports `e`/`E` with optional `+`/`-`
/// - Digit separators: Underscores allowed anywhere in the number
/// 
/// # Examples
/// - `42` → `PrimitiveValue::I8(42)`
/// - `300` → `PrimitiveValue::U16(300)`
/// - `3.14` → `PrimitiveValue::Float(3.14, 2)`
/// - `1.23e-4` → `PrimitiveValue::Float(0.000123, 1)`
pub fn number<'base>(input: NomSpan<'base>) -> IResult<NomSpan<'base>, PrimitiveValue<'base>, TimuParserError<'base>> {
    let (input, (representing, (number, floating))) = (
        opt(one_of("+-")),
        (
            recognize::<NomSpan<'base>, TimuParserError<'base>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            opt(preceded(
                char('.'),
                (
                    recognize::<NomSpan<'base>, TimuParserError<'base>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
                    opt(preceded(
                        one_of("Ee"),
                        (
                            opt(alt((value(true, char('-')), value(false, char('+'))))),
                            recognize::<NomSpan<'base>, TimuParserError<'base>, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
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
            true => PrimitiveValue::Float(number, dot_place as u8),
            false => PrimitiveValue::Double(number, dot_place as u8) 
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
            PrimitiveValue::I8(number as i8)
        } else if U8_RANGE.between(number) {
            PrimitiveValue::U8(number as u8)
        } else if I16_RANGE.between(number) {
            PrimitiveValue::I16(number as i16)
        } else if U16_RANGE.between(number) {
            PrimitiveValue::U16(number as u16)
        } else if I32_RANGE.between(number) {
            PrimitiveValue::I32(number as i32)
        } else if U32_RANGE.between(number) {
            PrimitiveValue::U32(number as u32)
        } else if I64_RANGE.between(number) {
            PrimitiveValue::I64(number as i64)
        } else if U64_RANGE.between(number) {
            PrimitiveValue::U64(number as u64)
        } else {
            return Err(Err::Failure(TimuParserError {
                errors: vec![(input, VerboseErrorKind::Context("Invalid number length"))],
            }));
        }
    };

    Ok((input, number))
}

impl PrimitiveValue<'_> {
    pub fn parse(input: NomSpan<'_>) -> IResult<NomSpan<'_>, (NomSpan<'_>, PrimitiveValue), TimuParserError<'_>> {
        let (input, value) =
            consumed(cleanup(alt((
                number, 
                string, 
                value(PrimitiveValue::Bool(true), tag("true")), 
                value(PrimitiveValue::Bool(false), tag("false"))
            )))).parse(input)?;

        Ok((input, value))
    }

    pub fn parse_for_expression(input: NomSpan<'_>) -> IResult<NomSpan<'_>, ExpressionAst<'_>, TimuParserError<'_>> {
        let (input, (span, value)) = Self::parse(input)?;
        Ok((
            input,
            ExpressionAst::Primitive { span: span.into(), value },
        ))
    }
}

impl Display for PrimitiveValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValue::String(value) => write!(f, "{value}"),
            PrimitiveValue::Bool(value) => write!(f, "{value}"),
            PrimitiveValue::I8(value) => write!(f, "{value}"),
            PrimitiveValue::U8(value) => write!(f, "{value}"),
            PrimitiveValue::I16(value) => write!(f, "{value}"),
            PrimitiveValue::U16(value) => write!(f, "{value}"),
            PrimitiveValue::I32(value) => write!(f, "{value}"),
            PrimitiveValue::U32(value) => write!(f, "{value}"),
            PrimitiveValue::I64(value) => write!(f, "{value}"),
            PrimitiveValue::U64(value) => write!(f, "{value}"),
            PrimitiveValue::Float(value, len) => write!(f, "{:.*}", *len as usize, value),
            PrimitiveValue::Double(value, len) => write!(f, "{:.*}", *len as usize, value),
        }
    }
}
