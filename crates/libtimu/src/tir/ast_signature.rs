//! AST signature management for the TIR (Type Intermediate Representation).
//!
//! This module handles the creation and management of AST signatures, which are used to
//! track type definitions and their locations within the Timu compilation process. AST
//! signatures provide a unified way to reference different language constructs (modules,
//! classes, functions, interfaces, extensions) and enable type resolution and semantic
//! analysis.
//!
//! # Core Concepts
//!
//! ## AST Signatures
//! AST signatures are metadata structures that contain:
//! - **Type information** - What kind of construct (class, function, etc.)
//! - **Source location** - Where the construct is defined in source code
//! - **Module context** - Which module contains the construct
//! - **Resolution data** - Information needed for type checking
//!
//! ## Signature Types
//! The system supports signatures for:
//! - **Modules** - Namespace containers for other constructs
//! - **Classes** - Object-oriented type definitions
//! - **Functions** - Callable procedures with parameters and return types
//! - **Interfaces** - Abstract type contracts
//! - **Extensions** - Type augmentations that add methods to existing types
//!
//! # Module Building Process
//!
//! The module building process occurs in two phases:
//!
//! ## Phase 1: Module Structure Creation
//! 1. Parse file paths to determine module hierarchy
//! 2. Create phantom modules for intermediate paths
//! 3. Create concrete modules for source files
//! 4. Establish parent-child module relationships
//!
//! ## Phase 2: Signature Registration
//! 1. Extract all top-level constructs from AST
//! 2. Create signatures for each construct
//! 3. Register signatures in the global context
//! 4. Build lookup tables for type resolution
//!
//! # Integration with Type System
//!
//! AST signatures integrate with the broader type system by:
//! - Providing source locations for error reporting
//! - Enabling qualified name resolution
//! - Supporting module import/export mechanics
//! - Facilitating incremental compilation

use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;
use simplelog::debug;
use strum_macros::EnumDiscriminants;

use crate::{
    ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst}, map::TimuHashMap, nom_tools::ToRange, tir::{scope::AstVariableInformation, TypeSignature, TypeValue}
};

use super::{
    context::TirContext, module::{Module, ModuleRef}, resolver::{ResolveAst, TypeLocation}, scope::ScopeLocation, signature::{Signature, SignaturePath}, AstSignature, TirError
};

