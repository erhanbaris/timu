use core::panic;
use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::{BodyStatementAst, ExpressionAst, FunctionCallAst, PrimitiveValue}, file::SourceFile, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, signature::SignaturePath, TirError, TypeSignature}
};

use super::{ResolveAst, TypeLocation};

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
    type Result = TypeLocation;
    
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
    let location = context.objects.find_or_insert(primitive);
    Ok(context.types.add_signature(SignaturePath::owned(context.create_tmp_type()), TypeSignature::new(TypeValue::Object(location), span.extra.file.clone(), span.to_range(), None)).unwrap())
}

impl<'base> BodyStatementAst<'base> {
    fn resolve_function_call(context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>, function_call: &FunctionCallAst<'base>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving function call: <u><b>{}(..)</b></u>", function_call.paths.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join("."));
        let module_object = module.upgrade(context).unwrap();

        let parent_location = parent.clone().unwrap();
        let parent_object = context.types.get_from_location(parent_location);
        
        let (function, function_parent) = match parent_object.map(|signature| (signature.value.as_ref(), signature.extra.clone())) {
            Some((TypeValue::Function(function), function_parent)) => (function, function_parent),
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
                        match context.types.get_from_location(callee_object_location.clone()).map(|signature| signature.value.as_ref()) {
                            Some(TypeValue::Class(class)) => {
                                if let Some(field) = class.fields.get(path) {
                                    callee_object_location = field.clone();
                                } else {
                                    panic!("Field not found in class: {}", path);
                                }
                            },
                            Some(TypeValue::Function(function)) => {
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

        
        let mut arguments = Vec::new();
        for argument in function_call.arguments.iter() {
            let argument_location = match argument {
                ExpressionAst::FunctionCall(func_call) => Self::resolve_function_call(context, module, parent.clone(), func_call)?,
                ExpressionAst::Primitive { span, value } => try_resolve_primitive(context, value, span)?,
                _ => {
                    panic!("Unsupported argument type: {:?}", argument);
                }
            };

            arguments.push(argument_location);
        }

        let callee_object = context.types.get_from_location(callee_object_location.clone()).unwrap();
        let callee = match callee_object.value.as_ref() {
            TypeValue::Function(function) => function,
            _ => panic!("Expected a function signature, but got {:?}", callee_object.value)
        };

        /* Validate parameters */
        if callee.arguments.len() != arguments.len() {
            panic!("Function call argument count mismatch: expected {}, got {}", callee.arguments.len(), function_call.arguments.len());
        }

        for (callee_arg, call_arg) in callee.arguments.iter().zip(arguments.iter()) {
            let callee_argument_signature = context.types.get_from_location(callee_arg.field_type.clone()).unwrap();
        
            let call_argument_signature = context.types.get_from_location(call_arg.clone()).unwrap();
            if !callee_argument_signature.value.compare_skeleton(context, &call_argument_signature.value) {
                panic!("Argument type mismatch: expected {:?}, got {:?}", callee_argument_signature.value, call_argument_signature.value);
            }
        }

        let _signature = TypeSignature::new(
            TypeValue::FunctionCall { callee: callee_object_location, arguments },
            Rc::new(SourceFile::new(vec!["<standart>".into()], "<native-code>")),
            0..0,
            parent,
        );

        //context.publish_object_location("asdasdasd", signature);
        
        Ok(TypeLocation::UNDEFINED)
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
