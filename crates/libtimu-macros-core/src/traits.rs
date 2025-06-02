use crate::SourceSpan;

pub trait TimuErrorTrait {
    fn labels(&self) -> Vec<SourceSpan>;
}