use std::{
    borrow::Cow, cell::{RefCell, RefMut}, collections::HashMap, rc::Rc
};

use context::TirContext;
use error::TirError;
use signature::{ModuleSignature, SignatureHolder, build_module_signature};

use crate::ast::{FileAst, UseAst};

mod context;
mod error;
mod signature;

#[derive(Debug)]
pub struct Module<'base> {
    name: String,
    path: String,
    imported_modules: HashMap<Cow<'base, str>, Rc<ModuleSignature<'base>>>,
    signatures: SignatureHolder<'base>,
    ast: Rc<FileAst<'base>>,
}

pub fn build(files: Vec<Rc<FileAst<'_>>>) -> Result<(), TirError<'_>> {
    let mut context: TirContext = TirContext::default();
    let mut modules = vec![];

    for ast in files.into_iter() {
        let module = Module {
            name: ast.file.path()[ast.file.path().len() - 1].to_string(),
            path: ast.file.path().join("."),
            imported_modules: HashMap::new(),
            signatures: Default::default(),
            ast,
        };

        let module = build_module_signature(&mut context, module)?;
        modules.push(module.clone());
    }

    for module in modules.iter() {
        build_file(&mut context, module.clone())?;
    }

    context.modules = modules;
    println!("Context: {:#?}", context);
    Ok(())
}

fn build_file<'base>(context: &mut TirContext<'base>, module: Rc<RefCell<Module<'base>>>) -> Result<(), TirError<'base>> {
    let uses = module.borrow().ast.get_uses().collect::<Vec<_>>();
    let mut module = module.borrow_mut();

    for use_item in uses {
        build_use(context, &mut module, use_item)?;
    }
    Ok(())
}

fn build_use<'base>(context: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>, use_item: Rc<UseAst<'base>>) -> Result<(), TirError<'base>> {
    if let Some(signature) = context.get_signature(&use_item.import) {
        println!("Module found: {}", module.name);
        if let Some(old_signature) = module.imported_modules.insert(use_item.import.clone(), signature.clone()) {
            return Err(TirError::ModuleAlreadyDefined {
                old_signature,
            });
        }
    } else {
        println!("Module not found: {}", use_item);
        return Err(TirError::ModuleNotFound {
            module: use_item.import.clone().into(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, rc::Rc};

    use crate::{ast::FileAst, file::SourceFile, process_code, tir::signature::{build_module_signature, ModuleSignature}};

    use super::{error::TirError, Module};

    #[test]
    fn find_module_test_1() {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".to_string()], ""));
        let module1 = Module {
            name: "test1".to_string(),
            path: "test1".to_string(),
            imported_modules: Default::default(),
            signatures: Default::default(),
            ast: Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            }),
        };

        let module2 = Module {
            name: "test2".to_string(),
            path: "test1.test2".to_string(),
            imported_modules: Default::default(),
            signatures: Default::default(),
            ast: Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            }),
        };

        let module3 = Module {
            name: "test3".to_string(),
            path: "test1.test2.test3".to_string(),
            imported_modules: Default::default(),
            signatures: Default::default(),
            ast: Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            }),
        };

        let mut context = super::TirContext::default();
        build_module_signature(&mut context, module1).unwrap();
        build_module_signature(&mut context, module2).unwrap();
        build_module_signature(&mut context, module3).unwrap();

        let found_module = context.get_signature("test1.test2.test3");
        if let ModuleSignature::Module(module) = found_module.unwrap().as_ref() {
            assert_eq!(module.borrow().name, "test3");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_signature("test1.test2");
        if let ModuleSignature::Module(module) = found_module.unwrap().as_ref() {
            assert_eq!(module.borrow().name, "test2");
        } else {
            panic!("Expected ModuleSignature::Module");
        }
        
        let found_module = context.get_signature("test1");
        if let ModuleSignature::Module(module) = found_module.unwrap().as_ref() {
            assert_eq!(module.borrow().name, "test1");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_signature("");
        assert!(found_module.is_none());

        let found_module = context.get_signature("abc");
        assert!(found_module.is_none());
    }

    #[test]
    fn module_test() -> Result<(), Box<dyn Error>> {
        let ast_1 = process_code(vec!["source1".to_string()], " class testclass1 {} ")?;
        let ast_2 = process_code(vec!["source2".to_string()], "use source1; use source1.testclass1;")?;
    
        let ast_3 = process_code(vec!["sub".to_string(), "source3".to_string()], "class testclass2 {}")?;
        let ast_4 = process_code(vec!["sub".to_string(), "source4".to_string()], "use source1; use source1.testclass1;")?;
        let ast_5 = process_code(
            vec!["sub".to_string(), "source5".to_string()],
            "use source1; use source1.testclass1;",
        )?;
        let ast_6 = process_code( 
            vec!["sub".to_string(), "source6".to_string()],
            "use sub.source3; use sub.source3.testclass2;",
        )?;
        let ast_7 = process_code(
            vec!["sub".to_string(), "source7".to_string()],
            "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;",
        )?;
        let ast_8 = process_code(vec!["sub".to_string(), "source8".to_string()], "class testclass1 {}")?;
        let ast_9 = process_code(
            vec!["sub".to_string(), "source9".to_string()],
            "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2; use sub.source8; use sub.source8.testclass1;",
        )?;
    
        crate::tir::build(vec![ast_1.into(), ast_2.into(), ast_3.into(), ast_4.into(), ast_5.into(), ast_6.into(), ast_7.into(), ast_8.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn missing_module() -> Result<(), Box<dyn Error>> {
        let ast = process_code(vec!["source1".to_string()], "use missing;")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::ModuleNotFound { module } = error {
            assert_eq!(module.as_str(), "missing");
        } else {
            panic!("Expected TirError::ModuleNotFound");
        }

        Ok(())
    }

    #[test]
    fn dublicated_module() -> Result<(), Box<dyn Error>> {
        let ast_1 = process_code(vec!["source".to_string()], " class testclass {} ")?;
        let ast_2 = process_code(vec!["lib".to_string()], "use source.testclass; use source.testclass;")?;
        let error = crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap_err();

        if let TirError::ModuleAlreadyDefined { old_signature } = error {
            if let ModuleSignature::Class(class) = old_signature.as_ref() {
                assert_eq!(*class.name.fragment(), "testclass");
            } else {
                panic!("Expected ModuleSignature::Class");
            }
        } else {
            panic!("Expected TirError::ModuleAlreadyDefined");
        }
        Ok(())
    }

}
