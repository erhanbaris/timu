use std::{collections::HashMap, rc::Rc};

use ast_signature::{AstSignatureValue, build_module};
use context::TirContext;
pub use error::TirError;
use module::Module;
use object_signature::ObjectSignatureValue;
use resolver::build_file;
use signature::{Signature, SignatureHolder};

use crate::ast::FileAst;

mod ast_signature;
mod context;
mod error;
mod module;
mod object_signature;
mod resolver;
mod signature;

pub type AstSignature<'base> = Signature<'base, AstSignatureValue<'base>>;
pub type AstSignatureHolder<'base> = SignatureHolder<'base, AstSignatureValue<'base>>;

pub type ObjectSignature<'base> = Signature<'base, ObjectSignatureValue<'base>>;
pub type ObjectSignatureHolder<'base> = SignatureHolder<'base, ObjectSignatureValue<'base>>;

pub fn build(files: Vec<Rc<FileAst>>) -> Result<(), TirError> {
    let mut context: TirContext = TirContext::default();
    let mut modules = HashMap::new();

    for ast in files.into_iter() {
        build_module(&mut context, ast, &mut modules)?;
    }

    for (_, module) in modules.iter() {
        build_file(&mut context, module.clone())?;
    }

    context.modules = modules;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        ast::FileAst,
        file::SourceFile,
        process_code,
        tir::ast_signature::{AstSignatureValue, build_module_signature},
    };

    use super::{Module, error::TirError};

    #[test]
    fn find_module_test_1() {
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".to_string()], ""));
        let module1 = Module {
            name: "test1".to_string(),
            path: "test1".to_string(),
            imported_modules: Default::default(),
            ast_signatures: Default::default(),
            object_signatures: Default::default(),
            file: source_file.clone(),
            modules: Default::default(),
            ast: Some(Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            })),
        };

        let module2 = Module {
            name: "test2".to_string(),
            path: "test1.test2".to_string(),
            imported_modules: Default::default(),
            ast_signatures: Default::default(),
            object_signatures: Default::default(),
            file: source_file.clone(),
            modules: Default::default(),
            ast: Some(Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            })),
        };

        let module3 = Module {
            name: "test3".to_string(),
            path: "test1.test2.test3".to_string(),
            imported_modules: Default::default(),
            ast_signatures: Default::default(),
            object_signatures: Default::default(),
            file: source_file.clone(),
            modules: Default::default(),
            ast: Some(Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            })),
        };

        let mut context = super::TirContext::default();
        build_module_signature(&mut context, module1).unwrap();
        build_module_signature(&mut context, module2).unwrap();
        build_module_signature(&mut context, module3).unwrap();

        let found_module = context.get_ast_signature("test1.test2.test3");
        if let AstSignatureValue::Module(module) = &found_module.unwrap().value {
            assert_eq!(module.borrow().name, "test3");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("test1.test2");
        if let AstSignatureValue::Module(module) = &found_module.unwrap().value {
            assert_eq!(module.borrow().name, "test2");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("test1");
        if let AstSignatureValue::Module(module) = &found_module.unwrap().value {
            assert_eq!(module.borrow().name, "test1");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("");
        assert!(found_module.is_none());

        let found_module = context.get_ast_signature("abc");
        assert!(found_module.is_none());
    }

    #[test]
    fn module_test() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".to_string()], " class testclass1 {} ")?;
        let ast_2 = process_code(vec!["source2".to_string()], "use source1; use source1.testclass1;")?;

        let ast_3 = process_code(vec!["sub".to_string(), "source3".to_string()], "class testclass2 {}")?;
        let ast_4 = process_code(vec!["sub".to_string(), "source4".to_string()], "use source1; use source1.testclass1;")?;
        let ast_5 = process_code(vec!["sub".to_string(), "source5".to_string()], "use source1; use source1.testclass1;")?;
        let ast_6 = process_code(vec!["sub".to_string(), "source6".to_string()], "use sub.source3; use sub.source3.testclass2;")?;
        let ast_7 =
            process_code(vec!["sub".to_string(), "source7".to_string()], "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;")?;
        let ast_8 = process_code(vec!["sub".to_string(), "source8".to_string()], "class testclass1 {}")?;
        let ast_9 = process_code(
            vec!["sub".to_string(), "source9".to_string()],
            "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2; use sub.source8; use sub.source8.testclass1 as newtestclass1;",
        )?;

        crate::tir::build(vec![ast_1.into(), ast_2.into(), ast_3.into(), ast_4.into(), ast_5.into(), ast_6.into(), ast_7.into(), ast_8.into(), ast_9.into()])
            .unwrap();
        Ok(())
    }

    #[test]
    fn missing_module() -> Result<(), ()> {
        let ast = process_code(vec!["source1".to_string()], "use missing;")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::ModuleNotFound {
            module,
            position: _,
            source: _,
        } = error
        {
            assert_eq!(module, "missing");
        } else {
            panic!("Expected TirError::ModuleNotFound");
        }

        Ok(())
    }

    #[test]
    fn dublicated_module() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source".to_string()], " class testclass {} ")?;
        let ast_2 = process_code(vec!["lib".to_string()], "use source.testclass; use source.testclass;")?;
        let error = crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap_err();

        if let TirError::AstModuleAlreadyDefined {
            source: _,
            position,
        } = error
        {
            assert_eq!(position, 26..42);
        } else {
            panic!("Expected TirError::AstModuleAlreadyDefined");
        }
        Ok(())
    }
}
