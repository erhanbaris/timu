use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::file::SourceFile;

use super::{context::{AstSignatureHolderType, TirContext}, Module};

#[derive(Debug)]
pub enum SignatureError<'base, T: SignatureHolderType> {
    AlreadyDefined {
        #[allow(dead_code)] old_signature: Rc<Signature<T>>,
        source: Rc<SourceFile<'base>>
    },
}

pub trait SignatureHolderType: Debug {
    type ModuleType: Debug;
    type ClassType: Debug;
    type FunctionType: Debug;
    type InterfaceType: Debug;
} 

#[derive(Debug)]
pub enum Signature<T: SignatureHolderType> {
    Module(#[allow(dead_code)] Rc<RefCell<T::ModuleType>>),
    Class(#[allow(dead_code)]Rc<T::ClassType>),
    Function(#[allow(dead_code)]Rc<T::FunctionType>),
    Interface(#[allow(dead_code)]Rc<T::InterfaceType>),
}

#[derive(Debug, Default)]
pub struct SignatureHolder<'base, T: SignatureHolderType> {
    signatures: HashMap<String, Rc<Signature<T>>>,
    _marker: std::marker::PhantomData<&'base ()>,
}


impl<'base, T> SignatureHolder<'base, T> where T: SignatureHolderType {
    pub fn add_class(&mut self, name: String, class: Rc<T::ClassType>) -> Result<(), SignatureError<T>> {
        self.add_signature(name, Signature::Class(class))
    }

    pub fn add_function(&mut self, name: String, func: Rc<T::FunctionType>) -> Result<(), SignatureError<T>> {
        self.add_signature(name, Signature::Function(func))
    }

    pub fn add_interface(&mut self, name: String, interface: Rc<T::InterfaceType>) -> Result<(), SignatureError<T>> {
        self.add_signature(name, Signature::Interface(interface))
    }

    pub fn add_module(&mut self, name: String, module: Rc<RefCell<T::ModuleType>>) -> Result<(), SignatureError<T>> {
        self.add_signature(name, Signature::Module(module))
    }

    fn add_signature(&mut self, name: String, new_signature: Signature::<T>) -> Result<(), SignatureError<T>> {
        match self.signatures.insert(name, new_signature.into()) {
            Some(old_signature) => Err(SignatureError::AlreadyDefined { old_signature }),
            None => Ok(()),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<Signature::<T>>> {
        self.signatures.get(name).cloned()
    }
}

pub fn build_module_signature<'base>(context: &mut TirContext<'base>, module: Module<'base>) -> Result<Rc<RefCell<Module<'base>>>, SignatureError<'base, AstSignatureHolderType<'base>>> {
    let mut module = module;
    let module_name = module.path.to_string();

    // Class signatures
    for class in module.ast.get_classes() {
        context.ast_signatures.add_class(format!("{}.{}", module.path, class.name.fragment()), class.clone())?;
        module.signatures.add_class(class.name.fragment().to_string(), class)?;
    }

    // Function signatures
    for func in module.ast.get_functions() {
        context.ast_signatures.add_function(format!("{}.{}", module.path, func.name.fragment()), func.clone())?;
        module.signatures.add_function(func.name.fragment().to_string(), func)?;
    }

    // Imterface signatures
    for interface in module.ast.get_interfaces() {
        context.ast_signatures.add_interface(format!("{}.{}", module.path, interface.name.fragment()), interface.clone())?;
        module.signatures.add_interface(interface.name.fragment().to_string(), interface)?;
    }

    let module = Rc::new(RefCell::new(module));
    context.ast_signatures.add_module(module_name, module.clone())?;
    Ok(module)
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::process_code;

    #[test]
    fn signature_generation() -> Result<(), Box<dyn Error>> {
        let ast_1 = process_code(vec!["source".to_string()], " class testclass {} func testfunction(): testclass {} interface testinterface {}")?;
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

