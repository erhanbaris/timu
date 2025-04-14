use std::{path::PathBuf, rc::Rc};

use internment::Intern;
use std::ops::{Deref, Range};

use crate::file::SourceFile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    source_file: Rc<SourceFile<'a>>,
    inner_span: pest::Span<'a>,
}

impl<'a> Span<'a> {
    pub fn new(source_file: Rc<SourceFile<'a>>, inner_span: pest::Span<'a>) -> Self {
        Self {
            source_file,
            inner_span,
        }
    }

    pub fn spanned<T>(self, value: T) -> Spanned<'a, T> {
        Spanned::new(value, self.source_file.clone(), self.inner_span)
    }

    pub fn src(&self) -> Rc<SourceFile<'a>> {
        self.source_file.clone()
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.inner_span.start()..self.inner_span.end()
    }

    pub fn start(&self) -> usize {
        self.inner_span.start()
    }
    pub fn end(&self) -> usize {
        self.inner_span.end()
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<'a, T> {
    pub value: T,
    pub span: Span<'a>,
}

impl<'a, T: PartialEq> PartialEq for Spanned<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a, T> Spanned<'a, T> {
    pub fn new(value: T, file: Rc<SourceFile<'a>>, inner_span: pest::Span<'a>) -> Self {
        Self {
            value,
            span: Span::new(file, inner_span),
        }
    }

    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned::new(
            &self.value,
            self.span.source_file.clone(),
            self.span.inner_span.clone(),
        )
    }

    pub fn span_to<V>(&self, to: V) -> Spanned<V> {
        Spanned::new(
            to,
            self.span.source_file.clone(),
            self.span.inner_span.clone(),
        )
    }
}

impl<'a, T> From<T> for Spanned<'a, T> {
    fn from(value: T) -> Self {
        Spanned::new(
            value,
            Rc::new(SourceFile::new(PathBuf::from("<MEMORY>"), "")),
            pest::Span::new("", 0, 0).unwrap(),
        )
    }
}

impl<'a, T: Copy> Spanned<'a, &T> {
    pub fn copied(&self) -> Spanned<T> {
        Spanned::new(
            *self.value,
            self.span.source_file.clone(),
            self.span.inner_span.clone(),
        )
    }
}

// just for simplicity (i.e. removes ".val" everywhere)
impl<'a, T> Deref for Spanned<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> ariadne::Span for Span<'a> {
    type SourceId = Rc<SourceFile<'a>>;

    fn source(&self) -> &Self::SourceId {
        &self.source_file
    }

    fn start(&self) -> usize {
        self.inner_span.start()
    }

    fn end(&self) -> usize {
        self.inner_span.end()
    }
}
