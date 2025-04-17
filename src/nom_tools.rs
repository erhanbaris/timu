use std::{cell::RefCell, rc::Rc};

use nom::Mode;
use nom::character::complete::multispace0;
use nom::{Err, OutputMode, PResult, Parser, error::ParseError, sequence::delimited};
use nom_locate::LocatedSpan;

use crate::file::SourceFile;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub errors: Rc<RefCell<Vec<Error>>>,
    pub file: Rc<SourceFile<'a>>,
}

impl<'a> State<'a> {
    pub fn report_error(&self, error: Error) {
        self.errors.borrow_mut().push(error);
    }
}

#[derive(Debug)]
pub struct Error(pub std::ops::Range<usize>, pub String);
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

pub fn cleanup<'a, O, E: std::fmt::Debug + ParseError<Span<'a>> + CustomErrorContext<'a>, F: Parser<Span<'a>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<Span<'a>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}

pub fn expected<F>(message: &'static str, parser: F) -> Expected<F> {
    Expected {
        message,
        parser,
    }
}

pub struct Expected<F> {
    message: &'static str,
    parser: F,
}

pub trait CustomErrorContext<'a> {
    fn add_error(input: Span<'a>, ctx: &'static str, other: Self) -> Self;
}

impl<'a, F> Parser<Span<'a>> for Expected<F>
where
    F: Parser<Span<'a>>,
    <F as Parser<Span<'a>>>::Error: CustomErrorContext<'a>,
{
    type Output = <F as Parser<Span<'a>>>::Output;
    type Error = <F as Parser<Span<'a>>>::Error;

    fn process<OM: OutputMode>(&mut self, input: Span<'a>) -> PResult<OM, Span<'a>, Self::Output, Self::Error> {
        match self.parser.process::<OM>(input.clone()) {
            Err(Err::Error(e)) => Err(Err::Error(OM::Error::map(e, |e| <F as Parser<Span<'a>>>::Error::add_error(input, self.message, e)))),
            Err(Err::Failure(e)) => Err(Err::Failure(<F as Parser<Span<'a>>>::Error::add_error(input, self.message, e))),
            x => x,
        }
    }
}
