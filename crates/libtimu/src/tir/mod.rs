//! Type Intermediate Representation (TIR) for the Timu language compiler.
//!
//! The TIR module implements the semantic analysis phase of the Timu compiler,
//! transforming the Abstract Syntax Tree (AST) into a type-checked intermediate
//! representation with full semantic information.
//!
//! # Architecture Overview
//!
//! The TIR system operates in two main phases:
//!
//! ## 1. Resolve Phase
//! - **Signature Building**: Collect all type signatures, function signatures, and module declarations
//! - **Symbol Tables**: Build lookup tables for efficient name resolution
//! - **Forward Declarations**: Handle forward references and circular dependencies
//! - **Module System**: Process `use` statements and build module hierarchy
//!
//! ## 2. Finish Phase  
//! - **Type Checking**: Verify all type relationships and constraints
//! - **Implementation Validation**: Ensure interface implementations are complete
//! - **Cross-References**: Validate references between modules and types
//! - **Error Collection**: Gather and report semantic errors
//!
//! # Key Components
//!
//! - [`TirContext`] - Global compilation state and symbol tables
//! - [`signature`] - Type and function signature management
//! - [`resolver`] - Two-phase semantic analysis implementation
//! - [`object_signature`] - Type system and primitive type definitions
//! - [`scope`] - Hierarchical scope management for variables and types
//! - [`module`] - Module system and import/export handling
//! - [`error`] - Rich error reporting with source locations
//!
//! # Type System
//!
//! The type system supports:
//! - **Primitive types**: Integers, floats, booleans, strings
//! - **Nullable types**: Optional values with `?` syntax
//! - **Reference types**: Memory references with `ref` keyword  
//! - **Classes and interfaces**: Object-oriented programming constructs
//! - **Generic types**: Parameterized types (planned)
//!
//! # Usage
//!
//! The main entry point is the [`build`] function which takes parsed AST files
//! and produces a fully type-checked [`TirContext`]:
//!
//! ```ignore
//! let files: Vec<Rc<FileAst>> = // ... parsed files
//! let tir_context = tir::build(files)?;
//! ```

use std::rc::Rc;

use ast_signature::{build_module, AstSignatureValue};
pub use context::TirContext;
pub use error::TirError;
use module::{Module, ModuleRef};
pub use object_signature::{PrimitiveType, TypeValue};
use resolver::{build_file, AstSignatureLocation, ObjectLocation, ResolveAst, TypeLocation};
use scope::ScopeLocation;
use signature::{Signature, SignatureHolder, SignaturePath};

use crate::{ast::{FileAst, FileStatementAst}, file::SourceFile, tir::{ast_signature::AstSignatureValueDiscriminants, object_signature::TypeValueDiscriminants}};

pub mod accessibility;
mod ast_signature;
mod context;
//pub mod error;
pub mod error;
mod module;
mod object_signature;
mod resolver;
mod signature;
mod scope;


/// Type alias for AST-based signatures used during the resolve phase
/// 
/// These signatures represent function and class declarations as they appear
/// in the AST, before full type resolution has occurred.
pub type AstSignature<'base> = Signature<AstSignatureValue<'base>, ModuleRef<'base>>;

/// Type alias for AST signature holders that manage signature collections
/// 
/// Provides efficient lookup and storage for AST signatures during compilation.
pub type AstSignatureHolder<'base> = SignatureHolder<'base, AstSignatureValue<'base>, AstSignatureValueDiscriminants, AstSignatureLocation, ModuleRef<'base>>;

/// Type alias for fully resolved type signatures
/// 
/// These signatures represent complete type information after semantic analysis,
/// including resolved references and validated type relationships.
pub type TypeSignature<'base> = Signature<TypeValue<'base>, TypeLocation>;

/// Type alias for type signature holders that manage type collections
/// 
/// Provides efficient lookup and storage for resolved type signatures.
pub type TypeSignatureHolder<'base> = SignatureHolder<'base, TypeValue<'base>, TypeValueDiscriminants, TypeLocation, TypeLocation>;

