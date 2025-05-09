use std::{
    borrow::Cow,
    cell::{RefCell, RefMut},
    collections::HashMap,
    panic,
    rc::Rc,
};

use simplelog::{debug, error, info};

use crate::{
    ast::{ClassDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst},
    nom_tools::ToRange,
};

use super::{
    ObjectSignature, TirError,
    context::TirContext,
    module::Module,
    object_signature::ObjectSignatureValue,
    resolver::ResolveSignature,
    signature::{Signature, SignatureHolder},
};

#[derive(Debug)]
pub enum AstSignatureValue<'base> {
    Module(#[allow(dead_code)] Rc<RefCell<Module<'base>>>),
    Class(#[allow(dead_code)] Rc<ClassDefinitionAst<'base>>),
    Function(#[allow(dead_code)] Rc<FunctionDefinitionAst<'base>>),
    Interface(#[allow(dead_code)] Rc<InterfaceDefinitionAst<'base>>),
}

impl<'base> ResolveSignature<'base> for AstSignatureValue<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Item, TirError<'base>> {
        match self {
            AstSignatureValue::Module(target_module) => {
                let target_module = target_module.borrow_mut();
                target_module.resolve(context, module)
            }
            AstSignatureValue::Class(class) => class.resolve(context, module),
            AstSignatureValue::Function(function) => function.resolve(context, module),
            AstSignatureValue::Interface(interface) => interface.resolve(context, module),
        }
    }
}

pub fn build_module<'base>(
    context: &mut TirContext<'base>, ast: Rc<FileAst<'base>>, modules: &mut HashMap<Cow<'base, str>, Rc<RefCell<Module<'base>>>>,
) -> Result<(), TirError<'base>> {
    let module_path = ast.file.path().clone();
    let file = ast.file.clone();
    info!("Building module: <u><b>{:?}</b></u>", module_path);

    if module_path.len() > 1 {
        let mut base_module_path = String::new();
        let total_item = module_path.len();

        for (index, name) in module_path[0..module_path.len()].iter().enumerate() {
            let full_module_path = module_path[..index + 1].join(".");
            let is_module_missing = context.get_ast_signature(full_module_path.as_str()).is_none();
            debug!("Searching module {}. Is missing: {}", full_module_path, is_module_missing);

            if is_module_missing {
                let sub_module = Module {
                    name: name.to_string(),
                    file: file.clone(),
                    path: full_module_path.clone(),
                    imported_modules: HashMap::new(),
                    object_signatures: SignatureHolder::<ObjectSignatureValue>::new(),
                    ast_signatures: SignatureHolder::<AstSignatureValue>::new(),
                    ast: match total_item == index + 1 {
                        true => Some(ast.clone()),
                        false => None,
                    },
                    modules: Default::default(),
                };
                debug!("Adding module {} to context", full_module_path);

                let sub_module = build_module_signature(context, sub_module)?;
                modules.insert(full_module_path.clone().into(), sub_module.clone());

                if !base_module_path.is_empty() {
                    debug!("Adding submodule {} to base module {}", full_module_path, base_module_path);

                    if let Some(base_module) = modules.get_mut(base_module_path.as_str()) {
                        base_module.borrow_mut().modules.insert(name.to_string().into(), sub_module.clone());
                    } else {
                        panic!("Base module {} not found in context", base_module_path);
                    }
                }
                base_module_path = full_module_path.clone();
            }
        }
    } else {
        let module = Module {
            name: ast.file.path()[ast.file.path().len() - 1].to_string(),
            file: ast.file.clone(),
            path: ast.file.path().join("."),
            imported_modules: HashMap::new(),
            object_signatures: SignatureHolder::<ObjectSignatureValue>::new(),
            ast_signatures: SignatureHolder::<AstSignatureValue>::new(),
            ast: Some(ast.clone()),
            modules: Default::default(),
        };
        debug!("Adding module {} to context", module.path);
        let module = build_module_signature(context, module)?;
        modules.insert(ast.file.path().join(".").into(), module.clone());
    }

    Ok(())
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, module: Module<'base>) -> Result<Rc<RefCell<Module<'base>>>, TirError<'base>> {
    let mut module = module;
    let module_name = module.path.to_string();

    if let Some(ast) = &module.ast {
        // Class signatures
        for class in ast.get_classes() {
            let signature = Rc::new(Signature::from(class.clone()));

            context
                .add_ast_signature(format!("{}.{}", module.path, class.name.fragment()), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature(class.name.fragment().to_string(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
        }

        // Function signatures
        for func in ast.get_functions() {
            let signature = Rc::new(Signature::from(func.clone()));

            context
                .add_ast_signature(format!("{}.{}", module.path, func.name.fragment()), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature(func.name.fragment().to_string(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
        }

        // Interface signatures
        for interface in ast.get_interfaces() {
            let signature = Rc::new(Signature::from(interface.clone()));

            context
                .add_ast_signature(format!("{}.{}", module.path, interface.name.fragment()), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature(interface.name.fragment().to_string(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
        }
    }

    let module = Rc::new(RefCell::new(module));
    let signature = Rc::new(Signature::from(module.clone()));

    context.add_ast_signature(module_name, signature).map_or(Ok(()), |item| {
        error!("Module already defined: {:?}", item);
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

        Signature::new(AstSignatureValue::Function(function), file, position)
    }
}

impl<'base> From<Rc<ClassDefinitionAst<'base>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(class: Rc<ClassDefinitionAst<'base>>) -> Self {
        let position = class.name.to_range();
        let file = class.name.extra.file.clone();

        Signature::new(AstSignatureValue::Class(class), file, position)
    }
}

impl<'base> From<Rc<InterfaceDefinitionAst<'base>>> for Signature<'base, AstSignatureValue<'base>> {
    fn from(interface: Rc<InterfaceDefinitionAst<'base>>) -> Self {
        let position = interface.name.to_range();
        let file = interface.name.extra.file.clone();

        Signature::new(AstSignatureValue::Interface(interface), file, position)
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
