use nom::character::complete::multispace0;
use nom::{Parser, error::ParseError, sequence::delimited};
use nom_locate::LocatedSpan;
use std::rc::Rc;

use crate::file::SourceFile;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub file: Rc<SourceFile<'a>>,
}

pub type Span<'a> = LocatedSpan<&'a str, State<'a>>;

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

pub fn cleanup<'a, O, E: std::fmt::Debug + ParseError<Span<'a>>, F: Parser<Span<'a>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<Span<'a>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}
