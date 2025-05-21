use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    ast::{InterfaceDefinitionAst, InterfaceDefinitionFieldAst, InterfaceFunctionDefinitionAst},
    nom_tools::{Span, ToRange},
    tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::{build_object_type, build_type_name, function_definition::FunctionArgument, try_resolve_signature}, signature::SignaturePath, ObjectSignature, TirError},
};

use super::{ResolveSignature, SignatureLocation};

#[derive(Debug)]
#[allow(dead_code)]
pub struct InterfaceDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, SignatureLocation>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct InterfaceFunctionDefinition<'base> {
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: SignatureLocation,
}

impl<'base> ResolveSignature<'base> for InterfaceDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface: <u><b>{}</b></u>", self.name.fragment());
        let (signature_path, signature_location) = context.reserve_object_location(Cow::Borrowed(self.name.fragment()), module, self.name.to_range(), self.name.extra.file.clone())?;

        let mut fields = IndexMap::<Cow<'_, str>, SignatureLocation>::default();

        for field in self.fields.iter() {
            match field {
                InterfaceDefinitionFieldAst::Function(function) => {
                    let signature = self.resolve_function(context, module, function)?;
                    fields.insert((*function.name.fragment()).into(), signature)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                }
                InterfaceDefinitionFieldAst::Field(field) => {
                    if field.is_public.is_some() {
                        return Err(TirError::extra_accessibility_identifier(field.is_public.as_ref().unwrap().to_range(), field.name.extra.file.clone()));
                    }

                    let field_type = build_object_type(context, &field.field_type, module)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
            };
        }
        
        let signature = ObjectSignature::new(ObjectSignatureValue::Interface(InterfaceDefinition {
            name: self.name.clone(),
            fields,
        }), self.name.extra.file.clone(), self.name.to_range());

        context.update_object_location(signature_path.clone(), signature);
        Ok(signature_location)
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

impl<'base> InterfaceDefinitionAst<'base> {
    fn resolve_function(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, interface_function: &InterfaceFunctionDefinitionAst<'base>) -> Result<SignatureLocation, TirError<'base>> {
        simplelog::debug!("Resolving interface function: <u><b>{}</b></u>", self.name.fragment());
                
        let full_name: Cow<'base, str> = Cow::Owned(format!("{}.{}", self.name.fragment(), self.name.fragment()));
        
        let tmp_module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
        let signature_path = SignaturePath::owned(format!("{}.{}", tmp_module.path, full_name));
        let signature_location = context.object_signatures.reserve(signature_path.clone())
            .map_err(|_| TirError::already_defined(self.name.to_range(), self.name.extra.file.clone()))?;
        tmp_module.object_signatures.insert(SignaturePath::cow(full_name), signature_location);

        let mut arguments = vec![];

        for argument in interface_function.arguments.iter() {
            let type_name = build_type_name(&argument.field_type);
            let field_type = match try_resolve_signature(context, module, type_name.as_str())? {
                Some(field_type) => field_type,
                None => {
                    return Err(TirError::TypeNotFound {
                        source: argument.field_type.names.last().unwrap().extra.file.clone(),
                        position: argument.field_type.to_range(),
                    });
                }
            };

            if arguments.iter().any(|item: &FunctionArgument| item.name.fragment() == argument.name.fragment()) {
                return Err(TirError::already_defined(argument.name.to_range(), argument.name.extra.file.clone()));
            }

            arguments.push(FunctionArgument {
                name: argument.name.clone(),
                field_type,
            });
        }

        let return_type = build_object_type(context, &interface_function.return_type, module)?;

        let signature = ObjectSignature::new(
            ObjectSignatureValue::InterfaceFunction(
                InterfaceFunctionDefinition {
                    name: self.name.clone(),
                    arguments,
                    return_type,
                },
            ),
            self.name.extra.file.clone(),
            self.name.to_range(),
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

