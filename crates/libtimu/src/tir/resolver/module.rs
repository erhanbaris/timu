//! Module reference resolution for the Timu TIR system.
//!
//! This module handles the registration and resolution of module references within
//! the Timu Type Intermediate Representation (TIR). Modules serve as the top-level
//! organizational units for code in Timu, providing namespacing and import/export
//! functionality for types, functions, and other declarations.
//!
//! # Module Resolution Process
//!
//! Module resolution is typically one of the first steps in TIR processing:
//!
//! ## Phase 1: Module Registration
//! 1. **Name resolution**: Build fully qualified module name
//! 2. **Type system registration**: Register module as a type entity
//! 3. **Signature creation**: Create module signature for type system
//! 4. **Location assignment**: Assign unique type location for module
//!
//! ## Phase 2: Module Integration
//! 1. **Scope association**: Associate module with its scope context
//! 2. **Reference tracking**: Track module references for import resolution
//! 3. **Type namespace**: Establish module as a namespace for contained types
//! 4. **Export preparation**: Prepare module for cross-module references
//!
//! # Module Functionality
//!
//! ## Namespace Provision
//! - **Type containment**: Modules contain classes, interfaces, functions
//! - **Name qualification**: Module names prefix contained type names
//! - **Scope isolation**: Each module has its own scope for type resolution
//! - **Import/export**: Modules can import types from other modules
//!
//! ## Cross-Module References
//! - **Qualified names**: Types can be referenced as `module.TypeName`
//! - **Import statements**: `use module.Type` brings types into local scope
//! - **Module aliases**: `use module.Type as Alias` provides aliasing
//! - **Path resolution**: Module paths are resolved during compilation
//!
//! # Type System Integration
//!
//! ## Module as Type Entity
//! Modules are registered in the type system as special type entities:
//! - **Type location**: Modules have unique identifiers in the type system
//! - **Signature path**: Hierarchical path for module identification
//! - **Type value**: Special `TypeValue::Module` variant for modules
//! - **Reference tracking**: Module references are tracked for resolution
//!
//! ## Namespace Management
//! - **Contained types**: Modules serve as containers for type definitions
//! - **Qualified access**: Types within modules are accessed via qualified names
//! - **Resolution context**: Modules provide context for type resolution
//! - **Import resolution**: Module system handles import statement processing
//!
//! # Resolution Implementation
//!
//! ## Simple Resolution
//! Module resolution is relatively straightforward compared to other AST nodes:
//! - **Direct registration**: Modules are directly registered in the type system
//! - **No complex validation**: Modules don't require extensive validation
//! - **Immediate availability**: Once registered, modules are available for reference
//! - **No finish phase work**: Modules require no additional processing in finish phase
//!
//! ## Integration Points
//! Module resolution integrates with:
//! - **Scope system**: For establishing module-level scopes
//! - **Type system**: For registering modules as type entities
//! - **Import system**: For processing cross-module references
//! - **Signature system**: For creating module signatures and paths
//!
//! # Architectural Role
//!
//! ## Code Organization
//! - **File mapping**: Each source file typically represents one module
//! - **Hierarchical structure**: Modules can be organized hierarchically
//! - **Dependency management**: Modules define dependency relationships
//! - **Compilation units**: Modules serve as compilation boundaries
//!
//! ## Future Extensibility
//! The module system supports future enhancements:
//! - **Hierarchical modules**: Nested module structures
//! - **Module interfaces**: Public/private exports from modules
//! - **Dynamic imports**: Runtime module loading capabilities
//! - **Package system**: Higher-level organization above modules

use std::borrow::Cow;

use crate::tir::{context::TirContext, module::ModuleRef, object_signature::{TypeValue, TypeValueDiscriminants}, scope::ScopeLocation, signature::SignaturePath, TirError, TypeSignature};

use super::{BuildFullNameLocater, ResolveAst, TypeLocation};

impl<'base> ResolveAst<'base> for ModuleRef<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), None);
        let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Module, SignaturePath::owned(full_name), &module_ref, 0..0, self.file())?;
        let signature = TypeSignature::new(TypeValue::Module(module_ref), self.file(), 0..0, None);
        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        self.0.clone()
    }
}
