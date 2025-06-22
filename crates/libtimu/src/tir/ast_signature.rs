use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;
use simplelog::debug;
use strum_macros::EnumDiscriminants;

use crate::{
    ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst}, map::TimuHashMap, nom_tools::ToRange, tir::{scope::AstVariableInformation, TypeSignature, TypeValue}
};

use super::{
    context::TirContext, module::{Module, ModuleRef}, resolver::{ResolveAst, TypeLocation}, scope::ScopeLocation, signature::{Signature, SignaturePath}, AstSignature, TirError
};

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(vis(pub))]
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
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        match self {
            AstSignatureValue::Module(target_module) => target_module.resolve(context, scope_location),
            AstSignatureValue::Class(class) => class.resolve(context, scope_location),
            AstSignatureValue::Function(function) => function.resolve(context, scope_location),
            AstSignatureValue::Interface(interface) => interface.resolve(context, scope_location),
            AstSignatureValue::Extend(extend) => extend.resolve(context, scope_location),
        }
    }

    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
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

pub fn build_module<'base>(context: &mut TirContext<'base>, ast: Rc<FileAst<'base>>) -> Result<(), TirError> {
    let module_path = ast.file.path();
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
                let next_scope_location = context.get_next_scope_location();
                let sub_module = match total_item == index + 1 {
                    true => Module::new(name.clone().into(), full_module_path.clone().into(),file.clone(), ast.clone(), next_scope_location),
                    false => Module::phantom(name.clone().into(), full_module_path.clone().into(),file.clone(), next_scope_location),
                };

                // Create new scope for module
                context.create_scope(full_module_path.clone().into(), sub_module.get_ref());
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
        let scope_location = context.get_next_scope_location();
        let module = Module {
            name: ast.file.path()[ast.file.path().len() - 1].clone().into(),
            file: ast.file.clone(),
            path: ast.file.path().join(".").into(),
            ast_imported_modules: IndexMap::new(),
            ast_signatures: TimuHashMap::new(),
            types: IndexMap::new(),
            ast: Some(ast.clone()),
            modules: Default::default(),
            scope_location
        };

        // Create new scope for module
        context.create_scope(ast.file.path().join(".").into(), module.get_ref());
        
        debug!("Adding module to context: <u><b>{}</b></u>", module.path);
        build_module_signature(context, module)?;
    }

    Ok(())
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, mut module: Module<'base>) -> Result<(), TirError> {
    let module_name = module.path.to_string();
    let mut ast_signature: TimuHashMap<SignaturePath<'base>, AstVariableInformation> = TimuHashMap::new();

    if let Some(ast) = &module.ast {
        // Interface signatures
        for interface in ast.get_interfaces() {
            let signature = Signature::from((interface.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), interface.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(interface.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(interface.name.text), variable)?;
        }

        // Extend signatures
        for extend in ast.get_extends() {
            let signature = Signature::from((extend.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), extend.name()).into(), signature)?;
            let variable = AstVariableInformation::basic(extend.name.names.last().unwrap().clone(), location);

            ast_signature.validate_insert(SignaturePath::cow(extend.name()), variable)?;
        }

        // Class signatures
        for class in ast.get_classes() {
            let signature = Signature::from((class.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), class.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(class.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(class.name.text), variable)?;
        }

        // Function signatures
        for func in ast.get_functions() {
            let signature = Signature::from((func.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), func.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(func.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(func.name.text), variable)?;
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

    let module_ref = module.get_ref();

    context.add_ast_signature(module_name.clone().into(), signature)?;
    context.types.add_signature(SignaturePath::owned(module_name.clone()), TypeSignature::new(TypeValue::Module(module_ref), module.file.clone(), 0..0, None)).unwrap();
    context.modules.insert(module_name.into(), module);
    Ok(())
}

impl<'base> From<(Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (function, module) = value;

        let position = function.name.to_range();
        let file = function.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Function(function), file, position, module)
    }
}

impl<'base> From<(Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (class, module) = value;

        let position = class.name.to_range();
        let file = class.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Class(class), file, position, module)
    }
}

impl<'base> From<(Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (interface, module) = value;

        let position = interface.name.to_range();
        let file = interface.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Interface(interface), file, position, module)
    }
}


impl<'base> From<(Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    fn from(value: (Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (extend, module) = value;

        let position = extend.name.to_range();
        let file = extend.name.names.first().unwrap().state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Extend(extend), file, position, module)
    }
}
