use std::{borrow::Cow, ops::Range};

use crate::nom_tools::{NomSpan, Span};


#[allow(dead_code)]
pub enum SplitedPathEnum<'base> {
    Path(NomSpan<'base>),
    Splited(Vec<NomSpan<'base>>),
}

#[derive(Debug)]
pub struct SplitedPath<'base> {
    pub paths: Vec<Span<'base>>,
    pub text: Cow<'base, str>,
}

impl<'base> SplitedPath<'base> {
    pub fn new(full_path: Span<'base>, paths: Vec<Span<'base>>) -> Self {
        let text = match full_path.text.contains(char::is_whitespace) {
            true => {
                let path = paths.iter().map(|path| path.text)
                .collect::<Vec<&str>>()
                .join(".");
                Cow::Owned(path)
            }
            false => Cow::Borrowed(full_path.text)
        };

        Self { paths, text }
    }

    pub fn to_range(&self) -> Range<usize> {
        let start = self.paths.first().map_or(0, |path| path.position.start);
        let end = self.paths.last().map_or(0, |path| path.position.end + path.text.len());
        start..end
    }
}