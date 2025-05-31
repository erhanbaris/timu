use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::sequence::{preceded, terminated};
use nom::{Parser, sequence::delimited};
use nom_locate::LocatedSpan;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

use crate::file::SourceFile;
use crate::parser::TimuParserError;

#[derive(Clone, Debug)]
pub struct State<'base> {
    pub file: Rc<SourceFile<'base>>,
    pub indexer: Rc<AtomicUsize>
}

pub type Span<'base, T = &'base str> = LocatedSpan<T, State<'base>>;

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

pub fn cleanup<'base, O, F: Parser<Span<'base>, Output = O, Error = TimuParserError<'base>>>(f: F) -> impl Parser<Span<'base>, Output = O, Error = TimuParserError<'base>> {
    let _left = preceded(char::<Span<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));
    let _right = preceded(char::<Span<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));

    delimited(multispace0, f, multispace0)
}
