use miette::Diagnostic;
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{ast::{BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallType}, nom_tools::SpanInfo, tir::{error::{CustomError, ErrorReport}, resolver::{statement::try_resolve_primitive, ResolverError, TypeLocation}, scope::ScopeLocation, TirContext, TirError, TypeValue}};

#[derive(Debug, Diagnostic, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum FunctionCallError {
    #[error("Unsupported argument type in function call")]
    UnsupportedArgumentType(SpanInfo),

    #[error("Function call argument count mismatch: expected {expected}, got {got}")]
    FunctionCallArgumentCountMismatch {
        expected: usize,
        expected_source: SpanInfo,
        got: usize,
    },
}

impl From<FunctionCallError> for TirError {
    fn from(value: FunctionCallError) -> Self {
        ResolverError::FunctionCall(Box::new(value)).into()
    }
}

impl CustomError for FunctionCallError {
    fn get_errors(&self, parent_error_code: &str) -> Vec<crate::tir::error::ErrorReport> {
        match self {
            FunctionCallError::UnsupportedArgumentType(span) => vec![ErrorReport {
                position: span.position.clone(), // Placeholder, should be replaced with actual position
                message: format!("{}", self),
                file: span.file.clone(),
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
    pub fn resolve_function_call(context: &mut TirContext<'base>, scope_location: ScopeLocation, function_call: &FunctionCallAst<'base>) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving function call: <u><b>{}(..)</b></u>", function_call.path.call());
        let (scope, paths, mut callee_object_location) = match &function_call.path {
            FunctionCallType::Direct(paths) => (context.get_scope(scope_location).unwrap(), paths, TypeLocation::UNDEFINED),
            FunctionCallType::This(paths) => {
                let scope =  context.get_scope(scope_location).unwrap();
                let parent_scope = context.get_scope(scope.parent_scope.unwrap()).expect("Parent scope not found, but this is a bug");
                (parent_scope, paths, parent_scope.current_type)
            }
        };

        for (index, path) in paths.iter().enumerate() {
            let path = *path.fragment();

            if index == 0 {
                if let Some(argument) = scope.get_variable(context, path) {
                    callee_object_location = argument
                } else {
                    panic!("Function argument or object not found: '{}'", path);
                }
            } else {
                match context.types.get_from_location(callee_object_location).map(|signature| signature.value.as_ref()) {
                    Some(TypeValue::Class(class)) => {
                        if let Some(field) = class.fields.get(path) {
                            callee_object_location = *field;
                        } else {
                            panic!("Field not found in class: {}", path);
                        }
                    },
                    Some(TypeValue::Function(function)) => {
                        if let Some(argument) = function.arguments.iter().find(|argument| *argument.name.fragment() == path) {
                            callee_object_location = argument.field_type;
                        } else {
                            panic!("Function argument not found: {}", path);
                        }
                    },
                    value => panic!("Object location is not a class or function, but this is a bug, '{path}, {:?}", value),
                }
            }
        }

        let mut arguments = Vec::new();
        for argument in function_call.arguments.iter() {
            let argument_location = match argument {
                ExpressionAst::FunctionCall(func_call) => Self::resolve_function_call(context, scope_location, func_call)?,
                ExpressionAst::Primitive { span, value } => try_resolve_primitive(context, value, span)?,
                _ => {
                    return Err(FunctionCallError::UnsupportedArgumentType(function_call.call_span.clone().into()).into());
                }
            };

            arguments.push(argument_location);
        }

        let callee_object = context.types.get_from_location(callee_object_location).expect("Compiler bug");

        let callee = match callee_object.value.as_ref() {
            TypeValue::Function(function) => function,
            _ => panic!("Expected a function signature, but got {:?}", callee_object.value)
        };
        let return_type = callee.return_type;

        /* Validate parameters */
        if callee.arguments.len() != arguments.len() {
            return Err(FunctionCallError::FunctionCallArgumentCountMismatch {
                expected: callee.arguments.len(),
                got: arguments.len(),
                expected_source: SpanInfo::new(callee_object.position.clone(), callee_object.file.clone()) 
            }.into());
        }

        for (callee_arg, call_arg) in callee.arguments.iter().zip(arguments.iter()) {
            let callee_argument_signature = context.types.get_from_location(callee_arg.field_type).unwrap();
        
            let call_argument_signature = context.types.get_from_location(*call_arg).unwrap();
            if !callee_argument_signature.value.compare_skeleton(context, &call_argument_signature.value) {
                panic!("Argument type mismatch: expected {:?}, got {:?}", callee_argument_signature.value, call_argument_signature.value);
            }
        }

        Ok(return_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code};

    #[test]
    fn func_call_1() -> miette::Result<()> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        abc();
    }
}

func abc(): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_2() -> miette::Result<()> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

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
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_3() -> miette::Result<()> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(a: string): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();

        let state = State::new(SourceFile::new(vec!["source".into()], r#"

        class TestClass {
            func init(this): string {
                this.abc("hello", "world");
            }
            func abc(a: string, b: string): string {
            }
        }
        "#.to_string()));
        let ast = process_code(&state).unwrap();
                crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_4() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc();
    }

    func abc(a: string): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_5() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_6() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.nope();
    }

    func abc(): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_7() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

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

"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn func_call_8() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("");
    }

    func abc(a: i32): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_9() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
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
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_10() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
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
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_11() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
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
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }

    #[test]
    fn func_call_12() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
func abc(a:string): string {
    abc(abc("erhan"));
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }
}
