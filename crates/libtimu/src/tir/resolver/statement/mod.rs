//! Statement resolution and validation for the Timu TIR system.
//!
//! This module provides the core infrastructure for resolving and validating
//! statements within function bodies during semantic analysis. It implements
//! the `ResolveAst` trait for body statements and provides utilities for
//! primitive type resolution and function signature management.
//!
//! # Statement Types
//!
//! The module handles resolution of various statement types that can appear
//! within function bodies:
//!
//! ## Function Calls
//! - **Direct function calls**: `functionName(args)`
//! - **Method calls**: `object.method(args)`
//! - **Module function calls**: `module.function(args)`
//!
//! ## Future Statement Types
//! The architecture supports expansion for additional statement types:
//! - Variable assignments
//! - Control flow statements (if, while, for)
//! - Return statements
//! - Expression statements
//!
//! # Resolution Process
//!
//! Statement resolution follows a two-phase approach:
//!
//! ## Phase 1: Resolve
//! - **Type determination**: Determine the result type of the statement
//! - **Scope validation**: Ensure all referenced identifiers are in scope
//! - **Signature matching**: For function calls, validate against signatures
//! - **Type checking**: Ensure type compatibility and correctness
//!
//! ## Phase 2: Finish
//! - **Finalization**: Complete any deferred resolution tasks
//! - **Cross-references**: Resolve any remaining type dependencies
//! - **Validation**: Final validation of resolved statements
//!
//! # Function Signature Support
//!
//! The module provides data structures for managing function signatures:
//! - **`FunctionArgument`**: Represents function parameters with names and types
//! - **`ClassFunctionSignature`**: Complete signature information for class methods
//!
//! # Primitive Type Resolution
//!
//! The `try_resolve_primitive` function handles resolution of primitive values
//! (strings, numbers, booleans) to their corresponding type representations
//! in the type system.
//!
//! # Integration
//!
//! This module integrates with:
//! - **Type system**: For type lookup and validation
//! - **Scope system**: For identifier resolution
//! - **Function call resolver**: For detailed function call analysis
//! - **Error system**: For comprehensive error reporting

use core::panic;
use std::borrow::Cow;

use crate::{
    ast::{BodyStatementAst, PrimitiveValue}, nom_tools::{Span, ToRange}, tir::{context::TirContext, object_signature::TypeValue, scope::ScopeLocation, signature::SignaturePath, TirError}
};

use super::{ResolveAst, TypeLocation};
mod function_call;

pub use function_call::FunctionCallError;

/// Represents a function parameter with its name and type information
/// 
/// This structure stores the essential information about a function parameter
/// including its identifier name and resolved type location within the type system.
#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: TypeLocation,
}

/// Complete signature information for class method functions
/// 
/// This structure contains all the metadata necessary to represent a class method
/// including visibility, parameters, return type, and signature path for resolution.
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
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        match self {
            BodyStatementAst::FunctionCall(function_call) => Self::resolve_function_call(context, scope_location, function_call),
            _ => panic!("Unsupported BodyStatementAst variant: {:?}", self),
        }
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> {
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed("")
    }
}

/// Resolves a primitive value to its corresponding type location in the type system
/// 
/// This function takes a primitive value (string, number, boolean) and finds its
/// corresponding type representation in the TIR type system. It's used during
/// expression resolution to determine the types of literal values.
/// 
/// # Arguments
/// * `context` - Mutable TIR context for type system access
/// * `primitive` - The primitive value to resolve
/// * `span` - Source location information for error reporting
/// 
/// # Returns
/// * `Ok(TypeLocation)` - Location of the primitive's type in the type system
/// * `Err(TirError)` - Error if the primitive type is not found
/// 
/// # Errors
/// Returns `TirError::type_not_found` if the primitive type is not registered
/// in the type system, which typically indicates a compiler setup issue.
pub fn try_resolve_primitive<'base>(context: &mut TirContext<'base>, primitive: &PrimitiveValue<'base>, span: &Span<'base>) -> Result<TypeLocation, TirError> {
    let location = context.types.find_by_value(&TypeValue::PrimitiveType(primitive.to_type()));
    match location {
        Some(location) => Ok(location),
        None => Err(TirError::type_not_found(context, span.to_string(), span.to_range(), span.state.file.clone())),
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};

    #[test]
    fn missing_type_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(): a {} ".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn duplicated_function_argument() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class a {} func test(a: a, a: a): a {} ".to_string()));
        let ast = process_code(&state)?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::AlreadyDefined(error) = error {
            assert_eq!(error.new_position, 27..28);
        } else {
            panic!("Expected TirError::AlreadyDefined but got {error:?}");
        }

        Ok(())
    }

    #[test]
    fn valid_types() -> Result<(), TirError> {
        
        let state_1 = State::new(SourceFile::new(vec!["lib".into()], " class testclass1 {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["main".into()],
            r#"use lib.testclass1 as test;
    func main(a: test): test {}"#.to_string()));
        
        let source_1 = process_code(&state_1)?;
        let source_2 = process_code(&state_2)?;

        let context = process_ast(vec![source_2.into(), source_1.into()])?;
        assert_eq!(context.modules.len(), 2);

        let main_module = context.modules.iter().find(|(name, _)| *name == "main").unwrap();
        let lib_module = context.modules.iter().find(|(name, _)| *name == "lib").unwrap();

        main_module.1.types.get("main").unwrap();

        assert!(main_module.1.ast_imported_modules.get("testclass1").is_none());
        assert!(main_module.1.ast_imported_modules.get("test").is_some());
        assert!(main_module.1.types.get("testclass1").is_none());

        lib_module.1.types.get("testclass1").unwrap();

        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(a: a): test {}".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn not_in_class() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "func test(this): test {}".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
