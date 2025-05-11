use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::FileAst, file::SourceFile};

use super::{AstSignature, AstSignatureHolder, ObjectSignatureHolder};

#[derive(Debug)]
pub struct Module<'base> {
    #[allow(dead_code)]
    pub name: Cow<'base, str>,
    pub path: Cow<'base, str>,
    pub file: Rc<SourceFile<'base>>,
    pub imported_modules: HashMap<Cow<'base, str>, Rc<AstSignature<'base>>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    pub object_signatures: ObjectSignatureHolder<'base>,
    pub ast: Option<Rc<FileAst<'base>>>,
    pub modules: HashMap<Cow<'base, str>, Rc<RefCell<Module<'base>>>>,
}

impl<'base> Module<'base> {
    pub fn new(name: Cow<'base, str>, path: Cow<'base, str>, file: Rc<SourceFile<'base>>, ast: Rc<FileAst<'base>>) -> Self {
        Self {
            name,
            path,
            file,
            imported_modules: HashMap::new(),
            ast_signatures: AstSignatureHolder::default(),
            object_signatures: ObjectSignatureHolder::default(),
            ast: Some(ast),
            modules: HashMap::new(),
        }
    }

    pub fn phantom(name: Cow<'base, str>, path: Cow<'base, str>, file: Rc<SourceFile<'base>>) -> Self {
        Self {
            name,
            path,
            file,
            imported_modules: HashMap::new(),
            ast_signatures: AstSignatureHolder::default(),
            object_signatures: ObjectSignatureHolder::default(),
            ast: None,
            modules: HashMap::new(),
        }
    }
}
