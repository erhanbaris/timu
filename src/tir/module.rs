use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::FileAst, file::SourceFile};

use super::{AstSignature, AstSignatureHolder, ObjectSignatureHolder};

#[derive(Debug)]
pub struct Module<'base> {
    #[allow(dead_code)] pub name: String,
    pub path: String,
    pub file: Rc<SourceFile<'base>>,
    pub imported_modules: HashMap<Cow<'base, str>, Rc<AstSignature<'base>>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    pub object_signatures: ObjectSignatureHolder<'base>,
    pub ast: Rc<FileAst<'base>>,
    pub modules: HashMap<Cow<'base, str>, Rc<RefCell<Module<'base>>>>,
}