/// Enumeration of all language constructs that can have AST signatures
/// 
/// This enum provides a unified representation for different types of language
/// constructs that need to be tracked in the type system. Each variant contains
/// the appropriate AST node or module reference for the construct type.
/// 
/// # Variants
/// 
/// - **Module** - A namespace container that can hold other constructs
/// - **Class** - An object-oriented type definition with fields and methods
/// - **Function** - A callable procedure with parameters and return type
/// - **Interface** - An abstract type contract defining required methods
/// - **Extend** - A type extension that adds methods to existing types
/// 
/// # Usage
/// 
/// The enum implements `ResolveAst` to enable uniform type resolution across
/// all construct types. It also provides name extraction and signature building
/// capabilities through trait implementations.
#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(vis(pub))]
pub enum AstSignatureValue<'base> {
    /// A module reference containing other language constructs
    Module(#[allow(dead_code)] ModuleRef<'base>),
    /// A class definition with fields and methods
    Class(#[allow(dead_code)] Rc<ClassDefinitionAst<'base>>),
    /// A function definition with parameters and return type
    Function(#[allow(dead_code)] Rc<FunctionDefinitionAst<'base>>),
    /// An interface definition specifying method contracts
    Interface(#[allow(dead_code)] Rc<InterfaceDefinitionAst<'base>>),
    /// An extension definition adding methods to existing types
    Extend(#[allow(dead_code)] Rc<ExtendDefinitionAst<'base>>),
}

impl<'base> AsRef<AstSignatureValue<'base>> for AstSignatureValue<'base> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'base> AsMut<AstSignatureValue<'base>> for AstSignatureValue<'base> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'base> ResolveAst<'base> for AstSignatureValue<'base> {
    /// Resolves the AST signature value to a type location in the type system
    /// 
    /// This method delegates to the appropriate resolve implementation for each
    /// construct type, enabling uniform resolution handling across all AST signature
    /// variants. The resolution process registers the construct in the type system
    /// and returns a location identifier for future reference.
    /// 
    /// # Arguments
    /// * `context` - The TIR context containing type information
    /// * `scope_location` - The scope where this resolution is occurring
    /// 
    /// # Returns
    /// * `Ok(TypeLocation)` - The location where this construct was registered
    /// * `Err(TirError)` - If resolution fails due to conflicts or errors
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        match self {
            AstSignatureValue::Module(target_module) => target_module.resolve(context, scope_location),
            AstSignatureValue::Class(class) => class.resolve(context, scope_location),
            AstSignatureValue::Function(function) => function.resolve(context, scope_location),
            AstSignatureValue::Interface(interface) => interface.resolve(context, scope_location),
            AstSignatureValue::Extend(extend) => extend.resolve(context, scope_location),
        }
    }

    /// Finishes the resolution process for this AST signature value
    /// 
    /// This method is called after the initial resolve phase to complete any
    /// additional processing needed. For AST signature values, no additional
    /// finishing work is required, so this returns successfully immediately.
    /// 
    /// # Arguments
    /// * `context` - The TIR context (unused for AST signatures)
    /// * `scope_location` - The scope location (unused for AST signatures)
    /// 
    /// # Returns
    /// Always returns `Ok(())` as no finishing work is needed
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    /// Extracts the name of the construct represented by this signature value
    /// 
    /// This method provides a uniform way to get the name of any language construct
    /// regardless of its specific type. The name is used for lookup operations,
    /// error reporting, and building qualified identifiers.
    /// 
    /// # Returns
    /// A borrowed or owned string containing the construct's name
    fn name(&self) -> Cow<'base, str> {
        match self {
            AstSignatureValue::Module(module) => module.name(),
            AstSignatureValue::Class(class) => class.name(),
            AstSignatureValue::Function(function) => function.name(),
            AstSignatureValue::Interface(interface) => interface.name(),
            AstSignatureValue::Extend(extend) => extend.name(),
        }
    }
}

/// Builds a module and its hierarchy from a parsed AST file
/// 
/// This function processes a source file's AST to create the corresponding module
/// structure in the TIR context. It handles both simple single-level modules and
/// complex nested module hierarchies based on the file path structure.
/// 
/// # Module Hierarchy Creation
/// 
/// For file paths like `std/collections/map.tim`, this function will:
/// 1. Create a phantom `std` module if it doesn't exist
/// 2. Create a phantom `std.collections` module if it doesn't exist  
/// 3. Create the concrete `std.collections.map` module with the actual AST
/// 4. Establish parent-child relationships between modules
/// 
/// # Phantom vs Concrete Modules
/// 
/// - **Phantom modules** - Created for intermediate path segments, contain no AST
/// - **Concrete modules** - Created for actual source files, contain full AST
/// 
/// # Arguments
/// * `context` - The TIR context to add the module to
/// * `ast` - The parsed AST for the source file
/// 
/// # Returns
/// * `Ok(())` - If the module was built successfully
/// * `Err(TirError)` - If module creation fails
/// 
/// # Errors
/// 
/// This function can fail if:
/// - Module signatures cannot be created
/// - Scope creation fails
/// - Parent module relationships cannot be established
pub fn build_module<'base>(context: &mut TirContext<'base>, ast: Rc<FileAst<'base>>) -> Result<(), TirError> {
    let module_path = ast.file.path();
    let file = ast.file.clone();
    debug!("Building module: <u><b>{:?}</b></u>", module_path);

    if module_path.len() > 1 {
        let mut base_module_path = String::new();
        let total_item = module_path.len();

        for (index, name) in module_path[0..module_path.len()].iter().enumerate() {
            let full_module_path = module_path[..index + 1].join(".");
            let is_module_missing = context.get_ast_signature(full_module_path.as_str()).is_none();
            debug!("Searching module <u><b>{}</b></u>. Is missing: {}", full_module_path, is_module_missing);

            if is_module_missing {
                let next_scope_location = context.get_next_scope_location();
                let sub_module = match total_item == index + 1 {
                    true => Module::new(name.clone().into(), full_module_path.clone().into(),file.clone(), ast.clone(), next_scope_location),
                    false => Module::phantom(name.clone().into(), full_module_path.clone().into(),file.clone(), next_scope_location),
                };

                // Create new scope for module
                context.create_scope(full_module_path.clone().into(), sub_module.get_ref());
                let sub_module_ref = sub_module.get_ref();
                build_module_signature(context, sub_module)?;
                
                if !base_module_path.is_empty() {
                    debug!("Adding submodule <u><b>{}</b></u> to base module {}", full_module_path, base_module_path);

                    if let Some(base_module) = context.modules.get_mut(base_module_path.as_str()) {
                        base_module.modules.insert(name.to_string().into(), sub_module_ref);
                    } else {
                        panic!("Base module <u><b>{base_module_path}</b></u> not found in context");
                    }
                }
                base_module_path = full_module_path.clone();
            }
        }
    } else {
        let scope_location = context.get_next_scope_location();
        let module = Module {
            name: ast.file.path()[ast.file.path().len() - 1].clone().into(),
            file: ast.file.clone(),
            path: ast.file.path().join(".").into(),
            ast_imported_modules: IndexMap::new(),
            ast_signatures: TimuHashMap::new(),
            types: IndexMap::new(),
            ast: Some(ast.clone()),
            modules: Default::default(),
            scope_location
        };

        // Create new scope for module
        context.create_scope(ast.file.path().join(".").into(), module.get_ref());
        
        debug!("Adding module to context: <u><b>{}</b></u>", module.path);
        build_module_signature(context, module)?;
    }

    Ok(())
}

