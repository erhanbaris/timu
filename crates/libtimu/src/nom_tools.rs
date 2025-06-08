use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::sequence::{preceded, terminated};
use nom::{Parser, sequence::delimited};
use nom_locate::LocatedSpan;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Range;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::file::SourceFile;
use crate::parser::TimuParserError;

#[derive(Debug, Clone)]
pub struct State {
    pub file: SourceFile,
    pub indexer: Arc<AtomicUsize>
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Eq for State { }

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
    }
}

impl State {
    pub fn new(file: SourceFile) -> Self {
        Self {
            file,
            indexer: Arc::new(AtomicUsize::new(0)),
        }
    }
}

pub type NomSpan<'base, T = &'base str> = LocatedSpan<T, State>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<'base> {
    pub text: &'base str,
    pub position: Range<usize>,
    pub state: State
}

impl<'base> Display for Span<'base> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text)
    }
}

impl<'base> From<NomSpan<'base>> for Span<'base> {
    fn from(value: NomSpan<'base>) -> Self {
        Span {
            text: value.fragment(),
            position: value.to_range(),
            state: value.extra.clone()
        }
    }
}

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
        let file: SourceFile = span.state.file.clone();
        Self { position, file }
    }
}

impl From<&Span<'_>> for SpanInfo {
    fn from(span: &Span<'_>) -> Self {
        let position = span.to_range();
        let file = span.state.file.clone();
        Self { position, file }
    }
}

pub trait ToRange {
    fn to_range(&self) -> std::ops::Range<usize>;
}

impl ToRange for NomSpan<'_> {
    fn to_range(&self) -> std::ops::Range<usize> {
        let start = self.location_offset();
        let end = match nom::character::complete::alphanumeric1::<NomSpan, ()>(self.clone()) {
            Ok((_, data)) => start + data.fragment().len(),
            Err(_) => start + self.fragment().len(),
        };
        start..end
    }
}

impl ToRange for Span<'_> {
    fn to_range(&self) -> std::ops::Range<usize> {
        self.position.clone()
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

pub fn cleanup<'base, O, F: Parser<NomSpan<'base>, Output = O, Error = TimuParserError<'base>>>(f: F) -> impl Parser<NomSpan<'base>, Output = O, Error = TimuParserError<'base>> {
    let _left = preceded(char::<NomSpan<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));
    let _right = preceded(char::<NomSpan<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));

    delimited(multispace0, f, multispace0)
}
