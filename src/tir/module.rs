use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{ast::FileAst, file::SourceFile};

use super::{AstSignature, AstSignatureHolder, ObjectSignatureHolder, TirContext};

#[derive(Debug)]
pub struct Module<'base> {
    #[allow(dead_code)]
    pub name: Cow<'base, str>,
    pub path: Cow<'base, str>,
    pub file: Rc<SourceFile<'base>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    pub imported_modules: IndexMap<Cow<'base, str>, Rc<AstSignature<'base>>>,
    pub object_signatures: ObjectSignatureHolder<'base>,
    pub ast: Option<Rc<FileAst<'base>>>,
    pub modules: IndexMap<Cow<'base, str>, ModuleRef<'base>>,
}

impl<'base> Module<'base> {
    pub fn new(name: Cow<'base, str>, path: Cow<'base, str>, file: Rc<SourceFile<'base>>, ast: Rc<FileAst<'base>>) -> Self {
        Self {
            name,
            path,
            file,
            ast_signatures: AstSignatureHolder::new(),
            imported_modules: IndexMap::new(),
            object_signatures: ObjectSignatureHolder::new(),
            ast: Some(ast),
            modules: IndexMap::new(),
        }
    }

    pub fn phantom(name: Cow<'base, str>, path: Cow<'base, str>, file: Rc<SourceFile<'base>>) -> Self {
        Self {
            name,
            path,
            file,
            imported_modules: IndexMap::new(),
            ast_signatures: AstSignatureHolder::new(),
            object_signatures: ObjectSignatureHolder::new(),
            ast: None,
            modules: IndexMap::new(),
        }
    }

    pub fn get_ref(&self) -> ModuleRef<'base> {
        ModuleRef::new(self.path.clone(), self.file.clone())
    }

    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<AstSignature<'base>>> {
        self.ast_signatures.get(key.as_ref())
    }
}
    

#[derive(Debug, Clone)]
pub struct ModuleRef<'base>(pub Cow<'base, str>, Rc<SourceFile<'base>>);

impl<'base> ModuleRef<'base> {
    pub fn new(path: Cow<'base, str>, file: Rc<SourceFile<'base>>) -> Self {
        ModuleRef(path, file)
    }

    pub fn upgrade<'ctx>(&self, context: &'ctx TirContext<'base>) -> Option<&'ctx Module<'base>> {
        context.modules.get(self.0.as_ref())
    }

    pub fn file(&self) -> Rc<SourceFile<'base>> {
        self.1.clone()
    }

    pub fn as_cow(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}

impl core::convert::AsRef<str> for ModuleRef<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
