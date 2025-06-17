use core::panic;
use std::borrow::Cow;

use crate::{
    ast::{FunctionArgumentAst, InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{ast_signature::AstSignatureValue, context::TirContext, module::ModuleRef, object_signature::{GetItem, TypeValue, TypeValueDiscriminants}, resolver::{build_type_name, function::{unwrap_for_this, FunctionArgument}, get_object_location_or_resolve, try_resolve_signature, BuildFullNameLocater}, scope::ScopeLocation, signature::SignaturePath, TirError, TypeSignature}
};

use super::{build_signature_path, find_ast_signature, TypeLocation, ResolveAst};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InterfaceDefinition<'base> {
    pub name: Span<'base>,
    pub full_name: String,
    pub fields: TimuHashMap<Span<'base>, TypeLocation>,
}

impl GetItem for InterfaceDefinition<'_> {
    fn get_item_location(&self, _: &TirContext<'_>, _: &str) -> Option<TypeLocation> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InterfaceFunctionDefinition<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeLocation,
}

impl GetItem for InterfaceFunctionDefinition<'_> {
    fn get_item_location(&self, _: &TirContext<'_>, _: &str) -> Option<TypeLocation> {
        None
    }
}

impl<'base> ResolveAst<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving interface: <u><b>{}</b></u>", self.name.text);

        let (module_ref, parent) = { 
            let scope = context.get_scope(scope_location).expect("Scope not found, it is a bug");
            (scope.module_ref.clone(), scope.parent_type)
        };
        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), parent);
        let (signature_path, signature_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Interface, SignaturePath::owned(full_name.clone()), &module_ref, self.name.to_range(), self.name.state.file.clone())?;

        let mut fields = TimuHashMap::<Span<'_>, TypeLocation>::default();
        let mut base_interfaces = TimuHashMap::<Cow<'_, str>, TypeLocation>::default();
        
        Self::resolve_interface(context, self, self, &mut fields, &mut base_interfaces, &module_ref, scope_location, parent)?;

        let signature = TypeSignature::new(TypeValue::Interface(InterfaceDefinition {
            name: self.name.clone(),
            full_name,
            fields,
        }), self.name.state.file.clone(), self.name.to_range(),None);

        context.publish_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn finish(&self, _: &mut TirContext<'base>, _: ScopeLocation) -> Result<(), TirError> { Ok(()) }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(self.name.text)
    }
}

