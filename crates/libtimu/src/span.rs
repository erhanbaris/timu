use std::{path::PathBuf, rc::Rc};

use internment::Intern;
use std::ops::{Deref, Range};

use crate::file::SourceFile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<'base> {
    source_file: Rc<SourceFile<'base>>,
    inner_span: pest::Span<'base>,
}

impl<'base> Span<'base> {
    pub fn new(source_file: Rc<SourceFile<'base>>, inner_span: pest::Span<'base>) -> Self {
        Self {
            source_file,
            inner_span,
        }
    }

    pub fn spanned<T>(self, value: T) -> Spanned<'base, T> {
        Spanned::new(value, self.source_file.clone(), self.inner_span)
    }

    pub fn src(&self) -> Rc<SourceFile<'base>> {
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
pub struct Spanned<'base, T> {
    pub value: T,
    pub span: Span<'base>,
}

impl<'base, T: PartialEq> PartialEq for Spanned<'base, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'base, T> Spanned<'base, T> {
    pub fn new(value: T, file: Rc<SourceFile<'base>>, inner_span: pest::Span<'base>) -> Self {
        Self {
            value,
            span: Span::new(file, inner_span),
        }
    }

    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned::new(&self.value, self.span.source_file.clone(), self.span.inner_span.clone())
    }

    pub fn span_to<V>(&self, to: V) -> Spanned<V> {
        Spanned::new(to, self.span.source_file.clone(), self.span.inner_span.clone())
    }
}

impl<'base, T> From<T> for Spanned<'base, T> {
    fn from(value: T) -> Self {
        Spanned::new(value, Rc::new(SourceFile::new(PathBuf::from("<MEMORY>"), "")), pest::Span::new("", 0, 0).unwrap())
    }
}

impl<'base, T: Copy> Spanned<'base, &T> {
    pub fn copied(&self) -> Spanned<T> {
        Spanned::new(*self.value, self.span.source_file.clone(), self.span.inner_span.clone())
    }
}

// just for simplicity (i.e. removes ".val" everywhere)
impl<'base, T> Deref for Spanned<'base, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'base> ariadne::Span for Span<'base> {
    type SourceId = Rc<SourceFile<'base>>;

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