/// Builds and registers signatures for all constructs within a module
/// 
/// This function processes a module's AST to extract all top-level language constructs
/// (classes, functions, interfaces, extensions) and creates signatures for each one.
/// These signatures are then registered in both the module's local signature table
/// and the global TIR context for type resolution.
/// 
/// # Signature Creation Process
/// 
/// For each construct type, the function:
/// 1. Creates a signature containing the AST node and module reference
/// 2. Generates a unique qualified name (e.g., "module.path.ConstructName")
/// 3. Registers the signature in the global context
/// 4. Adds variable information to the module's local signature table
/// 
/// # Construct Processing Order
/// 
/// The function processes constructs in a specific order:
/// 1. **Interfaces** - Type contracts that may be referenced by other constructs
/// 2. **Extensions** - Type augmentations that modify existing types
/// 3. **Classes** - Object-oriented type definitions
/// 4. **Functions** - Callable procedures and methods
/// 
/// # Arguments
/// * `context` - The TIR context to register signatures in
/// * `module` - The module containing the constructs to process
/// 
/// # Returns
/// * `Ok(())` - If all signatures were created successfully
/// * `Err(TirError)` - If signature creation or registration fails
/// 
/// # Errors
/// 
/// This function can fail if:
/// - Signature creation fails for any construct
/// - Global signature registration fails due to name conflicts
/// - Module signature table validation fails
pub fn build_module_signature<'base>(context: &mut TirContext<'base>, mut module: Module<'base>) -> Result<(), TirError> {
    let module_name = module.path.to_string();
    let mut ast_signature: TimuHashMap<SignaturePath<'base>, AstVariableInformation> = TimuHashMap::new();

    if let Some(ast) = &module.ast {
        // Interface signatures
        for interface in ast.get_interfaces() {
            let signature = Signature::from((interface.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), interface.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(interface.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(interface.name.text), variable)?;
        }

        // Extend signatures
        for extend in ast.get_extends() {
            let signature = Signature::from((extend.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), extend.name()).into(), signature)?;
            let variable = AstVariableInformation::basic(extend.name.names.last().unwrap().clone(), location);

            ast_signature.validate_insert(SignaturePath::cow(extend.name()), variable)?;
        }

        // Class signatures
        for class in ast.get_classes() {
            let signature = Signature::from((class.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), class.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(class.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(class.name.text), variable)?;
        }

        // Function signatures
        for func in ast.get_functions() {
            let signature = Signature::from((func.clone(), module.get_ref()));
            let location = context.add_ast_signature(format!("{}.{}", module.path.clone(), func.name.text).into(), signature)?;
            let variable = AstVariableInformation::basic(func.name.clone(), location);

            ast_signature.validate_insert(SignaturePath::borrowed(func.name.text), variable)?;
        }
    }

    module.ast_signatures = ast_signature;

    let signature = AstSignature::new(
        AstSignatureValue::Module(module.get_ref()),
        module.file.clone(),
        std::ops::Range {
            start: 0,
            end: 0,
        },
        None
    );

    let module_ref = module.get_ref();

    context.add_ast_signature(module_name.clone().into(), signature)?;
    context.types.add_signature(SignaturePath::owned(module_name.clone()), TypeSignature::new(TypeValue::Module(module_ref), module.file.clone(), 0..0, None)).unwrap();
    context.modules.insert(module_name.into(), module);
    Ok(())
}

