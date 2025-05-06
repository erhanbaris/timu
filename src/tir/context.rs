use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::ast::{ClassDefinitionAst, FunctionDefinitionAst, InterfaceDefinitionAst};

use super::{signature::SignatureHolderType, AstSignatureHolder, Module, Signature};

#[derive(Default, Debug)]
pub struct AstSignatureHolderType<'base> {
    marker: PhantomData<&'base ()>,
}

impl<'base> SignatureHolderType for AstSignatureHolderType<'base> {
    type ModuleType = Module<'base>;
    type ClassType = ClassDefinitionAst<'base>;
    type FunctionType = FunctionDefinitionAst<'base>;
    type InterfaceType = InterfaceDefinitionAst<'base>;
}

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: Vec<Rc<RefCell<Module<'base>>>>,
    pub ast_signatures: AstSignatureHolder<'base>,
}

impl<'base> TirContext<'base> {
    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<Rc<Signature<AstSignatureHolderType<'base>>>> {
        self.ast_signatures.get(key.as_ref())
    }
}
