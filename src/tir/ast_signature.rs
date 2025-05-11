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
    Module(#[allow(dead_code)] Cow<'base, str>),
    Class(#[allow(dead_code)] Rc<ClassDefinitionAst<'base>>),
    Function(#[allow(dead_code)] Rc<FunctionDefinitionAst<'base>>),
    Interface(#[allow(dead_code)] Rc<InterfaceDefinitionAst<'base>>),
}

impl<'base> ResolveSignature<'base> for AstSignatureValue<'base> {
    type Item = Rc<ObjectSignature<'base>>;

    fn resolve(&self, context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Item, TirError<'base>> {
        match self {
            AstSignatureValue::Module(target_module) => {
                let target_module = context.modules.get(target_module).ok_or_else(|| {
                    TirError::ModuleNotFound {
                        module: target_module.clone(),
                        source: module.file.clone(),

                    }
                })?;
                let target_module = target_module.borrow_mut();
                target_module.resolve(context, module)
            }
            AstSignatureValue::Class(class) => class.resolve(context, module),
            AstSignatureValue::Function(function) => function.resolve(context, module),
            AstSignatureValue::Interface(interface) => interface.resolve(context, module),
        }
    }
}

pub fn build_module<'base>(context: &mut TirContext<'base>, ast: Rc<FileAst<'base>>) -> Result<(), TirError<'base>> {
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
                let sub_module = match total_item == index + 1 {
                    true => Module::new(name.clone(), full_module_path.clone().into(),file.clone(), ast.clone()),
                    false => Module::phantom(name.clone(), full_module_path.clone().into(),file.clone()),
                };

                //debug!("Adding module {} to context", &full_module_path);

                let sub_module = build_module_signature(context, sub_module)?;
                if !base_module_path.is_empty() {
                    debug!("Adding submodule {} to base module {}", full_module_path, base_module_path);

                    if let Some(base_module) = context.modules.get_mut(base_module_path.as_str()) {
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
            name: ast.file.path()[ast.file.path().len() - 1].clone().clone(),
            file: ast.file.clone(),
            path: ast.file.path().join(".").into(),
            imported_modules: HashMap::new(),
            object_signatures: SignatureHolder::<ObjectSignatureValue>::new(),
            ast_signatures: SignatureHolder::<AstSignatureValue, Cow<'base, str>>::new(),
            ast: Some(ast.clone()),
            modules: Default::default(),
        };
        debug!("Adding module {} to context", module.path);
        build_module_signature(context, module)?;
    }

    Ok(())
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, module: Module<'base>) -> Result<Rc<RefCell<Module<'base>>>, TirError<'base>> {
    let mut module = module;
    let module_name = module.path.to_string();

    if let Some(ast) = &module.ast {
        // Class signatures
        for class in ast.get_classes() {
            let signature = Rc::new(Signature::from((class.clone(), module.path.clone())));

            context
                .add_ast_signature(format!("{}.{}", module.path.clone(), class.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature((*class.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), signature.file.clone())))?;
        }

        // Function signatures
        for func in ast.get_functions() {
            let signature = Rc::new(Signature::from((func.clone(), module.path.clone())));

            context
                .add_ast_signature(format!("{}.{}", module.path.clone(), func.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature((*func.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), signature.file.clone())))?;
        }

        // Interface signatures
        for interface in ast.get_interfaces() {
            let signature = Rc::new(Signature::from((interface.clone(), module.path.clone())));

            context
                .add_ast_signature(format!("{}.{}", module.path.clone(), interface.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
            module
                .ast_signatures
                .add_signature((*interface.name.fragment()).into(), signature.clone())
                .map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), signature.file.clone())))?;
        }
    }


    let signature = Signature::new(
        AstSignatureValue::Module(module.path.clone()),
        module.file.clone(),
        std::ops::Range {
            start: 0,
            end: 0,
        },
    );

    context.add_ast_signature(module_name.clone().into(), signature.into()).map_or(Ok(()), |item| {
        error!("Module already defined: {:?}", item);
        Err(TirError::ModuleAlreadyDefined {
            source: module.file.clone(),
        })
    })?;

    let module = Rc::new(RefCell::new(module));
    context.modules.insert(module_name.into(), module.clone());
    Ok(module)
}

impl<'base> From<(Rc<FunctionDefinitionAst<'base>>, Cow<'base, str>)> for Signature<'base, AstSignatureValue<'base>, Cow<'base, str>> {
    fn from(value: (Rc<FunctionDefinitionAst<'base>>, Cow<'base, str>)) -> Self {
        let (function, module) = value;

        let position = function.name.to_range();
        let file = function.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Function(function), file, position, module)
    }
}

impl<'base> From<(Rc<ClassDefinitionAst<'base>>, Cow<'base, str>)> for Signature<'base, AstSignatureValue<'base>, Cow<'base, str>> {
    fn from(value: (Rc<ClassDefinitionAst<'base>>, Cow<'base, str>)) -> Self {
        let (class, module) = value;

        let position = class.name.to_range();
        let file = class.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Class(class), file, position, module)
    }
}

impl<'base> From<(Rc<InterfaceDefinitionAst<'base>>, Cow<'base, str>)> for Signature<'base, AstSignatureValue<'base>, Cow<'base, str>> {
    fn from(value: (Rc<InterfaceDefinitionAst<'base>>, Cow<'base, str>)) -> Self {
        let (interface, module) = value;

        let position = interface.name.to_range();
        let file = interface.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Interface(interface), file, position, module)
    }
}
