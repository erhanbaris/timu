use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{ast::{ClassDefinitionAst, FunctionDefinitionAst, InterfaceDefinitionAst}, nom_tools::ToRange};

use super::{context::TirContext, module::Module, signature::Signature, TirError};

#[derive(Debug)]
pub enum AstSignatureValue<'base> {
    Module(#[allow(dead_code)] Rc<RefCell<Module<'base>>>),
    Class(#[allow(dead_code)]Rc<ClassDefinitionAst<'base>>),
    Function(#[allow(dead_code)]Rc<FunctionDefinitionAst<'base>>),
    Interface(#[allow(dead_code)]Rc<InterfaceDefinitionAst<'base>>),
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, module: Module<'base>) -> Result<Rc<RefCell<Module<'base>>>, TirError<'base>> {
    let mut module = module;
    let module_name = module.path.to_string();

    // Class signatures
    for class in module.ast.get_classes() {
        let signature = Rc::new(Signature::from(class.clone()));

        context.ast_signatures.add_signature(format!("{}.{}", module.path, class.name.fragment()), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
        module.ast_signatures.add_signature(class.name.fragment().to_string(), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
    }

    // Function signatures
    for func in module.ast.get_functions() {
        let signature = Rc::new(Signature::from(func.clone()));

        context.ast_signatures.add_signature(format!("{}.{}", module.path, func.name.fragment()), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
        module.ast_signatures.add_signature(func.name.fragment().to_string(), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
    }

    // Interface signatures
    for interface in module.ast.get_interfaces() {
        let signature = Rc::new(Signature::from(interface.clone()));

        context.ast_signatures.add_signature(format!("{}.{}", module.path, interface.name.fragment()), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
        module.ast_signatures.add_signature(interface.name.fragment().to_string(), signature.clone()).map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
    }

    let module = Rc::new(RefCell::new(module));
    let signature = Rc::new(Signature::from(module.clone()));

    context.ast_signatures.add_signature(module_name, signature).map_or(Ok(()), |_| {
        Err(TirError::ModuleAlreadyDefined {
            source: module.borrow().file.clone(),
        })
    })?;

    Ok(module)
}

impl<'base> From<Rc<FunctionDefinitionAst<'base>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(function: Rc<FunctionDefinitionAst<'base>>) -> Self {
        let position = function.name.to_range();
        let file = function.name.extra.file.clone();

        Signature::new(
            AstSignatureValue::Function(function),
            file,
            position,
        )
    }
}

impl<'base> From<Rc<ClassDefinitionAst<'base>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(class: Rc<ClassDefinitionAst<'base>>) -> Self {
        let position = class.name.to_range();
        let file = class.name.extra.file.clone();

        Signature::new(
            AstSignatureValue::Class(class),
            file,
            position,
        )
    }
}

impl<'base> From<Rc<InterfaceDefinitionAst<'base>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(interface: Rc<InterfaceDefinitionAst<'base>>) -> Self {
        let position = interface.name.to_range();
        let file = interface.name.extra.file.clone();

        Signature::new(
            AstSignatureValue::Interface(interface),
            file,
            position,
        )
    }
}

impl<'base> From<Rc<RefCell<Module<'base>>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(module: Rc<RefCell<Module<'base>>>) -> Self {
        let file = module.borrow().file.clone();

        Signature::new(
            AstSignatureValue::Module(module),
            file,
            std::ops::Range {
                start: 0,
                end: 0,
            },
        )
    }
}
