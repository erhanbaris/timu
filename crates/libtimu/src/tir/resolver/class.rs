use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst, TypeNameAst}, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::TypeValue, resolver::get_object_location_or_resolve, signature::SignaturePath, TirError, TypeSignature}
};

use super::{TypeLocation, ResolveAst};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<TypeSignature<'base>>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct ClassDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, TypeLocation>,
    pub extends:Vec<TypeNameAst<'base>>,
}

impl<'base> ResolveAst<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module_ref: &ModuleRef<'base>, _: Option<TypeLocation>) -> Result<TypeLocation, TirError<'base>> {
        simplelog::debug!("Resolving class: <u><b>{}</b></u>", self.name.fragment());

        let full_name = self.build_full_name(context, module_ref, None);
        let (signature_path, class_location) = context.reserve_object_location(self.name(), SignaturePath::owned(full_name), module_ref, self.name.to_range(), self.name.extra.file.clone())?;
        let mut fields = IndexMap::<Cow<'_, str>, TypeLocation>::default();

        let mut function_signatures = Vec::new();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::Field(field) => {
                    let field_type = get_object_location_or_resolve(context, &field.field_type, module_ref)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
                ClassDefinitionFieldAst::Function(function) => {
                    let signature = function.resolve(context, module_ref, Some(class_location.clone()))?;
                    fields.insert((*function.name.fragment()).into(), signature.clone())
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                    function_signatures.push((signature, function));
                }
            };
        }
        
        let class_signature = TypeSignature::new(TypeValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
            extends: Default::default(),
        }), self.name.extra.file.clone(), self.name.to_range(), None);

        context.publish_object_location(signature_path.clone(), class_signature);

        Ok(class_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: TypeLocation) -> Result<(), TirError<'base>> {
        for field in self.fields.iter() {
            if let ClassDefinitionFieldAst::Function(function) = field {
                let full_name: Cow<'base, str> = Cow::Owned(function.build_full_name(context, module, None));
                let function_location = context.types.location(full_name.as_ref()).unwrap();
                function.finish(context, module, function_location)?;
            }
        }
        
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(*self.name.fragment())
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn missing_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { func test(a: a): a {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn recursive_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { a: test; func test(a: test): test {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { func test(this): test {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { func test(this, a: test): test {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_3() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class test { func test(a: test, this): test {} }")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn call_interface_function() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"
        interface ITest {
    func test(): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test();
        this.a.test();
    }
}
    "#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}
