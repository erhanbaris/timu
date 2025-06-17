use std::{borrow::Cow, collections::HashSet, rc::Rc};

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst}, map::TimuHashMap, nom_tools::{Span, ToRange}, tir::{context::TirContext, object_signature::{GetItem, TypeValue, TypeValueDiscriminants}, resolver::{get_object_location_or_resolve, BuildFullNameLocater}, scope::ScopeLocation, signature::SignaturePath, TirError, TypeSignature}
};

use super::{TypeLocation, ResolveAst};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<TypeSignature<'base>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClassDefinition<'base> {
    pub name: Span<'base>,
    pub fields: TimuHashMap<Cow<'base, str>, TypeLocation>,
    pub extends: HashSet<TypeLocation>,
}

impl PartialEq for ClassDefinition<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl GetItem for ClassDefinition<'_> {
    fn get_item_location(&self, _: &TirContext<'_>, path: &str) -> Option<TypeLocation> {
        self
            .fields
            .get(path).copied()
    }
}

impl<'base> ResolveAst<'base> for ClassDefinitionAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError> {
        simplelog::debug!("Resolving class: <u><b>{}</b></u>", self.name.text);

        let full_name = self.build_full_name(context, BuildFullNameLocater::Scope(scope_location), None);
        let module_ref = context.get_scope(scope_location).expect("Scope not found").module_ref.clone();

        let (signature_path, class_location) = context.reserve_object_location(self.name(), TypeValueDiscriminants::Class, SignaturePath::owned(full_name), &module_ref, self.name.to_range(), self.name.state.file.clone())?;
        let mut fields = TimuHashMap::<Cow<'_, str>, TypeLocation>::default();

        context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").set_current_type(class_location);

        let mut function_signatures = Vec::new();

        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::Field(field) => {
                    let field_type = get_object_location_or_resolve(context, &field.field_type, &module_ref, scope_location)?;

                    fields.validate_insert((*field.name.text).into(), field_type, &field.name)?;
                    context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").add_variable(field.name.clone(), field_type).unwrap();
                }
                ClassDefinitionFieldAst::Function(function) => {
                    let type_name = function.build_full_name(context, BuildFullNameLocater::Module(&module_ref), None);

                    let function_scope_location = context.create_child_scope(type_name.into(), scope_location, None);
                    let function_type_location = function.resolve(context, function_scope_location)?;
                    
                    // Set scope type information
                    context.get_mut_scope(function_scope_location).expect("Scope not found, it is a bug").set_current_type(function_type_location);

                    fields.validate_insert((*function.name.text).into(), function_type_location, &function.name)?;
                    context.get_mut_scope(scope_location).expect("Scope not found, it is a bug").add_variable(function.name.clone(), function_type_location).unwrap();
                    function_signatures.push((function_type_location, function));
                }
            };
        }
        
        let class_signature = TypeSignature::new(TypeValue::Class(ClassDefinition {
            name: self.name.clone(),
            fields,
            extends: Default::default(),
        }), self.name.state.file.clone(), self.name.to_range(), None);

        context.publish_object_location(signature_path.clone(), class_signature);

        Ok(class_location)
    }
    
    fn finish(&self, context: &mut TirContext<'base>, scope: ScopeLocation) -> Result<(), TirError> {

        for field in self.fields.iter() {
            if let ClassDefinitionFieldAst::Function(function) = field {
                let module_ref = context.get_scope(scope).unwrap().module_ref.clone();
                let full_name = format!("{}.{}", module_ref.as_cow(), function.name());
                let search_scope = context.types_scope.get(full_name.as_str()).unwrap();               
                function.finish(context, *search_scope)?;
            }
        }
        
        Ok(())
    }
    
    fn name(&self) -> Cow<'base, str> {
        Cow::Borrowed(self.name.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::TirError};

    #[test]
    fn missing_type() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(a: a): a {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn recursive_type() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { a: test; func test(a: test): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_1() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(this): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_2() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(this, a: test): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn this_location_3() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class test { func test(a: test, this): test {} }".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn call_interface_function() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], r#"
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
    "#.to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }
}
