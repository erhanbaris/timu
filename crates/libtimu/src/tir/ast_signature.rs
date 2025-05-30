use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;
use simplelog::{debug, error};

use crate::{
    ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst},
    nom_tools::ToRange,
};

use super::{
    context::TirContext, module::{Module, ModuleRef}, resolver::{AstSignatureLocation, ResolveAst, TypeLocation}, scope::ScopeLocation, signature::{Signature, SignaturePath}, AstSignature, TirError
};

#[derive(Debug, Clone, PartialEq)]
pub enum AstSignatureValue<'base> {
    Module(#[allow(dead_code)] ModuleRef<'base>),
    Class(#[allow(dead_code)] Rc<ClassDefinitionAst<'base>>),
    Function(#[allow(dead_code)] Rc<FunctionDefinitionAst<'base>>),
    Interface(#[allow(dead_code)] Rc<InterfaceDefinitionAst<'base>>),
    Extend(#[allow(dead_code)] Rc<ExtendDefinitionAst<'base>>),
}

impl<'base> AsRef<AstSignatureValue<'base>> for AstSignatureValue<'base> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'base> AsMut<AstSignatureValue<'base>> for AstSignatureValue<'base> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'base> ResolveAst<'base> for AstSignatureValue<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError<'base>> {
        match self {
            AstSignatureValue::Module(target_module) => target_module.resolve(context, scope_location),
            AstSignatureValue::Class(class) => class.resolve(context, scope_location),
            AstSignatureValue::Function(function) => function.resolve(context, scope_location),
            AstSignatureValue::Interface(interface) => interface.resolve(context, scope_location),
            AstSignatureValue::Extend(extend) => extend.resolve(context, scope_location),
        }
    }

    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        match self {
            AstSignatureValue::Module(module) => module.name(),
            AstSignatureValue::Class(class) => class.name(),
            AstSignatureValue::Function(function) => function.name(),
            AstSignatureValue::Interface(interface) => interface.name(),
            AstSignatureValue::Extend(extend) => extend.name(),
        }
    }
}

pub fn build_module<'base>(context: &mut TirContext<'base>, ast: Rc<FileAst<'base>>) -> Result<(), TirError<'base>> {
    let module_path = ast.file.path().clone();
    let file = ast.file.clone();
    debug!("Building module: <u><b>{:?}</b></u>", module_path);

    if module_path.len() > 1 {
        let mut base_module_path = String::new();
        let total_item = module_path.len();

        for (index, name) in module_path[0..module_path.len()].iter().enumerate() {
            let full_module_path = module_path[..index + 1].join(".");
            let is_module_missing = context.get_ast_signature(full_module_path.as_str()).is_none();
            debug!("Searching module <u><b>{}</b></u>. Is missing: {}", full_module_path, is_module_missing);

            if is_module_missing {
                let sub_module = match total_item == index + 1 {
                    true => Module::new(name.clone(), full_module_path.clone().into(),file.clone(), ast.clone()),
                    false => Module::phantom(name.clone(), full_module_path.clone().into(),file.clone()),
                };


                let sub_module_ref = sub_module.get_ref();
                build_module_signature(context, sub_module)?;
                
                if !base_module_path.is_empty() {
                    debug!("Adding submodule <u><b>{}</b></u> to base module {}", full_module_path, base_module_path);

                    if let Some(base_module) = context.modules.get_mut(base_module_path.as_str()) {
                        base_module.modules.insert(name.to_string().into(), sub_module_ref);
                    } else {
                        panic!("Base module <u><b>{}</b></u> not found in context", base_module_path);
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
            ast_imported_modules: IndexMap::new(),
            ast_signatures: IndexMap::new(),
            types: IndexMap::new(),
            ast: Some(ast.clone()),
            modules: Default::default(),
        };
        debug!("Adding module to context: <u><b>{}</b></u>", module.path);
        build_module_signature(context, module)?;
    }

    Ok(())
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, mut module: Module<'base>) -> Result<(), TirError<'base>> {
    let module_name = module.path.to_string();
    let mut ast_signature: IndexMap<SignaturePath<'base>, AstSignatureLocation> = IndexMap::new();

    if let Some(ast) = &module.ast {
        // Interface signatures
        for interface in ast.get_interfaces() {
            let signature = Signature::from((interface.clone(), module.get_ref()));
            let source = signature.file.clone();
            
            let location = context
                .add_ast_signature(format!("{}.{}", module.path.clone(), interface.name.fragment()).into(), signature)
                .map_err(|_| TirError::already_defined(interface.name.to_range(), source.clone()))?;

            ast_signature.insert(SignaturePath::borrowed(interface.name.fragment()), location)
                .map_or(Ok(()), |_| Err(TirError::already_defined(interface.name.to_range(), source)))?;
        }

        // Extend signatures
        for extend in ast.get_extends() {
            let signature = Signature::from((extend.clone(), module.get_ref()));
            let source = signature.file.clone();

            let location = context
                .add_ast_signature(format!("{}.{}", module.path.clone(), extend.name()).into(), signature)
                .map_err(|_| TirError::already_defined(extend.name.to_range(), source.clone()))?;

            ast_signature.insert(SignaturePath::cow(extend.name()), location)
                .map_or(Ok(()), |_| Err(TirError::already_defined(extend.name.to_range(), source)))?;
        }

        // Class signatures
        for class in ast.get_classes() {
            let signature = Signature::from((class.clone(), module.get_ref()));
            let source = signature.file.clone();

            let location = context
                .add_ast_signature(format!("{}.{}", module.path.clone(), class.name.fragment()).into(), signature)
                .map_err(|_| TirError::already_defined(class.name.to_range(), source.clone()))?;

            ast_signature.insert(SignaturePath::borrowed(class.name.fragment()), location)
                .map_or(Ok(()), |_| Err(TirError::already_defined(class.name.to_range(), source)))?;
        }

        // Function signatures
        for func in ast.get_functions() {
            let signature = Signature::from((func.clone(), module.get_ref()));
            let source = signature.file.clone();

            let location = context
                .add_ast_signature(format!("{}.{}", module.path.clone(), func.name.fragment()).into(), signature)
                .map_err(|_| TirError::already_defined(func.name.to_range(), source.clone()))?;

            ast_signature.insert(SignaturePath::borrowed(func.name.fragment()), location)
                .map_or(Ok(()), |_| Err(TirError::already_defined(func.name.to_range(), source)))?;
        }
    }

    module.ast_signatures = ast_signature;

    let signature = AstSignature::new(
        AstSignatureValue::Module(module.get_ref()),
        module.file.clone(),
        std::ops::Range {
            start: 0,
            end: 0,
        },
        None
    );

    context.add_ast_signature(module_name.clone().into(), signature).map_err(|item| {
        error!("Module already defined: {:?}", item);
        TirError::ModuleAlreadyDefined {
            source: module.file.clone(),
        }
    })?;

    context.modules.insert(module_name.into(), module);
    Ok(())
}

impl<'base> From<(Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (function, module) = value;

        let position = function.name.to_range();
        let file = function.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Function(function), file, position, module)
    }
}

impl<'base> From<(Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (class, module) = value;

        let position = class.name.to_range();
        let file = class.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Class(class), file, position, module)
    }
}

impl<'base> From<(Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (interface, module) = value;

        let position = interface.name.to_range();
        let file = interface.name.extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Interface(interface), file, position, module)
    }
}


impl<'base> From<(Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (extend, module) = value;

        let position = extend.name.to_range();
        let file = extend.name.names.first().unwrap().extra.file.clone();
        Signature::new_with_extra(AstSignatureValue::Extend(extend), file, position, module)
    }
}
