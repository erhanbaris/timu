//! Extension definition resolution for the Timu TIR system.
//!
//! This module handles the semantic analysis and validation of extension definitions,
//! which implement interface contracts for existing classes. Extensions allow adding
//! new functionality to classes by implementing interface requirements, enabling
//! a form of multiple inheritance and compositional design patterns.
//!
//! # Extension Resolution Process
//!
//! Extension resolution follows a multi-phase validation approach:
//!
//! ## Phase 1: Target Resolution
//! 1. **Class identification**: Resolve the target class being extended
//! 2. **Interface resolution**: Resolve all interface contracts to implement
//! 3. **Scope preparation**: Access the class scope for member addition
//! 4. **Validation setup**: Prepare tracking structures for requirement validation
//!
//! ## Phase 2: Member Implementation
//! 1. **Field implementation**: Process field requirements from interfaces
//! 2. **Method implementation**: Process method requirements from interfaces
//! 3. **Type validation**: Ensure implementation types match interface contracts
//! 4. **Visibility validation**: Validate accessibility rules for extension fields
//!
//! ## Phase 3: Contract Validation
//! 1. **Requirement checking**: Ensure all interface requirements are satisfied
//! 2. **Type compatibility**: Validate implementation types match interface signatures
//! 3. **Completeness validation**: Ensure no interface requirements are missing
//! 4. **Extra member validation**: Detect and report unsupported extra members
//!
//! # Extension Types
//!
//! ## Interface Implementation Extensions
//! - **Single interface**: `extend Class: Interface { ... }`
//! - **Multiple interfaces**: `extend Class: Interface1, Interface2 { ... }`
//! - **Hierarchical interfaces**: Support for interfaces that extend other interfaces
//!
//! ## Member Types in Extensions
//! - **Required fields**: Fields specified in interface contracts
//! - **Required methods**: Methods specified in interface contracts
//! - **Implementation methods**: Concrete implementations of interface methods
//! - **Extension fields**: Additional data members (validated as extra/invalid)
//!
//! # Validation Rules
//!
//! ## Interface Contract Validation
//! - **Complete implementation**: All interface members must be implemented
//! - **Type compatibility**: Implementation types must match interface signatures exactly
//! - **Method signatures**: Parameter counts and types must match interface declarations
//! - **Return types**: Method return types must match interface specifications
//!
//! ## Access Control Rules
//! - **Extension fields are public**: Fields in extensions are implicitly public
//! - **Explicit public forbidden**: Using `pub` keyword in extensions is an error
//! - **Interface visibility**: Interface contracts define the required visibility
//!
//! ## Error Conditions
//! - **Missing implementations**: Interface requirements not satisfied
//! - **Type mismatches**: Implementation types don't match interface contracts
//! - **Extra members**: Extension contains members not required by interfaces
//! - **Invalid accessibility**: Using forbidden access modifiers in extensions
//! - **Duplicate members**: Attempting to add members that already exist in the class
//!
//! # Type System Integration
//!
//! ## Class Modification
//! Extensions modify the target class by:
//! - **Adding fields**: New fields are added to the class field map
//! - **Adding methods**: New methods are registered as class members
//! - **Interface tracking**: Implemented interfaces are recorded in class metadata
//! - **Scope updates**: Class scope is updated with new member variables
//!
//! ## Interface Implementation Tracking
//! - **Extension set**: Classes track which interfaces they implement
//! - **Type compatibility**: Interface types become compatible with class types
//! - **Method resolution**: Interface methods resolve to implementation methods
//! - **Polymorphism support**: Classes can be used where interfaces are expected
//!
//! # Architectural Benefits
//!
//! ## Compositional Design
//! - **Multiple inheritance simulation**: Classes can implement multiple interfaces
//! - **Contract-based programming**: Interfaces define clear contracts
//! - **Separation of concerns**: Interface definitions separate from implementations
//! - **Modular design**: Extensions can be defined in separate modules
//!
//! ## Future Extensibility
//! The extension system supports future enhancements:
//! - **Default implementations**: Interface methods with default bodies
//! - **Extension inheritance**: Extensions that build on other extensions
//! - **Conditional extensions**: Extensions that apply based on type parameters
//! - **Mixin patterns**: Shared functionality across multiple classes
//!
//! # Integration Points
//!
//! Extension resolution integrates with:
//! - **Class system**: For target class modification and member addition
//! - **Interface system**: For contract validation and requirement checking
//! - **Type system**: For type compatibility and signature validation
//! - **Module system**: For cross-module extension and interface resolution
//! - **Scope system**: For member visibility and access control
//! - **Error system**: For comprehensive validation error reporting

use core::panic;
use std::{borrow::Cow, collections::HashSet};

use indexmap::IndexMap;

use crate::{
    ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::{get_object_location_or_resolve, try_resolve_signature}, scope::{ScopeLocation, TypeVariableInformation}, TirError}
};

use super::{build_type_name, ResolveAst, TypeLocation};

