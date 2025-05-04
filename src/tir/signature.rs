use std::{collections::HashMap, rc::Rc};

use crate::ast::{ClassDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst};

use super::{ProjectModule, TirError, context::TirContext};

#[derive(Debug)]
pub enum ModuleSignature<'base> {
    Module(Rc<ProjectModule<'base>>),
    Class(Rc<ClassDefinitionAst<'base>>),
    Function(Rc<FunctionDefinitionAst<'base>>),
    Interface(Rc<InterfaceDefinitionAst<'base>>),
}

#[derive(Debug, Default)]
pub struct SignatureHolder<'base> {
    signatures: HashMap<String, Rc<ModuleSignature<'base>>>,
}

impl<'base> SignatureHolder<'base> {
    pub fn add_class(&mut self, name: String, class: Rc<ClassDefinitionAst<'base>>) -> Result<(), TirError<'base>> {
        self.add_signature(name, ModuleSignature::Class(class))
    }

    pub fn add_function(&mut self, name: String, func: Rc<FunctionDefinitionAst<'base>>) -> Result<(), TirError<'base>> {
        self.add_signature(name, ModuleSignature::Function(func))
    }

    pub fn add_interface(&mut self, name: String, interface: Rc<InterfaceDefinitionAst<'base>>) -> Result<(), TirError<'base>> {
        self.add_signature(name, ModuleSignature::Interface(interface))
    }

    pub fn add_module(&mut self, name: String, module: Rc<ProjectModule<'base>>) -> Result<(), TirError<'base>> {
        self.add_signature(name, ModuleSignature::Module(module))
    }

    fn add_signature(&mut self, name: String, new_signature: ModuleSignature<'base>) -> Result<(), TirError<'base>> {
        match self.signatures.insert(name, new_signature.into()) {
            Some(old_signature) => Err(TirError::SignatureAlreadyDefined {
                old_signature,
            }),
            None => Ok(()),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<ModuleSignature<'base>>> {
        self.signatures.get(name).cloned()
    }
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, file_ast: Rc<FileAst<'base>>) -> Result<(), TirError<'base>> {
    let mut module_signatures = SignatureHolder::default();

    // Class signatures
    for class in file_ast.get_classes() {
        context.signatures.add_class(format!("{}.{}", file_ast.file.name(), class.name.fragment()), class.clone())?;
        module_signatures.add_class(class.name.fragment().to_string(), class)?;
    }

    // Function signatures
    for func in file_ast.get_functions() {
        context.signatures.add_function(format!("{}.{}", file_ast.file.name(), func.name.fragment()), func.clone())?;
        module_signatures.add_function(func.name.fragment().to_string(), func)?;
    }

    // Imterface signatures
    for interface in file_ast.get_interfaces() {
        context.signatures.add_interface(format!("{}.{}", file_ast.file.name(), interface.name.fragment()), interface.clone())?;
        module_signatures.add_interface(interface.name.fragment().to_string(), interface)?;
    }

    let module: Rc<ProjectModule> = ProjectModule {
        name: file_ast.file.name(),
        modules: Vec::new(),
        imported_modules: HashMap::new(),
        signatures: module_signatures,
    }
    .into();
    context.modules.push(module.clone());
    context.signatures.add_module(file_ast.file.name().to_string(), module)?;

    Ok(())
}
