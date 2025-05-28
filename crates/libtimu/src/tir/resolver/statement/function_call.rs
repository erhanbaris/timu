use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{ast::{BodyStatementAst, ExpressionAst, FunctionCallAst}, nom_tools::{Span, ToRange}, tir::{error::{CustomError, ErrorReport}, module::ModuleRef, resolver::{statement::try_resolve_primitive, TypeLocation}, TirContext, TirError, TypeSignature, TypeValue}};

#[derive(Debug, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum FunctionCallError<'base> {
    #[error("Unsupported argument type in function call: {0}")]
    #[strum(props(code=1))]
    UnsupportedArgumentType(Span<'base>),

    #[error("Function call argument count mismatch: expected {expected}, got {got}")]
    #[strum(props(code=2))]
    FunctionCallArgumentCountMismatch {
        expected: usize,
        expected_source: TypeSignature<'base>,
        got: usize,
    },
}

impl<'base> From<FunctionCallError<'base>> for TirError<'base> {
    fn from(value: FunctionCallError<'base>) -> Self {
        TirError::FunctionCall(Box::new(value))
    }
}

impl CustomError for FunctionCallError<'_> {
    fn get_errors(&self, parent_error_code: &str) -> Vec<crate::tir::error::ErrorReport<'_>> {
        match self {
            FunctionCallError::UnsupportedArgumentType(span) => vec![ErrorReport {
                position: span.to_range(), // Placeholder, should be replaced with actual position
                message: format!("{}", self),
                file: span.extra.file.clone(),
                error_code: self.get_int("code").unwrap().to_string(),
            }],
            FunctionCallError::FunctionCallArgumentCountMismatch { expected_source, .. } => vec![ErrorReport {
                position: expected_source.position.clone(),
                message: format!("{}", self),
                file: expected_source.file.clone(),
                error_code: self.build_error_code(parent_error_code),
            }],
        }
    }
    
    fn get_error_code(&self) -> i64 {
        self.get_int("code").unwrap()
    }
}

impl<'base> BodyStatementAst<'base> {
    pub fn resolve_function_call(context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>, function_call: &FunctionCallAst<'base>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving function call: <u><b>{}(..)</b></u>", function_call.paths.iter().map(|p| *p.fragment()).collect::<Vec<_>>().join("."));
        let module_object = module.upgrade(context).unwrap();
        let parent_location = parent.clone().unwrap();
        let parent_object = context.types.get_from_location(parent_location);
        
        let (parent_function, function_parent) = match parent_object.map(|signature| (signature.value.as_ref(), signature.extra.clone())) {
            Some((TypeValue::Function(function), function_parent)) => (function, function_parent),
            _ => panic!("Parent object is not a function or is missing, {:?}", parent_object),
        };

        let mut callee_object_location = function_parent.clone().expect("Parent object is missing, but this is a bug");
        for (index, path) in function_call.paths.iter().enumerate() {

            match *path.fragment() {
                "this" => continue, // 'this' is handled by the parent object
                path => {
                    if index == 0 {
                        if let Some(argument) = parent_function.arguments.iter().find(|argument| *argument.name.fragment() == path) {
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
                    return Err(FunctionCallError::UnsupportedArgumentType(function_call.call_span.clone()).into());
                }
            };

            arguments.push(argument_location);
        }

        let callee_object = context.types.get_from_location(callee_object_location.clone()).expect("Compiler bug");

        let callee = match callee_object.value.as_ref() {
            TypeValue::Function(function) => function,
            _ => panic!("Expected a function signature, but got {:?}", callee_object.value)
        };
        let return_type = callee.return_type.clone();

        /* Validate parameters */
        if callee.arguments.len() != arguments.len() {
            return Err(FunctionCallError::FunctionCallArgumentCountMismatch {
                expected: callee.arguments.len(),
                got: arguments.len(),
                expected_source: callee_object.clone() 
            }.into());
        }

        for (callee_arg, call_arg) in callee.arguments.iter().zip(arguments.iter()) {
            let callee_argument_signature = context.types.get_from_location(callee_arg.field_type.clone()).unwrap();
        
            let call_argument_signature = context.types.get_from_location(call_arg.clone()).unwrap();
            if !callee_argument_signature.value.compare_skeleton(context, &call_argument_signature.value) {
                panic!("Argument type mismatch: expected {:?}, got {:?}", callee_argument_signature.value, call_argument_signature.value);
            }
        }

        Ok(return_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn func_call_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        abc();
    }
}

func abc(): string {
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc();
        abc();
    }

    func abc(): string {
    }
}

func abc(): string {
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_3() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(a: string): string {
    }
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();

        let ast = process_code(vec!["source".into()], r#"

        class TestClass {
            func init(this): string {
                this.abc("hello", "world");
            }
            func abc(a: string, b: string): string {
            }
        }
        "#)?;
                crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_4() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc();
    }

    func abc(a: string): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_5() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_6() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.nope();
    }

    func abc(): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_7() {
        let ast = process_code(vec!["source".into()], r#"

interface ITest {
    func test(a: string): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhanbaris");
        this.a.test("baris");
        abc();
    }
}

func abc(): TestClass {
}

"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn func_call_8() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("");
    }

    func abc(a: i32): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_9() {
        let ast = process_code(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        this.abc();
        abc();
    }

    func abc(): string {
    }
}

func abc(): string {
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_10() {
        let ast = process_code(vec!["source".into()], r#"
interface ITest {
    func test(a: string): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhanbaris");
        this.a.test("baris");
        abc(abc("erhan"));
    }
}

func abc(a:string): TestClass {
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_11() {
        let ast = process_code(vec!["source".into()], r#"
interface ITest {
    func test(a: string): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhanbaris");
        this.a.test("baris");
        abc(abc("erhan"));
    }
}

func abc(a:string): string {
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }
}
