use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::{get_object_location_or_resolve, try_resolve_signature}, scope::ScopeLocation, TirError, TypeSignature}
};

use super::{build_type_name, ResolveAst, TypeLocation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtendArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<TypeSignature<'base>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExtendDefinition<'base> {
    pub name: Span<'base>,
    pub fields: TimuHashMap<Cow<'base, str>, TypeLocation>,
}

impl<'base> ResolveAst<'base> for ExtendDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving extend: <u><b>{}</b></u>", self.name.names.first().unwrap().fragment());
        
        let mut extend_fields = TimuHashMap::<Cow<'_, str>, (Span<'base>, TypeLocation)>::default();
        let mut extend_fields_for_track = IndexMap::<Cow<'_, str>, TypeLocation>::default();

        let module_ref = context.get_scope(scope_location).unwrap().module_ref.clone();
        let class_location = get_object_location_or_resolve(context, &self.name, &module_ref)?;

        let class_name = context.types.get_from_location(class_location).unwrap().value.get_name();
        let class_name = format!("{}.{}", module_ref.as_ref(), class_name);
        let class_scope = *context.types_scope.get(class_name.as_str()).unwrap();

        self.resolve_fields(context, class_name.as_str(), class_scope, &module_ref, &mut extend_fields, &mut extend_fields_for_track, class_location)?;
        self.resolve_interfaces(context, &module_ref, &extend_fields, &mut extend_fields_for_track)?;

        /* Validate */
        if !extend_fields_for_track.is_empty() {
            let first_item = extend_fields_for_track.first().unwrap();
            let signature = context.types.get_from_location(*first_item.1).unwrap();
            return Err(TirError::extra_field_in_interface(signature.position.clone(), signature.file.clone()));
        }

        let class_binding = context.types.get_mut_from_location(class_location);
        let class = match class_binding {
            Some(signature) => match signature.value.as_mut() {
                TypeValue::Class(class) => class,
                _ => return Err(TirError::invalid_type(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
            },
            None => return Err(TirError::type_not_found(context, self.name.to_string(), self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
        };

        for (key, (_, value)) in extend_fields.iter() {
            class.fields.validate_insert(key.clone(), *value, self.name.names.first().unwrap())?;
        }


        let class_scope = context.get_mut_scope(class_scope).expect("Scope not found, it is a bug");
        for (_, (span, value)) in extend_fields.into_iter() {
            class_scope.add_variable(span.clone(), value)?;
        }
        
        Ok(TypeLocation::UNDEFINED)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        let name = self.name.names.first().unwrap().fragment();
        let interfaces = self.base_interfaces
            .iter()
            .map(|item| build_type_name(item))
            .collect::<Vec<_>>()
            .join("-");

        format!("{}-{}", name, interfaces).into()
    }
}

impl<'base> ExtendDefinitionAst<'base> {
    #[allow(clippy::too_many_arguments)]
    fn resolve_fields(&self, context: &mut TirContext<'base>, class_name: &str, class_scope_location: ScopeLocation, module: &ModuleRef<'base>, extend_fields: &mut TimuHashMap<Cow<'base, str>, (Span<'base>, TypeLocation)>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, TypeLocation>, _: TypeLocation) -> Result<(), TirError> {
        for field in self.fields.iter() {
            match field {
                ExtendDefinitionFieldAst::Function(function) => {
                    let full_name = format!("{}::{}", class_name, function.name()); 
                    let child_scope_location = context.create_child_scope(full_name.into(), class_scope_location, None);
                    let class_type_location = function.resolve(context, child_scope_location)?;
                    extend_fields.validate_insert((*function.name.fragment()).into(), (function.name.clone(), class_type_location), &function.name)?;
                    extend_fields_for_track.insert((*function.name.fragment()).into(), class_type_location);
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module)?;
                    extend_fields.validate_insert((*field.name.fragment()).into(), (field.name.clone(), field_type), &field.name)?;
                    extend_fields_for_track.insert((*field.name.fragment()).into(), field_type);
                }
            };
        }

        Ok(())
    }
    
    fn resolve_interfaces(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, extend_fields: &TimuHashMap<Cow<'base, str>, (Span<'base>, TypeLocation)>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, TypeLocation>) -> Result<(), TirError> {
        for interface_ast in self.base_interfaces.iter() {
            // Find the inferface signature
            let type_name = build_type_name(interface_ast);
            let interface_signature = try_resolve_signature(context, module, type_name.as_str())?;
            let interface_signature = match interface_signature {
                Some(interface_signature) => interface_signature,
                None => return Err(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone()))
            };

            let interface = if let Some(signature) = context.types.get_from_location(interface_signature) {
                match signature.value.as_ref() {
                    TypeValue::Interface(interface) => interface,
                    _ => return Err(TirError::invalid_type(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                }
            } else {
                return Err(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone()));
            };

            for interface_field in interface.fields.iter() {
                let (_, extend_field) = match extend_fields.get(interface_field.0) {
                    Some(defined_field) => defined_field,

                    // Field not defined in the extend
                    None => return Err(TirError::interface_field_not_defined(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                };

                // Check if the field type is the same
                let defined_field_type = match context.types.get_from_location(*extend_field) {
                    Some(field_type) => field_type,
                    None => return Err(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                };

                let interface_field_type = match context.types.get_from_location(*interface_field.1) {
                    Some(field_type) => field_type,
                    None => return Err(TirError::type_not_found(context, interface_ast.to_string(), interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                };

                if !defined_field_type.value.compare_skeleton(context, &interface_field_type.value) {
                    return Err(TirError::types_do_not_match(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone()));
                }

                extend_fields_for_track.swap_remove(interface_field.0);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::object_signature::TypeValue};

    #[test]
    fn empty_interface() -> miette::Result<()> {
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
    fn dublicate_field_1() -> miette::Result<()> {
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
    fn dublicate_field_2() -> miette::Result<()> {
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
    fn extended_fields() -> miette::Result<()> {
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
            let field1 = context.types.get_from_location(*class.fields.get("test").unwrap()).unwrap();
            let field2 = context.types.get_from_location(*class.fields.get("a").unwrap()).unwrap();

            if let TypeValue::Function(function) = field1.value.as_ref() {
                assert_eq!(*function.name.fragment(), "test");
                assert_eq!(function.arguments.len(), 0);
            } else {
                panic!("Expected ObjectSignatureValue::Function but got {:?}", field1.value);
            }

            if let TypeValue::Class(field) = field2.value.as_ref() {
                assert_eq!(*field.name.fragment(), "TestClass");
            } else {
                panic!("Expected ObjectSignatureValue::Class but got {:?}", field2.value);
            }
        } else {
            panic!("Expected ObjectSignatureValue::Class but got {:?}", testclass.value);
        }
        Ok(())
    }

    #[test]
    fn missing_definition() -> miette::Result<()> {
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
    fn interface_and_extend_informations_different_1() -> miette::Result<()> {
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
    fn interface_and_extend_informations_different_2() -> miette::Result<()> {
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
    fn multiple_interface_validation_1() -> miette::Result<()> {
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
    fn multiple_interface_validation_2() -> miette::Result<()> {
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
    fn multiple_interface_missing_field() -> miette::Result<()> {
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
    fn multiple_interface_validation_3() -> miette::Result<()> {
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
    fn multiple_interface_validation_4() -> miette::Result<()> {
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
    fn multiple_interface_validation_5() -> miette::Result<()> {
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
    fn multiple_interface_validation_6() -> miette::Result<()> {
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
    fn multiple_interface_validation_7() -> miette::Result<()> {
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

