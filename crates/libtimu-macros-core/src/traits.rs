use std::{fmt::{Debug, Display}, ops::Range};

#[derive(Clone, Debug)]
pub struct LabelField {
    pub position: Range<usize>,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct LabeledSpan {
    pub label: String,
    pub span: Range<usize>,
}

impl LabeledSpan {
    pub fn new(label: String, span: Range<usize>) -> Self {
        Self { label, span }
    }
}

pub trait TimuErrorTrait: Display {
    fn labels(&self) -> Option<Vec<LabelField>>;
    fn errors<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn TimuErrorTrait> + 'a>>;
    fn references(&self) -> Option<Vec<Box<&dyn TimuErrorTrait>>>;
    fn source_code(&self) -> Option<Box<crate::SourceCode>> { None }
    fn error_code(&self) -> Option<Box<dyn Display>> { None }
    fn help(&self) -> Option<Box<dyn Display>> { None }
}
