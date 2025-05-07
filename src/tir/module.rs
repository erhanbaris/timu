use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::{ast::FileAst, file::SourceFile};

use super::{resolver::try_resolve_signature, AstSignature, AstSignatureHolder, ObjectSignature, ObjectSignatureHolder};


#[derive(Debug)]
pub struct Module<'base> {
    #[allow(dead_code)] pub name: String,
    pub path: String,
    pub file: Rc<SourceFile<'base>>,
    pub imported_modules: HashMap<Cow<'base, str>, Rc<AstSignature<'base>>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    pub object_signatures: ObjectSignatureHolder<'base>,
    pub ast: Rc<FileAst<'base>>,
}

impl<'base> Module<'base> {
    pub fn get_or_resolve_object_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<ObjectSignature<'base>>> {
        match self.object_signatures.get(key.as_ref()) {
            Some(signature) => Some(signature),
            None => {
                if let Some(imported_module) = self.imported_modules.get(key.as_ref()) {
                    try_resolve_signature(context, module, signature)
                    return Some(imported_module.clone());
                }
                None
            }
        }
    }
}