impl<'base> InterfaceDefinitionAst<'base> {
    #[allow(clippy::only_used_in_recursion)]
    #[allow(clippy::too_many_arguments)]
    fn resolve_interface(context: &mut TirContext<'base>, resolve_interface: &InterfaceDefinitionAst<'base>, interface: &InterfaceDefinitionAst<'base>, fields: &mut TimuHashMap<Span<'base>, TypeLocation>, base_interfaces: &mut TimuHashMap<Cow<'base, str>, TypeLocation>, module: &ModuleRef<'base>, scope_location: ScopeLocation, parent: Option<TypeLocation>) -> Result<(), TirError>  {
        let interface_path = build_signature_path(context, interface.name.text, module);

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
                    let signature = interface.resolve_function(context, module, scope_location, function, parent)?;
                    fields.validate_insert(function.name.clone(), signature, &function.name)?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.state.file.clone()));
                    }

                    let field_type = get_object_location_or_resolve(context, &field.field_type, module, scope_location)?;
                    fields.validate_insert(field.name.clone(), field_type, &field.name)?;
                }
            };
        }

        for base_interface in interface.base_interfaces.iter() {
            let base_interface_name = build_type_name(base_interface);
            let base_interface_name = build_signature_path(context, base_interface_name.as_str(), module);

            let base_interface_location = match find_ast_signature(context, module, base_interface_name) {
                Some(location) => location,
                None => {
                    return Err(TirError::type_not_found(context, base_interface.to_string(), base_interface.to_range(), base_interface.names.last().unwrap().state.file.clone()));
                }
            };

            let base_interface_signature = context.ast_signatures.get_from_location(base_interface_location)
                .ok_or_else(|| TirError::type_not_found(context, base_interface.to_string(), base_interface.to_range(), base_interface.names.last().unwrap().state.file.clone()))?;

            match base_interface_signature.value.clone() {
                AstSignatureValue::Interface(base_interface) => {

                    if base_interface.index == resolve_interface.index {
                        return Err(TirError::circular_reference(resolve_interface.name.to_range(), resolve_interface.name.state.file.clone()));
                    }

                    Self::resolve_interface(context, resolve_interface, &base_interface, fields, base_interfaces, module, scope_location, parent)?
                },
                _ => return Err(TirError::invalid_type(base_interface.to_range(), "only interface type is valid", base_interface.names.last().unwrap().state.file.clone()))
            };
        }
        
        Ok(())
    }

    fn resolve_function(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, scope_location: ScopeLocation, interface_function: &InterfaceFunctionDefinitionAst<'base>, parent: Option<TypeLocation>) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving interface function: <u><b>{}</b></u>", self.name.text);
      
        let full_name: Cow<'base, str> = Cow::Owned(format!("{}::{}", self.name.text, interface_function.name.text));
        
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        let signature_path = SignaturePath::owned(format!("{}.{}", tmp_module.path, full_name));
        let signature_location = context.types.reserve(signature_path.clone(), Cow::Borrowed(interface_function.name.text), TypeValueDiscriminants::InterfaceFunction, interface_function.name.state.file.clone(), interface_function.name.to_range())?;
        tmp_module.types.insert(SignaturePath::cow(full_name), signature_location);

        let mut arguments = vec![];

        for argument in interface_function.arguments.iter() {
            let (argument_name, range, file) = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.types.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
                    (Cow::Owned(parent.value.get_name().to_string()), this.to_range(), this.state.file.clone())
                },
                FunctionArgumentAst::Argument { name, .. } => (Cow::Borrowed(name.text), name.to_range(), name.state.file.clone())
            };
            
            let (field_type_span, type_name) = match argument {
                FunctionArgumentAst::This(this) => {
                    let parent = context.types.get_from_location(unwrap_for_this(&parent, this)?).unwrap();
                    (this.clone(), parent.value.get_name().to_string())
                },
                FunctionArgumentAst::Argument { field_type, .. } => (field_type.names_span.clone(), build_type_name(field_type)),
            };

            let field_type = match try_resolve_signature(context, module, scope_location, type_name.as_str())? {
                Some(field_type) => field_type,
                None => return Err(TirError::type_not_found(context, type_name, range, file))
            };

            if let Some(old) = arguments.iter().find(|item: &&FunctionArgument<'_>| *item.name.text == argument_name) {
                return Err(TirError::already_defined(old.name.to_range(), range, file));
            }

            arguments.push(FunctionArgument {
                name: match argument {
                    FunctionArgumentAst::This(this) => this.clone(),
                    FunctionArgumentAst::Argument { name, .. } => name.clone()
                },
                field_type,
                field_type_span
            });
        }

        let return_type = get_object_location_or_resolve(context, &interface_function.return_type, module, scope_location)?;

        let signature = TypeSignature::new(
            TypeValue::InterfaceFunction(
                InterfaceFunctionDefinition {
                    name: interface_function.name.clone(),
                    arguments,
                    return_type,
                },
            ),
            self.name.state.file.clone(),
            self.name.to_range(),
            None,
        );
        
        Ok(context.types.update(signature_path, signature))
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::TirError};

    #[test]
    fn empty_interface() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
    }"#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn basic_interface() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        a: ?Myinterface;
        func test(a: Myinterface): Myinterface;
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn missing_type_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        a: nope;
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn missing_type_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        func test(a: nope): nope;
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        pub a: ?Myinterface;
        pub a: ?Myinterface;
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn dublicate_field_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        func test(a: Myinterface): Myinterface;
        func test(a: Myinterface): Myinterface;
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn cross_reference_test() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
    interface Myinterface {
        a: ?Myinterface;
        func test(a: test): test;
    }
    
    class test {
        func test(a: test): test {}
    }"#.to_string()));
        let ast = process_code(&state)?;

        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn circular_reference() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} interface a: a { b: string; }  ".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source.testclass; func abc2(a: testclass): source.testclass { } func abc(a: testclass): source.testclass { }".to_string()));

        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;

        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap_err();
        Ok(())
    }
}