impl<'base> ResolveAst<'base> for ExtendDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving extend: <u><b>{}</b></u>", self.name.names.first().unwrap().text);
        
        let mut extend_fields = TimuHashMap::<'base, Cow<'_, str>, TypeVariableInformation<'base>>::default();
        let mut extend_fields_for_track = IndexMap::<Cow<'_, str>, Span<'base>>::default();

        let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
        let class_location = get_object_location_or_resolve(context, &self.name, &module_ref, scope_location)?;

        let class_name = context.types.get_from_location(class_location).unwrap().value.get_name();
        let class_name = format!("{}.{}", module_ref.as_ref(), class_name);
        let class_scope = *context.types_scope.get(class_name.as_str()).unwrap();

        self.resolve_fields(context, class_name.as_str(), class_scope, &module_ref, &mut extend_fields, &mut extend_fields_for_track, class_location)?;
        self.resolve_interfaces(context, class_scope, &module_ref, &extend_fields, &mut extend_fields_for_track)?;

        /* Validate */
        if !extend_fields_for_track.is_empty() {
            for (_, span) in extend_fields_for_track.into_iter() {
                context.add_error(TirError::extra_field_in_extend(span.to_range(), span.state.file));
            }

            return Err(TirError::TemporaryError);
        }

        let class_binding = context.types.get_mut_from_location(class_location);
        let class = match class_binding {
            Some(signature) => match signature.value.as_mut() {
                TypeValue::Class(class) => class,
                _ => return Err(TirError::invalid_type(self.name.to_range(), "only class type is valid", self.name.names.first().unwrap().state.file.clone())),
            },
            None => return Err(TirError::type_not_found(context, self.name.to_string(), self.name.to_range(), self.name.names.first().unwrap().state.file.clone())),
        };

        for (key, argument) in extend_fields.iter() {
            class.fields.validate_insert(key.clone(), argument.clone())?;
        }
        
        let class_scope = context.get_mut_scope(class_scope).expect("Scope not found, it is a bug");
        for (_, argument) in extend_fields.into_iter() {
            class_scope.add_variable(argument)?;
        }
        
        Ok(TypeLocation::UNDEFINED)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        let name = self.name.names.first().unwrap().text;
        let interfaces = self.base_interfaces
            .iter()
            .map(|item| build_type_name(item))
            .collect::<Vec<_>>()
            .join("-");

        format!("{name}-{interfaces}").into()
    }
}

impl<'base> ExtendDefinitionAst<'base> {
    #[allow(clippy::too_many_arguments)]
    fn resolve_fields(&self, context: &mut TirContext<'base>, class_name: &str, class_scope_location: ScopeLocation, module: &ModuleRef<'base>, extend_fields: &mut TimuHashMap<'base, Cow<'base, str>, TypeVariableInformation<'base>>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, Span<'base>>, _: TypeLocation) -> Result<(), TirError> {
        for field in self.fields.iter() {
            match field {
                ExtendDefinitionFieldAst::Function(function) => {
                    let full_name = format!("{}::{}", class_name, function.name()); 
                    let child_scope_location = context.create_child_scope(full_name.into(), class_scope_location, None);
                    let class_type_location = function.resolve(context, child_scope_location)?;
                    let variable = TypeVariableInformation::basic(function.name.clone(), class_type_location);

                    extend_fields.validate_insert((function.name.text).into(), variable)?;
                    extend_fields_for_track.insert((function.name.text).into(), function.name.clone());
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.state.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module, class_scope_location)?;
                    let variable = TypeVariableInformation::basic(field.name.clone(), field_type);
                    
                    extend_fields.validate_insert((field.name.text).into(), variable)?;
                    extend_fields_for_track.insert((field.name.text).into(), field.name.clone());
                }
            };
        }

