//! Function call resolution and validation for the Timu TIR system.
//!
//! This module handles the semantic analysis and type checking of function calls within
//! the Timu Type Intermediate Representation (TIR). It performs comprehensive validation
//! of function calls including argument count verification, argument type checking, and
//! method resolution for object-oriented calls.
//!
//! # Function Call Types
//!
//! The module handles several types of function calls:
//!
//! ## Direct Function Calls
//! - **Module functions**: `moduleName.functionName(args)`
//! - **Local functions**: `functionName(args)`
//! - **Imported functions**: `importedFunction(args)`
//!
//! ## Method Calls
//! - **Instance methods**: `object.methodName(args)`
//! - **This methods**: `this.methodName(args)` within class context
//! - **Chained calls**: `object.field.method(args)`
//!
//! ## Interface Methods
//! - **Extension methods**: Methods added via `extend` declarations
//! - **Interface requirements**: Methods defined in interface contracts
//!
//! # Resolution Process
//!
//! Function call resolution follows these phases:
//!
//! 1. **Path Resolution**: Resolve the call path (object.method or direct function)
//! 2. **Argument Analysis**: Extract and type-check all arguments
//! 3. **Signature Matching**: Match call signature with function definition
//! 4. **Type Validation**: Ensure argument types match parameter types
//! 5. **Return Type**: Determine the return type for the expression
//!
//! # Error Handling
//!
//! The module provides detailed error reporting for common function call issues:
//! - **Argument count mismatches**: Too many or too few arguments
//! - **Type mismatches**: Arguments don't match expected parameter types
//! - **Invalid call paths**: Calling methods on non-object types
//! - **Unsupported expressions**: Complex expressions not yet supported as arguments
//!
//! # Integration with Type System
//!
//! Function call resolution integrates deeply with the TIR type system:
//! - **Scope management**: Resolves variables and this references
//! - **Type signatures**: Validates against function signatures
//! - **Type inference**: Contributes to overall type inference
//! - **Error propagation**: Provides rich diagnostic information

use std::ops::Range;

use libtimu_macros::TimuError;
use libtimu_macros_core::SourceCode;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{ast::{BodyStatementAst, ExpressionAst, FunctionCallAst, FunctionCallType}, nom_tools::ToRange, tir::{object_signature::GetItem, resolver::{function::{find_class_location, FunctionResolveError}, statement::try_resolve_primitive, ResolverError, TypeLocation}, scope::{ScopeLocation, TypeVariableInformation, VariableInformation}, TirContext, TirError, TypeValue}};

#[derive(thiserror::Error, TimuError, Debug, Clone, PartialEq)]
#[error("{ty}")]
pub struct TypeWithSpan {
    pub ty: String,

    /// Span of expected type
    #[label("{ty}")]
    pub at: Range<usize>,

