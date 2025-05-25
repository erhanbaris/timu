use std::{borrow::Cow, rc::Rc};

use ast_signature::{AstSignatureValue, build_module};
pub use context::TirContext;
pub use error::TirError;
use module::{Module, ModuleRef};
use object_signature::TypeValue;
use resolver::{build_file, AstSignatureLocation, ObjectLocation, ResolveAst, TypeLocation};
use signature::{Holder, Signature, SignatureHolder, SignaturePath};

use crate::{ast::{FileAst, FileStatementAst, PrimitiveValue}, file::SourceFile};

mod ast_signature;
mod context;
mod error;
mod module;
mod object_signature;
mod resolver;
mod signature;
mod scope;

pub type AstSignature<'base> = Signature<'base, AstSignatureValue<'base>, ModuleRef<'base>>;
pub type AstSignatureHolder<'base> = SignatureHolder<'base, AstSignatureValue<'base>, AstSignatureLocation, ModuleRef<'base>>;

pub type TypeSignature<'base> = Signature<'base, TypeValue<'base>, TypeLocation>;
pub type TypeSignatureHolder<'base> = SignatureHolder<'base, TypeValue<'base>, TypeLocation, TypeLocation>;

pub type ObjectSignatureHolder<'base> = Holder<Cow<'base, str>, PrimitiveValue<'base>, ObjectLocation>;

pub static BOOL_FALSE_LOCATION: ObjectLocation = ObjectLocation(0);
pub static BOOL_TRUE_LOCATION: ObjectLocation = ObjectLocation(1);

fn build_primitive_types(context: &mut TirContext<'_>) {
    context.types.add_signature(SignaturePath::borrowed("int"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Int), Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("float"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Float), Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("bool"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Bool), Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("string"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::String), Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")), 0..0, None)).unwrap();
    context.types.add_signature(SignaturePath::borrowed("void"), TypeSignature::new(TypeValue::PrimitiveType(object_signature::PrimitiveType::Void), Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")), 0..0, None)).unwrap();
}

fn build_primitive_values(context: &mut TirContext<'_>) {
    context.objects.find_or_insert(&PrimitiveValue::Bool(false));
    context.objects.find_or_insert(&PrimitiveValue::Bool(true));
}

impl<'base> ResolveAst<'base> for FileStatementAst<'base> {
    type Result = TypeLocation;

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        match self {
            FileStatementAst::Class(class_definition_ast) => class_definition_ast.resolve(context, module, parent),
            FileStatementAst::Function(function_definition_ast) => function_definition_ast.resolve(context, module, parent),
            FileStatementAst::Interface(interface_definition_ast) => interface_definition_ast.resolve(context, module, parent),
            FileStatementAst::Extend(extend_definition_ast) => extend_definition_ast.resolve(context, module, parent),
            FileStatementAst::Use(use_ast) => use_ast.resolve(context, module, parent),
        }
    }

    fn finish(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, location: TypeLocation) -> Result<(), TirError<'base>> {
        match self {
            FileStatementAst::Class(class_definition_ast) => class_definition_ast.finish(context, module, location),
            FileStatementAst::Function(function_definition_ast) => function_definition_ast.finish(context, module, location),
            FileStatementAst::Interface(interface_definition_ast) => interface_definition_ast.finish(context, module, location),
            FileStatementAst::Extend(extend_definition_ast) => extend_definition_ast.finish(context, module, location),
            FileStatementAst::Use(use_ast) => use_ast.finish(context, module, location),
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

pub fn build(files: Vec<Rc<FileAst<'_>>>) -> Result<TirContext<'_>, TirError<'_>> {
    let mut context = TirContext::default();
    build_primitive_types(&mut context);
    build_primitive_values(&mut context);

    for ast in files.into_iter() {
        build_module(&mut context, ast)?;
    }

    #[allow(clippy::iter_kv_map)]
    let modules = context.modules.iter().map(|(_, module)| module.get_ref()).collect::<Vec<_>>(); 
    for module in modules.into_iter() {
        build_file(&mut context, module)?;
    }

    Ok(context)
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
        let source_file = Rc::new(SourceFile::new(vec!["<memory>".into()], ""));
        let module1 = Module {
            name: "test1".into(),
            path: "test1".into(),
            ast_imported_modules: Default::default(),
            object_signatures: Default::default(),
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
            object_signatures: Default::default(),
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
            object_signatures: Default::default(),
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
    fn module_test() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".into()], " class testclass1 {} ")?;
        let ast_2 = process_code(vec!["source2".into()], "use source1; use source1.testclass1;")?;

        let ast_3 = process_code(vec!["sub".into(), "source3".into()], "class testclass2 {}")?;
        let ast_4 = process_code(vec!["sub".into(), "source4".into()], "use source1; use source1.testclass1;")?;
        let ast_5 = process_code(vec!["sub".into(), "source5".into()], "use source1; use source1.testclass1;")?;
        let ast_6 = process_code(vec!["sub".into(), "source6".into()], "use sub.source3; use sub.source3.testclass2;")?;
        let ast_7 =
            process_code(vec!["sub".into(), "source7".into()], "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;")?;
        let ast_8 = process_code(vec!["sub".into(), "source8".into()], "class testclass1 {}")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2; use sub.source8; use sub.source8.testclass1 as newtestclass1;",
        )?;

        crate::tir::build(vec![ast_1.into(), ast_2.into(), ast_3.into(), ast_4.into(), ast_5.into(), ast_6.into(), ast_7.into(), ast_8.into(), ast_9.into()])
            .unwrap();
        Ok(())
    }

    #[test]
    fn missing_module() -> Result<(), ()> {
        let ast = process_code(vec!["source1".into()], "use missing;")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::ImportNotFound {
            module,
            position: _,
            source: _,
        } = error
        {
            assert_eq!(module, "missing");
        } else {
            panic!("Expected TirError::ImportNotFound {}", error);
        }

        Ok(())
    }

    #[test]
    fn dublicated_module() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source".into()], " class testclass {} ")?;
        let ast_2 = process_code(vec!["lib".into()], "use source.testclass; use source.testclass;")?;
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

    #[test]
    fn no_dublicated_module() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source".into()], " class testclass {} ")?;
        let ast_2 = process_code(vec!["lib".into()], "use source.testclass as t1; use source.testclass as t2;")?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap();
        Ok(())
    }
}