impl<'base> From<(Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    /// Creates a signature for a function definition within a module
    /// 
    /// This implementation converts a function AST node and its containing module
    /// into a signature that can be registered in the type system. The signature
    /// captures the function's source location for error reporting and includes
    /// the module reference for qualified name resolution.
    /// 
    /// # Arguments
    /// * `value` - A tuple of (function AST, module reference)
    /// 
    /// # Returns
    /// A signature containing the function information and source location
    fn from(value: (Rc<FunctionDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (function, module) = value;

        let position = function.name.to_range();
        let file = function.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Function(function), file, position, module)
    }
}

impl<'base> From<(Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    /// Creates a signature for a class definition within a module
    /// 
    /// This implementation converts a class AST node and its containing module
    /// into a signature that can be registered in the type system. The signature
    /// captures the class's source location for error reporting and includes
    /// the module reference for qualified name resolution.
    /// 
    /// # Arguments
    /// * `value` - A tuple of (class AST, module reference)
    /// 
    /// # Returns
    /// A signature containing the class information and source location
    fn from(value: (Rc<ClassDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (class, module) = value;

        let position = class.name.to_range();
        let file = class.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Class(class), file, position, module)
    }
}

impl<'base> From<(Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    /// Creates a signature for an interface definition within a module
    /// 
    /// This implementation converts an interface AST node and its containing module
    /// into a signature that can be registered in the type system. The signature
    /// captures the interface's source location for error reporting and includes
    /// the module reference for qualified name resolution.
    /// 
    /// # Arguments
    /// * `value` - A tuple of (interface AST, module reference)
    /// 
    /// # Returns
    /// A signature containing the interface information and source location
    fn from(value: (Rc<InterfaceDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (interface, module) = value;

        let position = interface.name.to_range();
        let file = interface.name.state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Interface(interface), file, position, module)
    }
}


impl<'base> From<(Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)> for Signature<AstSignatureValue<'base>, ModuleRef<'base>> {
    /// Creates a signature for an extension definition within a module
    /// 
    /// This implementation converts an extension AST node and its containing module
    /// into a signature that can be registered in the type system. The signature
    /// captures the extension's source location for error reporting and includes
    /// the module reference for qualified name resolution.
    /// 
    /// Extensions are special constructs that add methods to existing types, so
    /// the source location is extracted from the first name component of the
    /// qualified type name being extended.
    /// 
    /// # Arguments
    /// * `value` - A tuple of (extension AST, module reference)
    /// 
    /// # Returns
    /// A signature containing the extension information and source location
    fn from(value: (Rc<ExtendDefinitionAst<'base>>, ModuleRef<'base>)) -> Self {
        let (extend, module) = value;

        let position = extend.name.to_range();
        let file = extend.name.names.first().unwrap().state.file.clone();
        Signature::new_with_extra(AstSignatureValue::Extend(extend), file, position, module)
    }
}
