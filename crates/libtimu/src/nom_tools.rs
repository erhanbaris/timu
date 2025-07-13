//! Nom parser tools and utilities for the Timu language compiler.
//!
//! This module provides specialized types and utilities for parsing Timu source code
//! using the nom parser combinator library. It includes source location tracking,
//! span handling, and helper functions for common parsing patterns.

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

/// Parser state containing source file information and atomic indexing
/// 
/// This struct maintains the context needed during parsing, including the
/// source file being parsed and an atomic counter for generating unique IDs.
#[derive(Debug, Clone)]
pub struct State {
    /// The source file being parsed
    pub file: SourceFile,
    /// Atomic counter for generating unique indices during parsing
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
    /// Creates a new parser state for the given source file
    pub fn new(file: SourceFile) -> Self {
        Self {
            file,
            indexer: Arc::new(AtomicUsize::new(0)),
        }
    }
}

/// Type alias for nom's LocatedSpan with Timu's parser state
pub type NomSpan<'base, T = &'base str> = LocatedSpan<T, State>;

/// A span representing a portion of source code with location information
/// 
/// This struct combines the text content with its position in the source
/// and the parser state, providing complete context for error reporting.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<'base> {
    /// The text content of this span
    pub text: &'base str,
    /// The position range in the source file
    pub position: Range<usize>,
    /// The parser state containing file information
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

/// Information about a source span without lifetime dependencies
/// 
/// This struct provides span information (position and file) without
/// borrowing the source text, making it suitable for error reporting
/// and storage beyond the parser's lifetime.
#[derive(Clone, Debug)]
pub struct SpanInfo {
    /// The position range in the source file
    pub position: Range<usize>,
    /// The source file this span belongs to
    pub file: SourceFile,
}

impl SpanInfo {
    /// Creates new span information with the given position and file
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

/// Trait for converting spans to position ranges
/// 
/// This trait provides a uniform way to extract position ranges from
/// different span types used throughout the parser.
pub trait ToRange {
    /// Converts the span to a position range
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

/// Trait for checking if a value falls within a range
/// 
/// This trait provides a convenient way to check if a value falls
/// within the bounds of a range.
pub trait Between<T: PartialOrd> {
    /// Returns true if the value `t` is within this range (inclusive)
    fn between(&self, t: T) -> bool;
}

impl<T: PartialOrd> Between<T> for std::ops::Range<T> {
    fn between(&self, t: T) -> bool {
        self.start <= t && t <= self.end
    }
}

/// Parser combinator that wraps a parser with whitespace cleanup
/// 
/// This function removes leading and trailing whitespace around the given parser,
/// making it easier to handle tokens that may be surrounded by whitespace.
pub fn cleanup<'base, O, F: Parser<NomSpan<'base>, Output = O, Error = TimuParserError<'base>>>(f: F) -> impl Parser<NomSpan<'base>, Output = O, Error = TimuParserError<'base>> {
    let _left = preceded(char::<NomSpan<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));
    let _right = preceded(char::<NomSpan<'base>, TimuParserError<'base>>('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),)));

    delimited(multispace0, f, multispace0)
}
