use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{ClassDefinitionAst, FunctionDefinitionAst, InterfaceDefinitionAst};

use super::{Module, TirError, context::TirContext};

#[derive(Debug)]
pub enum ModuleSignature<'base> {
    Module(#[allow(dead_code)] Rc<RefCell<Module<'base>>>),
    Class(#[allow(dead_code)]Rc<ClassDefinitionAst<'base>>),
    Function(#[allow(dead_code)]Rc<FunctionDefinitionAst<'base>>),
    Interface(#[allow(dead_code)]Rc<InterfaceDefinitionAst<'base>>),
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

    pub fn add_module(&mut self, name: String, module: Rc<RefCell<Module<'base>>>) -> Result<(), TirError<'base>> {
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

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, module: Module<'base>) -> Result<Rc<RefCell<Module<'base>>>, TirError<'base>> {
    let mut module = module;
    let module_name = module.path.to_string();

    // Class signatures
    for class in module.ast.get_classes() {
        context.signatures.add_class(format!("{}.{}", module.path, class.name.fragment()), class.clone())?;
        module.signatures.add_class(class.name.fragment().to_string(), class)?;
    }

    // Function signatures
    for func in module.ast.get_functions() {
        context.signatures.add_function(format!("{}.{}", module.path, func.name.fragment()), func.clone())?;
        module.signatures.add_function(func.name.fragment().to_string(), func)?;
    }

    // Imterface signatures
    for interface in module.ast.get_interfaces() {
        context.signatures.add_interface(format!("{}.{}", module.path, interface.name.fragment()), interface.clone())?;
        module.signatures.add_interface(interface.name.fragment().to_string(), interface)?;
    }

    let module = Rc::new(RefCell::new(module));
    context.signatures.add_module(module_name, module.clone())?;
    Ok(module)
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::process_code;

    #[test]
    fn signature_generation() -> Result<(), Box<dyn Error>> {
        let ast_1 = process_code(vec!["source".to_string()], " class testclass {} func testfunction(): void {} interface testinterface {}")?;
        let ast_2 = process_code(vec!["lib".to_string()], "use source; use source.testclass; use source.testfunction; use source.testinterface;")?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()])?;
        Ok(())
    }

    #[test]
    fn dublicate_signatures() -> Result<(), Box<dyn Error>> {
        let ast = process_code(vec!["source".to_string()], " class test {} func test(): void {} interface test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}

