use crate::SourceSpan;

#[derive(Clone, Debug)]
pub struct LabelField {
    pub position: SourceSpan,
    pub label: String,
}

pub trait TimuErrorTrait {
    fn labels(&self) -> Vec<LabelField>;
    fn source_code(&self) -> Option<String>;
    fn error_code(&self) -> Option<String>;
    fn help(&self) -> Option<String>;
}