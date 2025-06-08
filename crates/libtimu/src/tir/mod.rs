use std::rc::Rc;

use ast_signature::{build_module, AstSignatureValue};
pub use context::TirContext;
pub use error::TirError;
use module::{Module, ModuleRef};
pub use object_signature::{PrimitiveType, TypeValue};
use resolver::{build_file, AstSignatureLocation, ObjectLocation, ResolveAst, TypeLocation};
use scope::ScopeLocation;
use signature::{Signature, SignatureHolder, SignaturePath};

use crate::{ast::{FileAst, FileStatementAst}, file::SourceFile};

mod ast_signature;
mod context;
//pub mod error;
pub mod error;
mod module;
mod object_signature;
mod resolver;
mod signature;
mod scope;


pub type AstSignature<'base> = Signature<AstSignatureValue<'base>, ModuleRef<'base>>;
pub type AstSignatureHolder<'base> = SignatureHolder<'base, AstSignatureValue<'base>, AstSignatureLocation, ModuleRef<'base>>;

pub type TypeSignature<'base> = Signature<TypeValue<'base>, TypeLocation>;
pub type TypeSignatureHolder<'base> = SignatureHolder<'base, TypeValue<'base>, TypeLocation, TypeLocation>;

pub static BOOL_FALSE_LOCATION: ObjectLocation = ObjectLocation(0);
pub static BOOL_TRUE_LOCATION: ObjectLocation = ObjectLocation(1);

fn build_primitive_types(context: &mut TirContext<'_>) {
    context.types.add_signature(SignaturePath::borrowed("i8"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::I8), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("u8"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::U8), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("i16"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::I16), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("u16"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::U16), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("i32"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::I32), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("u32"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::U32), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("i64"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::I64), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("u64"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::U64), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("float"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Float), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("bool"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Bool), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("string"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::String), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("void"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Void), SourceFile::new(vec!["<standart>".into()], "<native-code>".to_string()), 0..0, None)).unwrap();
}

impl<'base> ResolveAst<'base> for FileStatementAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        match self {
            FileStatementAst::Class(class_definition_ast) => class_definition_ast.resolve(context, scope_location),
            FileStatementAst::Function(function_definition_ast) => function_definition_ast.resolve(context, scope_location),
            FileStatementAst::Interface(interface_definition_ast) => interface_definition_ast.resolve(context, scope_location),
            FileStatementAst::Extend(extend_definition_ast) => extend_definition_ast.resolve(context, scope_location),
            FileStatementAst::Use(use_ast) => use_ast.resolve(context, scope_location),
        }
    }

    fn finish(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<(), TirError> {
        match self {
            FileStatementAst::Class(class_definition_ast) => class_definition_ast.finish(context, scope_location),
            FileStatementAst::Function(function_definition_ast) => function_definition_ast.finish(context, scope_location),
            FileStatementAst::Interface(interface_definition_ast) => interface_definition_ast.finish(context, scope_location),
            FileStatementAst::Extend(extend_definition_ast) => extend_definition_ast.finish(context, scope_location),
            FileStatementAst::Use(use_ast) => use_ast.finish(context, scope_location),
        }
    }

    fn name(&self) -> std::borrow::Cow<'base, str> {
        match self {
            FileStatementAst::Class(class_definition_ast) => class_definition_ast.name(),
            FileStatementAst::Function(function_definition_ast) => function_definition_ast.name(),
            FileStatementAst::Interface(interface_definition_ast) => interface_definition_ast.name(),
            FileStatementAst::Extend(extend_definition_ast) => extend_definition_ast.name(),
            FileStatementAst::Use(use_ast) => use_ast.name(),
        }
    }
}

