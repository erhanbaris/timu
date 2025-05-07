use std::{borrow::Cow, ops::Range};

use crate::nom_tools::Span;


#[allow(dead_code)]
pub enum SplitedPathEnum<'base> {
    Path(Span<'base>),
    Splited(Vec<Span<'base>>),
}

#[derive(Debug)]
pub struct SplitedPath<'base> {
    pub paths: Vec<Span<'base>>,
    pub text: Cow<'base, str>,
}

impl<'base> SplitedPath<'base> {
    pub fn new(full_path: Span<'base>, paths: Vec<Span<'base>>) -> Self {
        let text = match full_path.fragment().contains(char::is_whitespace) {
            true => {
                let path = paths.iter().map(|path| *path.fragment())
                .collect::<Vec<&str>>()
                .join(".");
                Cow::Owned(path)
            }
            false => Cow::Borrowed(*full_path.fragment())
        };

        Self { paths, text }
    }

    pub fn to_range(&self) -> Range<usize> {
        let start = self.paths.first().map_or(0, |path| path.location_offset());
        let end = self.paths.last().map_or(0, |path| path.location_offset() + path.fragment().len());
        start..end
    }
}