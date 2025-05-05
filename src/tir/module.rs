use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::ast::FileAst;

use super::signature::{Signature, SignatureHolder};

#[derive(Debug)]
pub struct Module<'base> {
    pub name: String,
    pub path: String,
    pub imported_modules: HashMap<Cow<'base, str>, Rc<Signature<'base>>>,
    pub signatures: SignatureHolder<'base>,
    pub ast: Rc<FileAst<'base>>,
}
