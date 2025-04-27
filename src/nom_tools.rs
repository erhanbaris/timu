use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::sequence::{preceded, terminated};
use nom::{Parser, sequence::delimited};
use nom_locate::LocatedSpan;
use std::rc::Rc;

use crate::file::SourceFile;
use crate::parser::TimuParserError;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub file: Rc<SourceFile<'a>>,
}

pub type Span<'a, T = &'a str> = LocatedSpan<T, State<'a>>;

pub trait ToRange {
    fn to_range(&self) -> std::ops::Range<usize>;
}

impl ToRange for Span<'_> {
    fn to_range(&self) -> std::ops::Range<usize> {
        let start = self.location_offset();
        let end = match nom::character::complete::alphanumeric1::<Span, ()>(self.clone()) {
            Ok((_, data)) => start + data.fragment().len(),
            Err(_) => start + self.fragment().len(),
        };
        start..end
    }
}

pub trait Between<T: PartialOrd> {
    fn between(&self, t: T) -> bool;
}

impl<T: PartialOrd> Between<T> for std::ops::Range<T> {
    fn between(&self, t: T) -> bool {
        self.start <= t && t <= self.end
    }
}

pub fn cleanup<'a, O, F: Parser<Span<'a>, Output = O, Error = TimuParserError<'a>>>(f: F) -> impl Parser<Span<'a>, Output = O, Error = TimuParserError<'a>> {
    let _left = preceded(char::<Span<'a>, TimuParserError<'a>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));
    let _right = preceded(char::<Span<'a>, TimuParserError<'a>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));

    delimited(multispace0, f, multispace0)
}