/// Object location constant for the boolean `false` literal
/// 
/// This provides a stable reference to the false value in the type system.
pub static BOOL_FALSE_LOCATION: ObjectLocation = ObjectLocation(0);

/// Object location constant for the boolean `true` literal
/// 
/// This provides a stable reference to the true value in the type system.
pub static BOOL_TRUE_LOCATION: ObjectLocation = ObjectLocation(1);

/// Initializes the built-in primitive types in the TIR context
/// 
/// This function registers all primitive types (i8, u8, i16, u16, i32, u32, i64, u64,
/// float, bool, string, void) in the type signature table so they can be referenced
/// during type resolution. These types are considered part of the language's
/// standard library and are always available.
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

/// Builds the Type Intermediate Representation from parsed AST files
/// 
/// This is the main entry point for semantic analysis. It processes all source files
/// through the two-phase TIR compilation process:
/// 
/// 1. **Resolve Phase**: Builds signatures and symbol tables for all declarations
/// 2. **Finish Phase**: Performs type checking and validates all references
/// 
/// # Arguments
/// * `files` - Vector of parsed AST files to process
/// 
/// # Returns
/// * `Ok(TirContext)` - Complete type-checked compilation context
/// * `Err(TirError)` - Semantic analysis errors with source locations
/// 
/// # Process
/// 1. Initialize primitive types in the context
/// 2. Build module structure from file paths
/// 3. Resolve all signatures (classes, functions, interfaces)
/// 4. Finish phase validation and type checking
/// 5. Return completed context or collected errors
/// 
/// # Examples
/// ```ignore
/// let files = vec![ast_file1, ast_file2];
/// let tir_context = tir::build(files)?;
/// // tir_context now contains complete semantic information
/// ```
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
        ast::FileAst, file::SourceFile, nom_tools::State, process_code, tir::{ast_signature::{build_module_signature, AstSignatureValue}, scope::ScopeLocation}
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
            scope_location: ScopeLocation::UNDEFINED
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
            scope_location: ScopeLocation::UNDEFINED
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
            scope_location: ScopeLocation::UNDEFINED
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
        let state_1 = State::new(SourceFile::new(vec!["source1".into()], " pub class testclass1 {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["source2".into()], "use source1; use source1.testclass1;".to_string()));
        let state_3 = State::new(SourceFile::new(vec!["sub".into(), "source3".into()], "pub class testclass2 {}".to_string()));
        let state_4 = State::new(SourceFile::new(vec!["sub".into(), "source4".into()], "use source1; use source1.testclass1;".to_string()));
        let state_5 = State::new(SourceFile::new(vec!["sub".into(), "source5".into()], "use source1; use source1.testclass1;".to_string()));
        let state_6 = State::new(SourceFile::new(vec!["sub".into(), "source6".into()], "use sub.source3; use sub.source3.testclass2;".to_string()));
        let state_7 = State::new(SourceFile::new(vec!["sub".into(), "source7".into()], "use source1; use source1.testclass1; use sub.source3; use sub.source3.testclass2;".to_string()));
        let state_8 = State::new(SourceFile::new(vec!["sub".into(), "source8".into()], "pub class testclass1 {}".to_string()));
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
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::ImportNotFound(error) = error
        {
            assert_eq!(error.module, "missing");
        } else {
            panic!("Expected TirError::ImportNotFound {error}");
        }

        Ok(())
    }

    #[test]
    fn duplicated_module() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " pub class testclass {} ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source.testclass; use source.testclass;".to_string()));
        
        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        let error = crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap_err();

        if let TirError::ModuleAlreadyImported(error) = error {
            assert_eq!(error.old_position, 11..20);
            assert_eq!(error.new_position, 26..42);
        } else {
            panic!("Expected TirError::ModuleAlreadyImported");
        }
        Ok(())
    }

    #[test]
    fn no_duplicated_module() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " pub class testclass {} ".to_string()));
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
