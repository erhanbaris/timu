use std::borrow::Cow;

use crate::{
    ast::{FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{ast_signature::AstSignatureValue, context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::{build_type_name, function::{unwrap_for_this, FunctionArgument}, get_object_location_or_resolve, try_resolve_signature, BuildFullNameLocater}, scope::ScopeLocation, signature::SignaturePath, TirError, TypeSignature}
};

use super::{build_signature_path, find_ast_signature, TypeLocation, ResolveAst};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InterfaceDefinition<'base> {
    pub name: Span<'base>,
    pub fields: TimuHashMap<Cow<'base, str>, TypeLocation>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InterfaceFunctionDefinition<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeLocation,
}

impl<'base> ResolveAst<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface: <u><b>{}</b></u>", self.name.fragment());

        let (module_ref, parent) = { 
            let scope = context.get_scope(scope_location).expect("Scope not found, it is a bug");
            (scope.module_ref.clone(), scope.parent_type)
        };
        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), parent);
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), SignaturePath::owned(full_name), &module_ref, self.name.to_range(), self.name.extra.file.clone())?;

        let mut fields = TimuHashMap::<Cow<'_, str>, TypeLocation>::default();
        
        Self::resolve_interface(context, self, &mut fields, &module_ref, parent)?;

        let signature = TypeSignature::new(TypeValue::Interface(InterfaceDefinition {
            name: self.name.clone(),
            fields,
        }), self.name.extra.file.clone(), self.name.to_range(),None);

        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError<'base>> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

impl<'base> InterfaceDefinitionAst<'base> {
    fn resolve_interface(context: &mut TirContext<'base>, interface: &InterfaceDefinitionAst<'base>, fields: &mut TimuHashMap<Cow<'base, str>, TypeLocation>, module: &ModuleRef<'base>, parent: Option<TypeLocation>) -> Result<(), TirError<'base>>  {
        let interface_path = build_signature_path(context, &interface.name, module);

        // Check if the interface is already defined
        if let Some(TypeValue::Interface(interface)) = context.types.get(interface_path.get_raw_path()).map(|signature| signature.value.as_ref()){
            for (field, location) in interface.fields.iter() {
                fields.insert(field.clone(), *location);
            }
            return Ok(());
        }

        // Interface is not defined, proceed with resolution
        for field in interface.fields.iter() {
            match field {
                InterfaceDefinitionFieldAst::Function(function) => {
                    let signature = interface.resolve_function(context, module, function, parent)?;
                    fields.validate_insert((*function.name.fragment()).into(), signature, &function.name)?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module)?;
                    fields.validate_insert((*field.name.fragment()).into(), field_type, &field.name)?;
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
                Self::resolve_interface(context, &base_interface, fields, module, parent)?;
            } else {
                return Err(TirError::InvalidType {
                    source: base_interface.names.last().unwrap().extra.file.clone(),
                    position: base_interface.to_range(),
                });
            }
        }
        
        Ok(())
    }

    fn resolve_function(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, interface_function: &InterfaceFunctionDefinitionAst<'base>, parent: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface function: <u><b>{}</b></u>", self.name.fragment());
      
        let full_name: Cow<'base, str> = Cow::Owned(format!("{}::{}", self.name.fragment(), interface_function.name.fragment()));
        
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        let signature_path = SignaturePath::owned(format!("{}.{}", tmp_module.path, full_name));
        let signature_location = context.types.reserve(signature_path.clone(), Cow::Borrowed(*interface_function.name.fragment()), interface_function.name.extra.file.clone(), interface_function.name.to_range())
            .map_err(|_| TirError::already_defined(self.name.to_range(), self.name.extra.file.clone()))?;
        tmp_module.types.insert(SignaturePath::cow(full_name), signature_location);

        let mut arguments = vec![];

        for argument in interface_function.arguments.iter() {
            let (argument_name, range, file) = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.types.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
                    (Cow::Owned(parent.value.get_name().to_string()), this.to_range(), this.extra.file.clone())
                },
                FunctionArgumentAst::Argument { name, .. } => (Cow::Borrowed(*name.fragment()), name.to_range(), name.extra.file.clone())
            };
            
            let type_name: String = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.types.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
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

        let signature = TypeSignature::new(
            TypeValue::InterfaceFunction(
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
        
        Ok(context.types.update(signature_path, signature))
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

