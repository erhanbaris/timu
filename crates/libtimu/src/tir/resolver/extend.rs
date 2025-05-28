use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ExtendDefinitionAst, ExtendDefinitionFieldAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::{get_object_location_or_resolve, try_resolve_signature}, TypeSignature, TirError},
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
    pub fields: IndexMap<Cow<'base, str>, TypeLocation>,
}

impl<'base> ResolveAst<'base> for ExtendDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving extend: <u><b>{}</b></u>", self.name.names.first().unwrap().fragment());
        
        let mut extend_fields = IndexMap::<Cow<'_, str>, TypeLocation>::default();
        let mut extend_fields_for_track = IndexMap::<Cow<'_, str>, TypeLocation>::default();

        let class_location = get_object_location_or_resolve(context, &self.name, module)?;

        self.resolve_fields(context, module, &mut extend_fields, &mut extend_fields_for_track, class_location.clone())?;
        self.resolve_interfaces(context, module, &extend_fields, &mut extend_fields_for_track)?;

        let class_binding = context.types.get_mut_from_location(class_location.clone());
        let class = match class_binding {
            Some(signature) => match signature.value.as_mut() {
                TypeValue::Class(class) => class,
                _ => return Err(TirError::invalid_type(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
            },
            None => return Err(TirError::type_not_found(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone())),
        };

        /* Validate */
        if !extend_fields_for_track.is_empty() {
            let first_item = extend_fields_for_track.first().unwrap();
            let signature = context.types.get_from_location(first_item.1.clone()).unwrap();
            return Err(TirError::extra_field_in_interface(signature.position.clone(), signature.file.clone()));
        }

        for (key, value) in extend_fields.into_iter() {
            if class.fields.contains_key(&key) {
                return Err(TirError::already_defined(self.name.to_range(), self.name.names.first().unwrap().extra.file.clone()));
            }

            class.fields.insert(key, value);
        }
        
        Ok(TypeLocation::UNDEFINED)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: TypeLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
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
    fn resolve_fields(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, extend_fields: &mut IndexMap<Cow<'base, str>, TypeLocation>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, TypeLocation>, class_location: TypeLocation) -> Result<(), TirError<'base>> {
        for field in self.fields.iter() {
            match field {
                ExtendDefinitionFieldAst::Function(function) => {
                    let signature = function.resolve(context, module, Some(class_location.clone()))?;
                    extend_fields.insert((*function.name.fragment()).into(), signature.clone())
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                    extend_fields_for_track.insert((*function.name.fragment()).into(), signature);
                }
                ExtendDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module)?;
                    extend_fields.insert((*field.name.fragment()).into(), field_type.clone())
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                    extend_fields_for_track.insert((*field.name.fragment()).into(), field_type);
                }
            };
        }

        Ok(())
    }
    
    fn resolve_interfaces(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, extend_fields: &IndexMap<Cow<'base, str>, TypeLocation>, extend_fields_for_track: &mut IndexMap<Cow<'base, str>, TypeLocation>) -> Result<(), TirError<'base>> {
        for interface_ast in self.base_interfaces.iter() {
            // Find the inferface signature
            let type_name = build_type_name(interface_ast);
            let interface_signature = try_resolve_signature(context, module, type_name.as_str())?;
            let interface_signature = match interface_signature {
                Some(interface_signature) => interface_signature,
                None => return Err(TirError::type_not_found(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone()))
            };

            let interface = if let Some(signature) = context.types.get_from_location(interface_signature) {
                match signature.value.as_ref() {
                    TypeValue::Interface(interface) => interface,
                    _ => return Err(TirError::invalid_type(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                }
            } else {
                return Err(TirError::type_not_found(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone()));
            };

            for interface_field in interface.fields.iter() {
                let extend_field = match extend_fields.get(interface_field.0) {
                    Some(defined_field) => defined_field,

                    // Field not defined in the extend
                    None => return Err(TirError::interface_field_not_defined(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                };

                // Check if the field type is the same
                let defined_field_type = match context.types.get_from_location(extend_field.clone()) {
                    Some(field_type) => field_type,
                    None => return Err(TirError::type_not_found(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
                };

                let interface_field_type = match context.types.get_from_location(interface_field.1.clone()) {
                    Some(field_type) => field_type,
                    None => return Err(TirError::type_not_found(interface_ast.to_range(), interface_ast.names.last().unwrap().extra.file.clone())),
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
    use crate::{process_code, tir::object_signature::TypeValue};

    #[test]
    fn empty_interface() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest {}
extend TestClass: ITest {}
class TestClass {}
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn dublicate_field_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { a: TestClass; }
extend TestClass: ITest { a: TestClass; }
class TestClass { a: TestClass; }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(): TestClass { } }
class TestClass { func test(): TestClass { } }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn extended_fields() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; a: TestClass; }
extend TestClass: ITest { func test(): TestClass { } a: TestClass; }
class TestClass { }
    "#)?;
        let context = crate::tir::build(vec![ast.into()]).unwrap();

        let testclass = context.types.get("source.TestClass").unwrap();
        if let TypeValue::Class(class) = testclass.value.as_ref() {
            assert_eq!(class.fields.len(), 2);
            let field1 = context.types.get_from_location(class.fields["test"].clone()).unwrap();
            let field2 = context.types.get_from_location(class.fields["a"].clone()).unwrap();

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
    fn missing_definition() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; a: TestClass; }
extend TestClass: ITest { func test(): TestClass { } }
class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn interface_and_extend_informations_different_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(a: TestClass): TestClass { } }
class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn interface_and_extend_informations_different_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; }
extend TestClass: ITest { func test(): TmpClass { } }
class TestClass { }
class TmpClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } }
extend TestClass: Interface2 { func world(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1, Interface2 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_missing_field() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }
interface Interface2 { func world(): TestClass; }

extend TestClass: Interface1, Interface2 { func hello(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_3() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface2 { func world(): TestClass; }
interface Interface1: Interface2 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_4() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface1 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_5() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2 { func world(): TestClass; }
interface Interface1: Interface2, Interface3 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_6() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2: Interface3 { func world(): TestClass; }
interface Interface1: Interface2 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn multiple_interface_validation_7() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
interface Interface3 { func test(): TestClass; }
interface Interface2: Interface3 { func world(): TestClass; }
interface Interface1: Interface2, Interface3 { func hello(): TestClass; }

extend TestClass: Interface1 { func hello(): TestClass { } func world(): TestClass { } func test(): TestClass { } }

class TestClass { }
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}

