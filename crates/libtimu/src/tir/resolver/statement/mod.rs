use core::panic;
use std::borrow::Cow;

use crate::{
    ast::{BodyStatementAst, PrimitiveValue}, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, signature::SignaturePath, TirError}
};

use super::{ResolveAst, TypeLocation};
mod function_call;

pub use function_call::FunctionCallError;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: TypeLocation,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassFunctionSignature<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeLocation,
    pub signature_path: SignaturePath<'base>,
}

impl<'base> ResolveAst<'base> for BodyStatementAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        match self {
            BodyStatementAst::FunctionCall(function_call) => Self::resolve_function_call(context, module, parent, function_call),
            _ => panic!("Unsupported BodyStatementAst variant: {:?}", self),
        }
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: TypeLocation) -> Result<(), TirError<'base>> {
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed("")
    }
}

pub fn try_resolve_primitive<'base>(context: &mut TirContext<'base>, primitive: &PrimitiveValue<'base>, span: &Span<'base>) -> Result<TypeLocation, TirError<'base>> {
    let location = context.types.find_by_value(&TypeValue::PrimitiveType(primitive.to_type()));
    match location {
        Some(location) => Ok(location),
        None => Err(TirError::TypeNotFound { source: span.extra.file.clone(), position: span.to_range() }),
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_ast, process_code, tir::TirError};

    #[test]
    fn missing_type_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicated_function_argument() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class a {} func test(a: a, a: a): a {} ")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::AlreadyDefined {
            position,
            source,
        } = error
        {
            assert_eq!(position, 27..28);
            assert_eq!(source.path().join("/"), "source");
        } else {
            panic!("Expected TirError::AlreadyDefined but got {:?}", error);
        }
        Ok(())
    }

    #[test]
    fn valid_types() -> Result<(), ()> {
        
        let source_1 = process_code(vec!["lib".into()], " class testclass1 {} ")?;
        let source_2 = process_code(vec!["main".into()],
            r#"use lib.testclass1 as test;
    func main(a: test): test {}"#,
        )?;

        let context = process_ast(vec![source_2.into(), source_1.into()])?;
        assert_eq!(context.modules.len(), 2);

        let main_module = context.modules.iter().find(|(name, _)| *name == "main").unwrap();
        let lib_module = context.modules.iter().find(|(name, _)| *name == "lib").unwrap();

        main_module.1.object_signatures.get("main").unwrap();

        assert!(main_module.1.ast_imported_modules.get("testclass1").is_none());
        assert!(main_module.1.ast_imported_modules.get("test").is_some());
        assert!(main_module.1.object_signatures.get("testclass1").is_none());

        lib_module.1.object_signatures.get("testclass1").unwrap();

        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(a: a): test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn not_in_class() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "func test(this): test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
