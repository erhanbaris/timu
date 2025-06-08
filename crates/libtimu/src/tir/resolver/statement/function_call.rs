use std::ops::Range;

use libtimu_macros::TimuError;
use libtimu_macros_core::SourceCode;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{ast::{BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallType}, nom_tools::{SpanInfo, ToRange}, tir::{object_signature::GetItem, resolver::{statement::try_resolve_primitive, ResolverError, TypeLocation}, scope::ScopeLocation, TirContext, TirError, TypeValue}};

#[derive(thiserror::Error, TimuError, Debug, Clone, PartialEq)]
#[error("")]
pub struct TypeWithSpan {
    pub ty: String,

    /// Span of expected type
    #[label("this has `{ty}`")]
    pub at: Range<usize>,

    #[source_code]
    pub source_code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Function call argument count mismatch: expected {expected_size}, got {got_size}")]
#[diagnostic(code("timu::error::function_call_argument_count_mismatch"))]
pub struct FunctionCallArgumentCountMismatch {
    pub expected_size: usize,
    pub got_size: usize,

    #[reference]
    pub expected: TypeWithSpan,

    #[reference]
    pub got: TypeWithSpan,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("`{path}` not valid call path. It is not class or pointer")]
#[diagnostic(code("timu::error::call_path_not_valid"))]
pub struct CallPathNotValid {
    pub path: String,

    #[label("this path not valid")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("expected `{expected}` type, got `{got}`")]
pub struct ArgumentTypeMismatch {
    #[reference]
    pub expected: TypeWithSpan,

    #[reference]
    pub got: TypeWithSpan,
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum FunctionCallError {
    #[error("Unsupported argument type in function call")]
    UnsupportedArgumentType(SpanInfo),

    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionCallArgumentCountMismatch(Box<FunctionCallArgumentCountMismatch>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    CallPathNotValid(Box<CallPathNotValid>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ArgumentTypeMismatch(Box<ArgumentTypeMismatch>),
}

impl From<FunctionCallArgumentCountMismatch> for FunctionCallError {
    fn from(value: FunctionCallArgumentCountMismatch) -> Self {
        FunctionCallError::FunctionCallArgumentCountMismatch(Box::new(value))
    }
}

impl From<FunctionCallError> for TirError {
    fn from(value: FunctionCallError) -> Self {
        ResolverError::FunctionCall(Box::new(value)).into()
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

        for (index, span) in paths.iter().enumerate() {
            let path = span.text;

            if index == 0 {
                if let Some(argument) = scope.get_variable(context, path) {
                    callee_object_location = argument
                } else {
                    panic!("Function argument or object not found: '{}'", path);
                }
            } else {
                callee_object_location = match context
                        .types
                        .get_from_location(callee_object_location)
                        .map(|signature| signature.value.as_ref())
                        .and_then(|value| value.get_item_location(context, path)) {
                        Some(type_location) => type_location,
                    _ => return Err(FunctionCallError::CallPathNotValid(CallPathNotValid {
                        path: path.to_string(),
                        position: span.to_range(),
                        code: span.state.file.clone().into()
                    }.into()).into()),
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
            return Err(FunctionCallError::FunctionCallArgumentCountMismatch(FunctionCallArgumentCountMismatch {
                expected_size: callee.arguments.len(),
                got_size: arguments.len(),
                expected: TypeWithSpan {
                        ty: callee.ast.arguments_span.text.to_string(),
                        at: callee.ast.arguments_span.to_range(),
                        source_code: callee.ast.arguments_span.state.file.clone().into()
                    },
                got: TypeWithSpan {
                    ty: function_call.arguments_span.text.to_string(),
                    at: function_call.arguments_span.position.clone(),
                    source_code: function_call.arguments_span.state.file.clone().into()
                }

            }.into()).into());
        }

        for (callee_arg, call_arg) in callee.arguments.iter().zip(arguments.iter()) {
            let callee_argument_signature = context.types.get_from_location(callee_arg.field_type).unwrap();
        
            let call_argument_signature = context.types.get_from_location(*call_arg).unwrap();
            if !callee_argument_signature.value.compare_skeleton(context, &call_argument_signature.value) {
                return Err(FunctionCallError::ArgumentTypeMismatch(ArgumentTypeMismatch {
                    expected: TypeWithSpan {
                        ty: callee_arg.field_type_span.text.to_string(),
                        at: callee_arg.field_type_span.to_range(),
                        source_code: callee_arg.field_type_span.state.file.clone().into()
                    },
                    got: TypeWithSpan {
                        ty: call_argument_signature.value.get_name().to_string(),
                        at: call_argument_signature.position.clone(),
                        source_code: call_argument_signature.file.clone().into()
                    }
                }.into()).into());
            }
        }

        Ok(return_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};

    #[test]
    fn func_call_1() -> Result<(), TirError> {
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
    fn func_call_2() -> Result<(), TirError> {
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
    fn func_call_3() -> Result<(), TirError> {
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

    #[test]
    fn func_call_13() {
        let state1 = State::new(SourceFile::new(vec!["lib".into()], r#"
        interface ITest {
            func test(a: string): string;
            a: main.TestClass;
        }
        func abc(a:string): string {
        }

        "#.to_string()));


        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
        use lib.ITest;

        extend TestClass: ITest {
            func test(a: string): string { }
            a: main.TestClass;
        }

        class TestClass {
            func init(this): string {
                lib.abc();
            }
        }

        "#.to_string()));
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();

        process_ast(vec![ast1.into(), ast2.into()]).unwrap_err();
    }

    #[test]
    fn func_call_14() {
        let state1 = State::new(SourceFile::new(vec!["lib".into()], r#"
        interface ITest {
            func test(a: string): string;
            a: main.TestClass;
        }
        func abc(a:string): string {
        }

        "#.to_string()));


        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
        use lib.ITest;
        use lib.abc;

        extend TestClass: ITest {
            func test(a: string): string { }
            a: main.TestClass;
        }

        class TestClass {
            func init(this): string {
                lib.abc("hello");
                abc("hello");
            }
        }

        "#.to_string()));
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();

        process_ast(vec![ast1.into(), ast2.into()]).unwrap();
    }
}
