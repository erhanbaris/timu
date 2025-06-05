use std::{fmt::{Debug, Display}, ops::Range};

use crate::SourceSpan;

#[derive(Clone, Debug)]
pub struct LabelField {
    pub position: SourceSpan,
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

pub trait TimuErrorTrait {
    fn labels(&self) -> Option<Vec<LabelField>>;
    fn source_code(&self) -> Option<Box<crate::SourceCode>> { None }
    fn error_code(&self) -> Option<Box<dyn Display>> { None }
    fn help(&self) -> Option<Box<dyn Display>> { None }
}