pub fn build(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, TirError> {
    //let mut has_error = false;
    let mut context: TirContext<'_> = TirContext::default();

    /*simplelog::debug!("Adding base module");
    let base_module = Module::phantom("<root>".into(), "<root>".into(), Rc::new(SourceFile::new(vec!["<memory>".into()], "")));
    build_module_signature(&mut context, base_module)?;*/
    build_primitive_types(&mut context);

    for ast in files.into_iter() {
        if let Err(error) = build_module(&mut context, ast.clone()) {
            if !context.errors.is_empty() {
                return Err(TirError::multiple_errors(context.errors.clone()));
            } else {
                return Err(error);
            }
            //has_error = true;
        }
    }

    #[allow(clippy::iter_kv_map)]
    let modules = context.modules.iter().map(|(_, module)| module.get_ref()).collect::<Vec<_>>(); 
    for module in modules.into_iter() {
        if let Err(error) = build_file(&mut context, module) {
            if !context.errors.is_empty() {
                return Err(TirError::multiple_errors(context.errors.clone()));
            } else {
                return Err(error);
            }
            //has_error = true;
        }
    }

    if !context.errors.is_empty() {
        return Err(TirError::multiple_errors(context.errors.clone()));
    }

    Ok(context)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        ast::FileAst, file::SourceFile, nom_tools::State, process_code, tir::ast_signature::{build_module_signature, AstSignatureValue}
    };

    use super::{Module, TirError};

    #[test]
    fn find_module_test_1() {
        let source_file = SourceFile::new(vec!["<memory>".into()], "".to_string());
        let module1 = Module {
            name: "test1".into(),
            path: "test1".into(),
            ast_imported_modules: Default::default(),
            types: Default::default(),
            ast_signatures: Default::default(),
            file: source_file.clone(),
            modules: Default::default(),
            ast: Some(Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            })),
        };

        let module2 = Module {
            name: "test2".into(),
            path: "test1.test2".into(),
            ast_imported_modules: Default::default(),
            types: Default::default(),
            file: source_file.clone(),
            ast_signatures: Default::default(),
            modules: Default::default(),
            ast: Some(Rc::new(FileAst {
                file: source_file.clone(),
                statements: vec![],
            })),
        };

        let module3 = Module {
            name: "test3".into(),
            path: "test1.test2.test3".into(),
            ast_imported_modules: Default::default(),
            types: Default::default(),
            file: source_file.clone(),
            ast_signatures: Default::default(),
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
            assert_eq!(module.as_ref(), "test1.test2.test3");
            assert_eq!(context.modules.get(module.as_ref()).unwrap().name, "test3");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("test1.test2");
        if let AstSignatureValue::Module(module) = &found_module.unwrap().value {
            assert_eq!(module.as_ref(), "test1.test2");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("test1");
        if let AstSignatureValue::Module(module) = &found_module.unwrap().value {
            assert_eq!(module.as_ref(), "test1");
            assert_eq!(context.modules.get(module.as_ref()).unwrap().name, "test1");
        } else {
            panic!("Expected ModuleSignature::Module");
        }

        let found_module = context.get_ast_signature("");
        assert!(found_module.is_none());

        let found_module = context.get_ast_signature("abc");
        assert!(found_module.is_none());
    }

    #[test]
    fn module_test() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source1".into()], " class testclass1 {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["source2".into()], "use source1; use source1.testclass1;".to_string()));
        let state_3 = State::new(SourceFile::new(vec!["sub".into(), "source3".into()], "class testclass2 {}".to_string()));
        let state_4 = State::new(SourceFile::new(vec!["sub".into(), "source4".into()], "use source1; use source1.testclass1;".to_string()));
        let state_5 = State::new(SourceFile::new(vec!["sub".into(), "source5".into()], "use source1; use source1.testclass1;".to_string()));
        let state_6 = State::new(SourceFile::new(vec!["sub".into(), "source6".into()], "use sub.source3; use sub.source3.testclass2;".to_string()));
        let state_7 = State::new(SourceFile::new(vec!["sub".into(), "source7".into()], "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;".to_string()));
        let state_8 = State::new(SourceFile::new(vec!["sub".into(), "source8".into()], "class testclass1 {}".to_string()));
        let state_9 = State::new(SourceFile::new(vec!["sub".into(), "source9".into()], "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2; use sub.source8; use sub.source8.testclass1 as newtestclass1;".to_string()));

        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        let ast_3 = process_code(&state_3)?;
        let ast_4 = process_code(&state_4)?;
        let ast_5 = process_code(&state_5)?;
        let ast_6 = process_code(&state_6)?;
        let ast_7 = process_code(&state_7)?;
        let ast_8 = process_code(&state_8)?;
        let ast_9 = process_code(&state_9)?;

        crate::tir::build(vec![ast_1.into(), ast_2.into(), ast_3.into(), ast_4.into(), ast_5.into(), ast_6.into(), ast_7.into(), ast_8.into(), ast_9.into()])
            .unwrap();
        Ok(())
    }

    #[test]
    fn missing_module() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source1".into()], "use missing;".to_string()));
        let ast = process_code(&state)?;
        let _error = crate::tir::build(vec![ast.into()]).unwrap_err();

        // todo: fix this test
        /*if let TirError::ImportNotFound(error) = error
        {
            assert_eq!(error.module, "missing");
        } else {
            panic!("Expected TirError::ImportNotFound {}", error);
        }*/

        Ok(())
    }

    #[test]
    fn dublicated_module() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source.testclass; use source.testclass;".to_string()));
        
        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        let _error = crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap_err();

        /*
        todo: fix this test
        if let TirError::ModuleAlreadyImported(error) = error
        {
            assert_eq!(error.old_position, SourceSpan::from(7..16));
            assert_eq!(error.new_position, SourceSpan::from(26..42));
        } else {
            panic!("Expected TirError::AstModuleAlreadyDefined");
        } */
        Ok(())
    }

    #[test]
    fn no_dublicated_module() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source.testclass as t1; use source.testclass as t2;".to_string()));

        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn no_import_works_fine() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "func abc(a: source.testclass): source.testclass { }".to_string()));

        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap();
        Ok(())
    }
}
