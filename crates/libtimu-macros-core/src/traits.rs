use crate::SourceSpan;

#[derive(Clone, Debug)]
pub struct LabelField<'a> {
    pub position: SourceSpan,
    pub label: &'a str,
}

pub trait TimuErrorTrait {
    fn labels(&self) -> Vec<LabelField>;
    //fn code(&self) -> Vec<String>;
}