use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    ast::{FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst},
    nom_tools::{Span, ToRange},
    tir::{ast_signature::AstSignatureValue, context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::{build_type_name, function::{unwrap_for_this, FunctionArgument}, get_object_location_or_resolve, try_resolve_signature}, signature::SignaturePath, ObjectSignature, TirError},
};

use super::{build_signature_path, find_ast_signature, ObjectLocation, ResolveSignature};

#[derive(Debug)]
#[allow(dead_code)]
pub struct InterfaceDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, ObjectLocation>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct InterfaceFunctionDefinition<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: ObjectLocation,
}

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    fn definition(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface: <u><b>{}</b></u>", self.name.fragment());

        let (signature_path, signature_location) = context.reserve_object_location(Cow::Borrowed(self.name.fragment()), module, self.name.to_range(), self.name.extra.file.clone())?;

        let mut fields = IndexMap::<Cow<'_, str>, ObjectLocation>::default();
        
        Self::resolve_interface(context, self, &mut fields, module, parent)?;

        let signature = ObjectSignature::new(ObjectSignatureValue::Interface(InterfaceDefinition {
            name: self.name.clone(),
            fields,
        }), self.name.extra.file.clone(), self.name.to_range(),None);

        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: &ModuleRef<'base>, _: ObjectLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

impl<'base> InterfaceDefinitionAst<'base> {
    fn resolve_interface(context: &mut TirContext<'base>, interface: &InterfaceDefinitionAst<'base>, fields: &mut IndexMap<Cow<'base, str>, ObjectLocation>, module: &ModuleRef<'base>, parent: Option<ObjectLocation>) -> Result<(), TirError<'base>>  {
        let interface_path = build_signature_path(context, &interface.name, module);

        // Check if the interface is already defined
        if let Some(ObjectSignatureValue::Interface(interface)) = context.object_signatures.get(interface_path.get_raw_path()).map(|signature| signature.value.as_ref()){
            for (field, location) in interface.fields.iter() {
                fields.insert(field.clone(), location.clone());
            }
            return Ok(());
        }

        // Interface is not defined, proceed with resolution
        for field in interface.fields.iter() {
            match field {
                InterfaceDefinitionFieldAst::Function(function) => {
                    let signature = interface.resolve_function(context, module, function, parent.clone())?;
                    fields.insert((*function.name.fragment()).into(), signature)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
            };
        }

        for base_interface in interface.base_interfaces.iter() {
            let base_interface_name = build_type_name(base_interface);
            let base_interface_name = build_signature_path(context, base_interface_name.as_str(), module);

            let base_interface_location = match find_ast_signature(context, module, base_interface_name) {
                Some(location) => location,
                None => {
                    return Err(TirError::TypeNotFound {
                        source: base_interface.names.last().unwrap().extra.file.clone(),
                        position: base_interface.to_range(),
                    });
                }
            };

            let base_interface_signature = context.ast_signatures.get_from_location(base_interface_location)
                .ok_or_else(|| TirError::TypeNotFound {
                    source: base_interface.names.last().unwrap().extra.file.clone(),
                    position: base_interface.to_range(),
                })?;

            if let AstSignatureValue::Interface(base_interface) = base_interface_signature.value.clone() {
                Self::resolve_interface(context, &base_interface, fields, module, parent.clone())?;
            } else {
                return Err(TirError::InvalidType {
                    source: base_interface.names.last().unwrap().extra.file.clone(),
                    position: base_interface.to_range(),
                });
            }
        }
        
        Ok(())
    }

    fn resolve_function(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, interface_function: &InterfaceFunctionDefinitionAst<'base>, parent: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface function: <u><b>{}</b></u>", self.name.fragment());
                
        let full_name: Cow<'base, str> = Cow::Owned(format!("{}::{}", self.name.fragment(), interface_function.name.fragment()));
        
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        let signature_path = SignaturePath::owned(format!("{}.{}", tmp_module.path, full_name));
        let signature_location = context.object_signatures.reserve(signature_path.clone(), Cow::Borrowed(*interface_function.name.fragment()), interface_function.name.extra.file.clone(), interface_function.name.to_range())
            .map_err(|_| TirError::already_defined(self.name.to_range(), self.name.extra.file.clone()))?;
        tmp_module.object_signatures.insert(SignaturePath::cow(full_name), signature_location);

        let mut arguments = vec![];

        for argument in interface_function.arguments.iter() {
            let (argument_name, range, file) = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.object_signatures.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
                    (Cow::Owned(parent.value.get_name().to_string()), this.to_range(), this.extra.file.clone())
                },
                FunctionArgumentAst::Argument { name, .. } => (Cow::Borrowed(*name.fragment()), name.to_range(), name.extra.file.clone())
            };
            
            let type_name: String = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.object_signatures.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
                    parent.value.get_name().to_string()
                },
                FunctionArgumentAst::Argument { field_type, .. } => build_type_name(field_type),
            };
                        let field_type = match try_resolve_signature(context, module, type_name.as_str())? {
                Some(field_type) => field_type,
                None => return Err(TirError::type_not_found(range, file))
            };

            if arguments.iter().any(|item: &FunctionArgument| *item.name.fragment() == argument_name) {
                return Err(TirError::already_defined(range, file));
            }

            arguments.push(FunctionArgument {
                name: match argument {
                    FunctionArgumentAst::This(this) => this.clone(),
                    FunctionArgumentAst::Argument { name, .. } => name.clone()
                },
                field_type,
            });
        }

        let return_type = get_object_location_or_resolve(context, &interface_function.return_type, module)?;

        let signature = ObjectSignature::new(
            ObjectSignatureValue::InterfaceFunction(
                InterfaceFunctionDefinition {
                    name: interface_function.name.clone(),
                    arguments,
                    return_type,
                },
            ),
            self.name.extra.file.clone(),
            self.name.to_range(),
            None,
        );
        
        Ok(context.object_signatures.update(signature_path, signature))
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn empty_interface() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn basic_interface() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        a: ?Myinterface;
        func test(a: Myinterface): Myinterface;
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn missing_type_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        a: nope;
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        func test(a: nope): nope;
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        pub a: ?Myinterface;
        pub a: ?Myinterface;
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        func test(a: Myinterface): Myinterface;
        func test(a: Myinterface): Myinterface;
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn cross_reference_test() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
    interface Myinterface {
        a: ?Myinterface;
        func test(a: test): test;
    }
    
    class test {
        func test(a: test): test {}
    }"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}