    #[source_code]
    pub source_code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Function `{function_name}` expects {expected_size} argument{expected_plural}, but {got_size} {got_plural} provided")]
#[diagnostic(
    code("timu::error::function_call_argument_count_mismatch"),
    help("Check the function definition and provide the correct number of arguments")
)]
pub struct FunctionCallArgumentCountMismatch {
    pub function_name: String,
    pub expected_size: usize,
    pub got_size: usize,
    pub expected_plural: String,
    pub got_plural: String,

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

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Unsupported argument type in function call")]
pub struct UnsupportedArgumentType {
    #[label("Unsupported argument type")]
    pub position: Range<usize>,

    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum FunctionCallError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    UnsupportedArgumentType(Box<UnsupportedArgumentType>),

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
    /// Analyzes and resolves type information for function call arguments
    /// 
    /// This method processes individual arguments in function calls, determining their types
    /// and validating that they can be used as function arguments. It handles various
    /// expression types including function calls, primitives, identifiers, and `this` references.
    /// 
    /// # Arguments
    /// * `context` - Mutable TIR context for type resolution
    /// * `scope_location` - Current scope location for variable resolution
    /// * `function_scope_location` - Function-specific scope for parameter resolution
    /// * `function_call` - The containing function call for error reporting
    /// * `argument` - The expression argument to analyze
    /// 
    /// # Returns
    /// * `Ok(TypeVariableInformation)` - Successfully resolved argument type information
    /// * `Err(TirError)` - Type resolution error or unsupported argument type
    /// 
    /// # Supported Argument Types
    /// - **Function calls**: Nested function call expressions
    /// - **Primitives**: String, number, and boolean literals
    /// - **Identifiers**: Variable references and `this` keyword
    /// 
    /// # Special Handling
    /// - **`this` keyword**: Resolves to the current class instance within method contexts
    /// - **Variable lookup**: Searches the function scope for variable definitions
    /// - **Type validation**: Ensures all arguments have valid, resolvable types
    /// 
    /// # Errors
    /// Returns errors for:
    /// - `this` used outside of class method context
    /// - Undefined variables or identifiers
    /// - Unsupported expression types as arguments
    /// - Type resolution failures for nested expressions
    pub fn get_type_information_from_expression(context: &mut TirContext<'base>, scope_location: ScopeLocation, function_scope_location: ScopeLocation, function_call: &FunctionCallAst<'base>, argument: &ExpressionAst<'base>) -> Result<TypeVariableInformation<'base>, TirError> {        
        let value = match argument {
            ExpressionAst::FunctionCall(func_call) => VariableInformation::basic(func_call.call_span.clone(), Self::resolve_function_call(context, scope_location, func_call)?),
            ExpressionAst::Primitive { span, value } => VariableInformation::basic(span.clone(), try_resolve_primitive(context, value, span)?),
            ExpressionAst::Ident(ident) => {
                match ident.text == "this" {
                    true => {
                        // Find the class
                        let class_search = find_class_location(context, scope_location)
                            .and_then(|location| context.types.get_from_location(location).map(|item| (location, item)))
                            .map(|(location, signature)| (location, &signature.value));

                        match class_search {
                            Some((location, TypeValue::Class(_))) => VariableInformation::basic(ident.clone(), location),
                            _ => return Err(FunctionResolveError::this_need_to_define_in_class(ident.into()))
                        }
                    }
                    false => {
                        let scope = context.get_scope(function_scope_location).unwrap();
                        match scope.get_variable(context, ident) {
                            Some(variable) => variable,
                            None => return Err(FunctionResolveError::variable_not_found(ident.into()))
                        }
                    }
                }
            },
            _ => {
                return Err(FunctionCallError::UnsupportedArgumentType(UnsupportedArgumentType {
                    position: function_call.call_span.to_range(),
                    code: (&function_call.call_span.state.file).into()
                }.into()).into());
            }
        };

        Ok(value)
    }

    /// Resolves and validates a complete function call expression
    /// 
    /// This is the main entry point for function call resolution in the TIR system.
    /// It performs comprehensive analysis including path resolution, argument validation,
    /// signature matching, and type checking to ensure the function call is valid.
    /// 
    /// # Resolution Process
    /// 1. **Path Resolution**: Determines if this is a direct call or method call
    /// 2. **Object Resolution**: For method calls, resolves the target object
    /// 3. **Function Lookup**: Finds the function signature in the type system
    /// 4. **Argument Processing**: Analyzes all arguments and their types
    /// 5. **Signature Validation**: Ensures argument count and types match
    /// 6. **Type Checking**: Validates argument types against parameter types
    /// 
    /// # Arguments
    /// * `context` - Mutable TIR context for type resolution and error reporting
    /// * `scope_location` - Current scope location for variable and type resolution
    /// * `function_call` - The function call AST node to resolve
    /// 
    /// # Returns
    /// * `Ok(TypeLocation)` - Location of the function's return type
    /// * `Err(TirError)` - Detailed error information for validation failures
    /// 
    /// # Call Types Supported
    /// - **Direct calls**: `functionName(args)` - calls to module or local functions
    /// - **Method calls**: `object.method(args)` - calls on object instances
    /// - **This calls**: `this.method(args)` - calls within class method context
    /// - **Module calls**: `module.function(args)` - calls to imported functions
    /// - **Chained calls**: `object.field.method(args)` - calls through object fields
    /// 
    /// # Validation Performed
    /// - **Argument count**: Ensures provided arguments match expected parameters
    /// - **Argument types**: Validates each argument type against parameter types
    /// - **Call path validity**: Ensures the call target exists and is callable
    /// - **Scope validity**: Ensures all referenced variables are in scope
    /// 
    /// # Error Conditions
    /// Returns detailed errors for:
    /// - **Argument count mismatch**: Wrong number of arguments provided
    /// - **Type mismatch**: Argument types don't match parameter types
    /// - **Invalid call path**: Calling methods on non-objects or undefined functions
    /// - **Scope errors**: Variables or functions not found in current scope
    /// - **Unsupported expressions**: Complex argument expressions not yet supported
    /// 
    /// # Integration
    /// This method integrates with:
    /// - **Type system**: For signature lookup and type validation
    /// - **Scope system**: For variable and function resolution
    /// - **Error system**: For detailed diagnostic reporting
    /// - **Module system**: For cross-module function calls
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
                if let Some(argument) = scope.get_variable(context, span) {
                    callee_object_location = argument.location;
                } else {
                    panic!("Function argument or object not found: '{path}'");
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
                        code: (&span.state.file).into()
                    }.into()).into()),
                }
            }
        }

        let function_scope_location = scope.location;
        let mut arguments = Vec::new();
        for argument in function_call.arguments.iter() {
            let type_information = Self::get_type_information_from_expression(context, scope_location, function_scope_location, function_call, argument)?;
            arguments.push(type_information);
        }

        let callee_object = context.types.get_from_location(callee_object_location).expect("Compiler bug");

        let callee = match callee_object.value.as_ref() {
            TypeValue::Function(function) => function,
            _ => panic!("Expected a function signature, but got {:?}", callee_object.value)
        };
        
        let return_type = callee.return_type;

        /* Validate parameters */
        if callee.arguments.len() != arguments.len() {
            let function_name = callee.ast.name.text.to_string();
            let expected_plural = if callee.arguments.len() == 1 { "" } else { "s" };
            let got_plural = if arguments.len() == 1 { "was" } else { "were" };
            let function_call_path = function_call.path.call();
            
            return Err(FunctionCallError::FunctionCallArgumentCountMismatch(FunctionCallArgumentCountMismatch {
                function_name,
                expected_size: callee.arguments.len(),
                got_size: arguments.len(),
                expected_plural: expected_plural.to_string(),
                got_plural: got_plural.to_string(),
                expected: TypeWithSpan {
                        ty: format!("this function expects {} argument{}", callee.arguments.len(), expected_plural),
                        at: callee.ast.arguments_span.to_range(),
                        source_code: (&callee.ast.arguments_span.state.file).into()
                    },
                got: TypeWithSpan {
                    ty: if arguments.is_empty() {
                        format!("{function_call_path}() does not have arguments")
                    } else {
                        format!("{}() has {} argument{}", function_call_path, arguments.len(), if arguments.len() == 1 { "" } else { "s" })
                    },
                    at: function_call.arguments_span.position.clone(),
                    source_code: (&function_call.arguments_span.state.file).into()
                }

            }.into()).into());
        }

        for (callee_arg, call_information_type) in callee.arguments.iter().zip(arguments.iter()) {
            let callee_argument_signature = context.types.get_from_location(callee_arg.field_type).unwrap();
            let call_argument_signature = context.types.get_from_location(call_information_type.location).unwrap();

            if !callee_argument_signature.value.is_same_type(context, &call_argument_signature.value) {
                return Err(FunctionCallError::ArgumentTypeMismatch(ArgumentTypeMismatch {
                    expected: TypeWithSpan {
                        ty: callee_arg.field_type_span.text.to_string(),
                        at: callee_arg.field_type_span.to_range(),
                        source_code: (&callee_arg.field_type_span.state.file).into()
                    },
                    got: TypeWithSpan {
                        ty: call_argument_signature.value.get_name().to_string(),
                        at: call_information_type.span.position.clone(),
                        source_code: (&call_information_type.span.state.file).into()
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
        pub func abc(a:string): string {
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

    // Test cases for improved error messages
    #[test]
    fn func_call_argument_mismatch_no_args() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        abc();
    }
}

func abc(a: string): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `abc` expects 1 argument, but 0 were provided"));
    }

    #[test]
    fn func_call_argument_mismatch_too_many() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        abc("hello", "world");
    }
}

func abc(a: string): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `abc` expects 1 argument, but 2 were provided"));
    }

    #[test]
    fn func_call_argument_mismatch_multiple_args() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        abc();
    }
}

