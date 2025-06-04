use std::fmt::{Debug, Display};

use crate::SourceSpan;

#[derive(Clone, Debug)]
pub struct LabelField {
    pub position: SourceSpan,
    pub label: String,
}

pub trait TimuErrorTrait {
    fn labels(&self) -> Vec<LabelField>;
    fn source_code(&self) -> Option<Box<dyn Display>> { None }
    fn error_code(&self) -> Option<Box<dyn Display>> { None }
    fn help(&self) -> Option<Box<dyn Display>> { None }
}