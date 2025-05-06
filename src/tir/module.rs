use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::ast::FileAst;

use super::{AstSignature, AstSignatureHolder};

#[derive(Debug)]
pub struct Module<'base> {
    pub name: String,
    pub path: String,
    pub imported_modules: HashMap<Cow<'base, str>, Rc<AstSignature<'base>>>,
    pub signatures: AstSignatureHolder<'base>,
    pub ast: Rc<FileAst<'base>>,
}
