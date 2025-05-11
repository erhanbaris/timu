use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::{ast::FileAst, file::SourceFile};

use super::{AstSignature, AstSignatureHolder, ObjectSignatureHolder, TirContext};

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
    pub modules: HashMap<Cow<'base, str>, ModuleRef<'base>>,
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

    pub fn get_ref(&self) -> ModuleRef<'base> {
        ModuleRef::new(self.path.clone(), self.file.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ModuleRef<'base>(Cow<'base, str>, Rc<SourceFile<'base>>);

impl<'base> ModuleRef<'base> {
    pub fn new(path: Cow<'base, str>, file: Rc<SourceFile<'base>>) -> Self {
        ModuleRef(path, file)
    }

    pub fn upgrade<'ctx>(&self, context: &'ctx TirContext<'base>) -> Option<&'ctx Module<'base>> {
        context.modules.get(self.0.as_ref())
    }

    pub fn upgrade_mut<'ctx>(&self, context: &'ctx mut TirContext<'base>) -> Option<&'ctx mut Module<'base>> {
        context.modules.get_mut(self.0.as_ref())
    }

    pub fn file<'ctx>(&self) -> Rc<SourceFile<'base>> {
        self.1.clone()
    }

    pub fn as_cow<'ctx>(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}

impl<'base> core::convert::AsRef<str> for ModuleRef<'base> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