func abc(a: string, b: i8, c: bool): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `abc` expects 3 arguments, but 0 were provided"));
    }

    #[test]
    fn func_call_module_function_success() {
        let state1 = State::new(SourceFile::new(vec!["utils".into()], r#"
func helper(text: string): string {
}
"#.to_string()));

        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
class TestClass {
    func init(this): string {
        utils.helper("test");
    }
}
"#.to_string()));
        
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();
        
        let result = process_ast(vec![ast1.into(), ast2.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_module_function_arg_mismatch() {
        let state1 = State::new(SourceFile::new(vec!["utils".into()], r#"
func helper(text: string, count: i8): string {
}
"#.to_string()));

        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
class TestClass {
    func init(this): string {
        utils.helper("test");
    }
}
"#.to_string()));
        
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();
        
        let result = process_ast(vec![ast1.into(), ast2.into()]);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `helper` expects 2 arguments, but 1 was provided"));
    }

    #[test]
    fn func_call_this_method_success() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        this.validate("test");
    }
    
    func validate(input: string): bool {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_this_method_arg_mismatch() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        this.validate();
    }
    
    func validate(input: string): bool {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `validate` expects 1 argument, but 0 were provided"));
    }

    #[test]
    fn func_call_interface_method_success() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface IValidator {
    func validate(input: string): bool;
}

extend TestClass: IValidator {
    func validate(input: string): bool {
    }
}

class TestClass {
    func init(this): string {
        this.validate("test");
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_chained_calls() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        process(getData());
    }
    
    func getData(): string {
    }
    
    func process(data: string): string {
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_zero_args_function() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        initialize();
    }
}

func initialize(): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_zero_args_function_with_args() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        initialize("unexpected");
    }
}

func initialize(): string {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Function `initialize` expects 0 arguments, but 1 was provided"));
    }

    #[test]
    fn func_call_recursive_function() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
func factorial(n: i8): i8 {
    factorial(n);
}

class TestClass {
    func init(this): string {
        factorial(5);
    }
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_cross_module_with_alias() {
        let state1 = State::new(SourceFile::new(vec!["utilities".into()], r#"
pub func format(text: string): string {
}
"#.to_string()));

        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
use utilities.format as fmt;

class TestClass {
    func init(this): string {
        fmt("hello");
    }
}
"#.to_string()));
        
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();
        
        let result = process_ast(vec![ast1.into(), ast2.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_multiple_parameters_success() {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
class TestClass {
    func init(this): string {
        calculate(10, 20, "sum");
    }
}

func calculate(a: i8, b: i8, operation: string): i8 {
}
"#.to_string()));
        let ast = process_code(&state).unwrap();
        let result = crate::tir::build(vec![ast.into()]);
        assert!(result.is_ok());
    }

    #[test]
    fn func_call_interface_extension_method() {
        let state1 = State::new(SourceFile::new(vec!["interfaces".into()], r#"
interface IProcessor {
    func process(data: string): string;
}
"#.to_string()));

        let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
use interfaces.IProcessor;

extend DataProcessor: IProcessor {
    func process(data: string): string {
    }
}

class DataProcessor {
    func init(this): string {
        this.process("test data");
    }
}
"#.to_string()));
        
        let ast1 = process_code(&state1).unwrap();
        let ast2 = process_code(&state2).unwrap();
        
        let result = process_ast(vec![ast1.into(), ast2.into()]);
        assert!(result.is_ok());
    }
}
