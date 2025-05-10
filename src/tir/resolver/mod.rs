use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

use crate::{ast::TypeNameAst, nom_tools::Span, tir::ast_signature::AstSignatureValue};

use super::{ObjectSignature, context::TirContext, error::TirError, module::Module};

pub mod class_definition;
pub mod function_definition;
pub mod interface_definition;
pub mod module_definition;
pub mod module_use;

pub trait ResolveSignature<'base> {
    type Item;

    fn resolve(&self, context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Item, TirError<'base>>;
}

fn build_type_name(ast: &TypeNameAst) -> String {
    ast.names.iter().map(|path| *path.fragment()).collect::<Vec<&str>>().join(".")
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module: Rc<RefCell<Module<'base>>>) -> Result<(), TirError<'base>> {
    let mut module = module.borrow_mut();
    log::debug!("Building file: {:?}", module.path);

    if let Some(ast) = &module.ast {
        let uses = ast.get_uses().collect::<Vec<_>>();
        let functions = ast.get_functions().collect::<Vec<_>>();

        for use_item in uses {
            use_item.resolve(context, &mut module)?;
        }

        for function in functions {
            function.resolve(context, &mut module)?;
        }
    }

    Ok(())
}

fn find_module<'base, K: AsRef<str>>(module: &mut RefMut<'_, Module<'base>>, key: K) -> Option<Rc<RefCell<Module<'base>>>> {
    let mut parts = key.as_ref().split('.').peekable();
    let module_name = parts.next()?;

    match module.imported_modules.get(module_name) {
        Some(found_module) => {
            if let AstSignatureValue::Module(found_module) = &found_module.value {
                Some(found_module.clone())
            } else {
                None
            }
        }
        None => module.modules.get(module_name).cloned(),
    }
}

pub fn try_resolve_signature<'base, K: AsRef<str>>(
    context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>, key: K,
) -> Result<Option<Rc<ObjectSignature<'base>>>, TirError<'base>> {
    // Check if the key is a module name
    if key.as_ref().contains('.') {
        let mut parts = key.as_ref().split('.').peekable();
        let module_name = match parts.next() {
            Some(module_name) => module_name,
            None => return Ok(None),
        };

        let found_module = match find_module(module, module_name) {
            Some(found_module) => found_module,
            None => return Ok(None),
        };

        let mut found_module = found_module.borrow_mut();
        let signature_name = parts.collect::<Vec<_>>().join(".");
        return try_resolve_signature(context, &mut found_module, signature_name);
    }

    if let Some(signature) = module.object_signatures.get(key.as_ref()) {
        return Ok(Some(signature));
    }

    let signature = match module.imported_modules.get(key.as_ref()) {
        Some(signature) => signature.clone(),
        None => match module.ast_signatures.get(key.as_ref()) {
            Some(signature) => signature.clone(),
            None => return Ok(None),
        },
    };

    Ok(Some(signature.value.resolve(context, module)?))
}

#[derive(Debug)]
pub struct SpannedValue<'a, T: Debug> {
    #[allow(dead_code)]
    pub span: Span<'a>,
    pub value: T,
}

impl<T> std::ops::Deref for SpannedValue<'_, T>
where
    T: Debug,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_ast, process_code};

    #[test]
    fn found_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], "class a {} func test(a: a): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn cross_reference1() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use source1.testclass1;
    func testfunction1(): testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;

        Ok(())
    }

    #[test]
    fn cross_reference2() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use source1;
    func testfunction1(): source1.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;

        Ok(())
    }

    #[test]
    fn cross_reference3() -> Result<(), ()> {
        let ast_1 = process_code(vec!["test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use test1;
    func testfunction1(): test1.source1.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference4() -> Result<(), ()> {
        let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use base1;
    func testfunction1(): base1.test1.source1.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference5() -> Result<(), ()> {
        let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1;
    func testfunction1(): test1.source1.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference6() -> Result<(), ()> {
        let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1;
    func testfunction1(): source1.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias1() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use source1 as abc;
    func testfunction1(): abc.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias2() -> Result<(), ()> {
        let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1 as test;
    func testfunction1(): test.testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias3() -> Result<(), ()> {
        let ast_1 = process_code(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1.testclass1 as test;
func testfunction1(a: test): test {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }
}
