use std::{borrow::Cow, rc::Rc};

use indexmap::IndexMap;

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst, FunctionDefinitionLocationAst, TypeNameAst}, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::ModuleRef, object_signature::ObjectSignatureValue, resolver::get_object_location_or_resolve, ObjectSignature, TirError}
};

use super::{ObjectLocation, ResolveSignature};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<ObjectSignature<'base>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassDefinition<'base> {
    pub name: Span<'base>,
    pub fields: IndexMap<Cow<'base, str>, ObjectLocation>,
    pub extends:Vec<TypeNameAst<'base>>,
}

impl<'base> ResolveSignature<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: Option<ObjectLocation>) -> Result<ObjectLocation, TirError<'base>> {
        simplelog::debug!("Resolving class: <u><b>{}</b></u>", self.name.fragment());

        let (signature_path, class_location) = context.reserve_object_location(Cow::Borrowed(self.name.fragment()), module, self.name.to_range(), self.name.extra.file.clone())?;
        let mut fields = IndexMap::<Cow<'_, str>, ObjectLocation>::default();

        let mut function_signatures = Vec::new();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::Field(field) => {
                    let field_type = get_object_location_or_resolve(context, &field.field_type, module)?;
                    fields.insert((*field.name.fragment()).into(), field_type)
                        .map_or(Ok(()), |_| Err(TirError::already_defined(field.name.to_range(), field.name.extra.file.clone())))?;
                }
                ClassDefinitionFieldAst::Function(function) => {
                    let signature = function.resolve_signature(context, module, Some(class_location.clone()))?;
                    fields.insert((*function.name.fragment()).into(), signature.clone())
                        .map_or(Ok(()), |_| Err(TirError::already_defined(function.name.to_range(), function.name.extra.file.clone())))?;
                    function_signatures.push((signature, function));
                }
            };
        }
        
        let class_signature = ObjectSignature::new(ObjectSignatureValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
            extends: Default::default(),
        }), self.name.extra.file.clone(), self.name.to_range(), None);

        context.publish_object_location(signature_path.clone(), class_signature);

        Ok(class_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, _: ObjectLocation) -> Result<(), TirError<'base>> {
        for field in self.fields.iter() {
            if let ClassDefinitionFieldAst::Function(function) = field {
                let full_name = match &function.location {
                    FunctionDefinitionLocationAst::Module => Cow::Borrowed(*function.name.fragment()),
                    FunctionDefinitionLocationAst::Class(class) => Cow::Owned(format!("{}.{}", class.fragment(), function.name.fragment())),
                };

                let function_location = context.object_signatures.location(format!("{}.{}", module.name(), full_name).as_str()).unwrap();
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
