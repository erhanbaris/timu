use std::{cell::RefCell, rc::Rc};

use nom::bits::complete::tag;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::recognize;
use nom::multi::{many0, many0_count};
use nom::sequence::pair;
use nom::{Err, OutputMode, PResult, Parser, error::ParseError, sequence::delimited};
use nom::{IResult, Mode};
use nom_locate::LocatedSpan;

use crate::file::SourceFile;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub file: Rc<SourceFile<'a>>,
}

impl<'a> State<'a> {
}

pub type Span<'a> = LocatedSpan<&'a str, State<'a>>;

pub trait ToRange {
    fn to_range(&self) -> std::ops::Range<usize>;
}

impl<'a> ToRange for Span<'a> {
    fn to_range(&self) -> std::ops::Range<usize> {
        let start = self.location_offset();
        let end = match nom::character::complete::alphanumeric1::<Span, ()>(self.clone()) {
            Ok((input, data)) => start + data.fragment().len(),
            Err(err) => start + self.fragment().len(),
        };
        start..end
    }
}

pub fn cleanup_whitespaces(c: char) -> bool {
    (c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

pub fn cleanup<'a, O, E: std::fmt::Debug + ParseError<Span<'a>>, F: Parser<Span<'a>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<Span<'a>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}