        Ok(())
    }
    
    fn resolve_interfaces(&self, context: &mut TirContext<'base>, class_scope_location: ScopeLocation, module: &ModuleRef<'base>, extend_fields: &TimuHashMap<'base, Cow<'base, str>, TypeVariableInformation<'base>>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, Span<'base>>) -> Result<(), TirError> {
        let mut errors = Vec::new();
        let mut extends = HashSet::new();

        for interface_ast in self.base_interfaces.iter() {
            // Find the inferface signature
            let type_name = build_type_name(interface_ast);
            let interface_signature = try_resolve_signature(context, module, class_scope_location, type_name.as_str())?;
            let interface_signature = match interface_signature {
                Some(interface_signature) => interface_signature,
                None => {
                    errors.push(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().state.file.clone()));
                    continue;
                }
            };

            extends.insert(interface_signature);
            let interface = if let Some(signature) = context.types.get_from_location(interface_signature) {
                match signature.value.as_ref() {
                    TypeValue::Interface(interface) => interface,
                    _ => {
                        errors.push(TirError::invalid_type(interface_ast.to_range(), "only interface type is valid", interface_ast.names.last().unwrap().state.file.clone()));
                        continue;
                    },
                }
            } else {
                errors.push(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().state.file.clone()));
                continue;
            };

            for interface_field in interface.fields.iter() {
                let extend_field = match extend_fields.get(interface_field.0.text) {
                    Some(defined_field) => defined_field,

                    // Field not defined in the extend
                    None => {
                        errors.push(TirError::interface_field_not_defined(self.name.to_range(), self.name.names.last().unwrap().state.file.clone()));
                        continue;
                    }
                };

                // Check if the field type is the same
                let defined_field_type = match context.types.get_from_location(extend_field.location) {
                    Some(field_type) => field_type,
                    None => {
                        errors.push(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().state.file.clone()));
                        continue;
                    }
                };

                let interface_field_type = match context.types.get_from_location(interface_field.1.location) {
                    Some(field_type) => field_type,
                    None => {
                        errors.push(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().state.file.clone()));
                        continue;
                    }
                };

                if !defined_field_type.value.is_same_type(context, &interface_field_type.value) {
                    errors.push(TirError::types_do_not_match(interface_field.0.to_range(), interface_field.0.state.file.clone()));
                }
                else {
                    extend_fields_for_track.swap_remove(interface_field.0.text);
                }
            }
        }

        // Copy all extend informations to class
        let class_type_location = context.get_scope(class_scope_location).unwrap().current_type;
        let class_signature = context.types.get_mut_from_location(class_type_location).unwrap();

        let class = match class_signature.value.as_mut() {
           TypeValue::Class(class) => class,
            _ => panic!("Expected class type")
        };
        class.extends.extend(&mut extends.into_iter());

        if !errors.is_empty() {
            context.add_errors(errors);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::{object_signature::TypeValue, TirError}};

    #[test]
    fn empty_interface() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {}
extend TestClass: ITest {}
class TestClass {}
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn dublicate_field_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { a: TestClass; }
extend TestClass: ITest { a: TestClass; }
class TestClass { a: TestClass; }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(): TestClass { } }
class TestClass { func test(): TestClass { } }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn extended_fields() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { func test(): TestClass; a: TestClass; }
extend TestClass: ITest { func test(): TestClass { } a: TestClass; }
class TestClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        let context = crate::tir::build(vec![ast.into()]).unwrap();

        let testclass = context.types.get("source.TestClass").unwrap();
        if let TypeValue::Class(class) = testclass.value.as_ref() {
            assert_eq!(class.fields.len(), 2);
            let field1 = context.types.get_from_location(class.fields.get("test").unwrap().location).unwrap();
            let field2 = context.types.get_from_location(class.fields.get("a").unwrap().location).unwrap();

            if let TypeValue::Function(function) = field1.value.as_ref() {
                assert_eq!(function.name.text, "test");
                assert_eq!(function.arguments.len(), 0);
            } else {
                panic!("Expected ObjectSignatureValue::Function but got {:?}", field1.value);
            }

            if let TypeValue::Class(field) = field2.value.as_ref() {
                assert_eq!(field.name.text, "TestClass");
            } else {
                panic!("Expected ObjectSignatureValue::Class but got {:?}", field2.value);
            }
        } else {
            panic!("Expected ObjectSignatureValue::Class but got {:?}", testclass.value);
        }
        Ok(())
    }

    #[test]
    fn missing_definition() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { func test(): TestClass; a: TestClass; }
extend TestClass: ITest { func test(): TestClass { } }
class TestClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn interface_and_extend_informations_different_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(a: TestClass): TestClass { } }
class TestClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn interface_and_extend_informations_different_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(): TmpClass { } }
class TestClass { }
class TmpClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn pass_class_to_interface_variable() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {
    func hello(): string;
}


extend TestClass: ITest {
    func hello(): string { }
}

class TestClass {
    func call(this): string {
        echo(this);
    }
}

func echo(a: ITest): string {
}
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn pass_class_to_interface_variable_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {
    func hello(): string;
}


extend TestClass: ITest {
    func hello(): string { }
}

class TestClass {
    func call(this): string {
        echo1(this);
        echo2(this);
    }
}

func echo1(a: ITest): string {
    echo2(a);
}

func echo2(a: ITest): string {
}
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
    
    #[test]
    fn pass_wrong_class_to_interface_variable() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {
    func hello(): string;
}

interface ITest2 {
    func hello(): string;
}

extend TestClass: ITest {
    func hello(): string { }
}

class TestClass {
    func call(this): string {
        echo(this);
    }
}

func echo(a: ITest2): string {
}
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } }
extend TestClass: Interface2 { func world(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1, Interface2 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_missing_field() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1, Interface2 { func hello(): TestClass { } }

class TestClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_3() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface2 { func world(): TestClass; }
interface Interface1: Interface2 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_4() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_5() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2 { func world(): TestClass; }
interface Interface1: Interface2, Interface3 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_6() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2: Interface3 { func world(): TestClass; }
interface Interface1: Interface2 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_7() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2: Interface3 { func world(): TestClass; }
interface Interface1: Interface2, Interface3 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#.to_string()));    
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}

