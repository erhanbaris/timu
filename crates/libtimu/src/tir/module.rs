use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{ast::FileAst, file::SourceFile, map::TimuHashMap};

use super::{resolver::{AstSignatureLocation, TypeLocation}, signature::SignaturePath, TirContext};

#[derive(Debug)]
pub struct Module<'base> {
    #[allow(dead_code)]
    pub name: Cow<'base, str>,
    pub path: Cow<'base, str>,
    pub file: SourceFile,
    pub ast_signatures: TimuHashMap<SignaturePath<'base>, AstSignatureLocation>,
    pub ast_imported_modules: IndexMap<Cow<'base, str>, AstSignatureLocation>,
    pub types: IndexMap<SignaturePath<'base>, TypeLocation>,
    pub ast: Option<Rc<FileAst<'base>>>,
    pub modules: IndexMap<Cow<'base, str>, ModuleRef<'base>>,
}

impl<'base> Module<'base> {
    pub fn new(name: Cow<'base, str>, path: Cow<'base, str>, file: SourceFile, ast: Rc<FileAst<'base>>) -> Self {
        Self {
            name,
            path,
            file,
            ast_signatures: TimuHashMap::new(),
            ast_imported_modules: IndexMap::new(),
            types: IndexMap::new(),
            ast: Some(ast),
            modules: IndexMap::new(),
        }
    }

    pub fn phantom(name: Cow<'base, str>, path: Cow<'base, str>, file: SourceFile) -> Self {
        Self {
            name,
            path,
            file,
            ast_imported_modules: IndexMap::new(),
            ast_signatures: TimuHashMap::new(),
            types: IndexMap::new(),
            ast: None,
            modules: IndexMap::new(),
        }
    }

    pub fn get_ref(&self) -> ModuleRef<'base> {
        ModuleRef::new(self.path.clone(), self.file.clone())
    }

    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<AstSignatureLocation> {
        self.ast_signatures.get(key.as_ref()).cloned()
    }
}
    

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleRef<'base>(pub Cow<'base, str>, SourceFile);

impl<'base> ModuleRef<'base> {
    pub fn new(path: Cow<'base, str>, file: SourceFile) -> Self {
        ModuleRef(path, file)
    }

    pub fn upgrade<'ctx>(&self, context: &'ctx TirContext<'base>) -> Option<&'ctx Module<'base>> {
        context.modules.get(self.0.as_ref())
    }

    pub fn file(&self) -> SourceFile {
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
