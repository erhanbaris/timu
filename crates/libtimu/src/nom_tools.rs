use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::sequence::{preceded, terminated};
use nom::{Parser, sequence::delimited};
use nom_locate::LocatedSpan;
use std::ops::Range;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::file::SourceFile;
use crate::parser::TimuParserError;

#[derive(Clone, Debug)]
pub struct State {
    pub file: SourceFile,
    pub indexer: Arc<AtomicUsize>
}

impl State {
    pub fn new(file: SourceFile) -> Self {
        Self {
            file,
            indexer: Arc::new(AtomicUsize::new(0)),
        }
    }
}

pub type Span<'base, T = &'base str> = LocatedSpan<T, State>;

#[derive(Clone, Debug)]
pub struct SpanInfo {
    pub position: Range<usize>,
    pub file: SourceFile,
}

impl SpanInfo {
    pub fn new(position: Range<usize>, file: SourceFile) -> Self {
        Self { position, file }
    }
}

impl From<Span<'_>> for SpanInfo {
    fn from(span: Span<'_>) -> Self {
        let position = span.to_range();
        let file = span.extra.file.clone();
        Self { position, file }
    }
}

impl From<&Span<'_>> for SpanInfo {
    fn from(span: &Span<'_>) -> Self {
        let position = span.to_range();
        let file = span.extra.file.clone();
        Self { position, file }
    }
}

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
