use core::panic;
use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::{BodyStatementAst, ExpressionAst, FunctionCallAst}, file::SourceFile, nom_tools::Span, tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, signature::SignaturePath, ObjectSignature, TirError}
};

use super::{ObjectLocation, ResolveSignature};

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: ObjectLocation,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassFunctionSignature<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: ObjectLocation,
    pub signature_path: SignaturePath<'base>,
}

impl<'base> ResolveSignature<'base> for BodyStatementAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        match self {
            BodyStatementAst::FunctionCall(function_call) => Self::resolve_function_call(context, module, parent, function_call),
            _ => panic!("Unsupported BodyStatementAst variant: {:?}", self),
        }
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: ObjectLocation) -> Result<(), TirError<'base>> {
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed("")
    }
}

impl<'base> BodyStatementAst<'base> {
    fn resolve_function_call(context: &TirContext<'base>, module: &ModuleRef<'base>, parent: Option<ObjectLocation>, function_call: &FunctionCallAst<'base>) -> Result<ObjectLocation, TirError<'base>> {
        simplelog::debug!("Resolving function call: <u><b>{}</b></u>", "func");
        let module_object = module.upgrade(context).unwrap();

        let parent_location = parent.clone().unwrap();
        let parent_object = context.object_signatures.get_from_location(parent_location);
        
        let (function, function_parent) = match parent_object.map(|signature| (signature.value.as_ref(), signature.extra.clone())) {
            Some((ObjectSignatureValue::Function(function), function_parent)) => (function, function_parent),
            _ => panic!("Parent object is not a function or is missing, {:?}", parent_object),
        };

        let mut callee_object_location = function_parent.clone().expect("Parent object is missing, but this is a bug");
        for (index, path) in function_call.paths.iter().enumerate() {
            match *path.fragment() {
                "this" => continue, // 'this' is handled by the parent object
                path => {
                    if index == 0 {
                        if let Some(argument) = function.arguments.iter().find(|argument| *argument.name.fragment() == path) {
                            callee_object_location = argument.field_type.clone();

                        } else if let Some(data) = module_object.object_signatures.get(path) {
                            callee_object_location = data.clone();

                        } else {
                            panic!("Function argument or object not found: {}", path);
                        }
                    } else {
                        match context.object_signatures.get_from_location(callee_object_location.clone()).map(|signature| signature.value.as_ref()) {
                            Some(ObjectSignatureValue::Class(class)) => {
                                if let Some(field) = class.fields.get(path) {
                                    callee_object_location = field.clone();
                                } else {
                                    panic!("Field not found in class: {}", path);
                                }
                            },
                            Some(ObjectSignatureValue::Function(function)) => {
                                if let Some(argument) = function.arguments.iter().find(|argument| *argument.name.fragment() == path) {
                                    callee_object_location = argument.field_type.clone();
                                } else {
                                    panic!("Function argument not found: {}", path);
                                }
                            },
                            _ => panic!("Object location is not a class or function, but this is a bug"),
                        }
                    }
                }
            }
        }

        let callee_object = context.object_signatures.get_from_location(callee_object_location.clone()).unwrap();

        let callee = match callee_object.value.as_ref() {
            ObjectSignatureValue::Function(function) => function,
            _ => panic!("Expected a function signature, but got {:?}", callee_object.value)
        };

        /* Parse arguments */
        if callee.arguments.len() != function_call.arguments.len() {
            panic!("Function call argument count mismatch: expected {}, got {}", callee.arguments.len(), function_call.arguments.len());
        }
        
        for (index, argument) in function_call.arguments.iter().enumerate() {
            let callee_function_argument = context.object_signatures.get_from_location(callee.arguments[index].field_type.clone()).unwrap();

            let argument_location = match argument {
                ExpressionAst::FunctionCall(func_call) => Self::resolve_function_call(context, module, parent.clone(), func_call)?,
                ExpressionAst::Primitive(_) => ObjectLocation::UNDEFINED,
                _ => {
                    panic!("Unsupported argument type: {:?}", argument);
                }
            };

            let pass_argument = context.object_signatures.get_from_location(argument_location.clone()).unwrap();

            if callee_function_argument.value.compare_skeleton(&pass_argument.value) {
                panic!("Argument type mismatch: expected {:?}, got {:?}", callee_function_argument.value, pass_argument.value);
            }
        }

        let _signature = ObjectSignature::new(
            ObjectSignatureValue::FunctionCall(callee_object_location),
            Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")),
            0..0,
            parent,
        );
        
        Ok(ObjectLocation::UNDEFINED)
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
