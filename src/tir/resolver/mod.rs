use std::borrow::Cow;

use crate::{ast::TypeNameAst, nom_tools::ToRange};

use super::{ast_signature::AstSignatureValue, context::TirContext, error::TirError, module::ModuleRef};

pub mod class_definition;
pub mod extend_definition;
pub mod function_definition;
pub mod interface_definition;
pub mod module_definition;
pub mod module_use;

#[derive(Debug, Clone)]
pub struct SignatureLocation(#[allow(dead_code)]pub usize);
impl From<usize> for SignatureLocation {
    fn from(signature_location: usize) -> Self {
        SignatureLocation(signature_location)
    }
}

pub trait ResolveSignature<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>>;
    fn name(&self) -> Cow<'base, str>;
}

fn build_type_name(type_name: &TypeNameAst) -> String {
    type_name.names.iter().map(|path| *path.fragment()).collect::<Vec<&str>>().join(".")
}

fn build_object_type<'base>(context: &mut TirContext<'base>, type_name: &TypeNameAst<'base>, module: &ModuleRef<'base>) -> Result<SignatureLocation, TirError<'base>> {
    let type_name_str = build_type_name(type_name);
    let field_type = match try_resolve_signature(context, module, type_name_str.as_str())? {
        Some(field_type) => field_type,
        None => {
            return Err(TirError::TypeNotFound {
                source: type_name.names.last().unwrap().extra.file.clone(),
                position: type_name.to_range(),
            });
        }
    };

    Ok(field_type)
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module: ModuleRef<'base>) -> Result<(), TirError<'base>> {
    simplelog::debug!("<on-red>Building file: {:?}</>", module.as_ref());
    
    if let Some(ast) = context.modules.get(module.as_ref()).and_then(|module| module.ast.clone()) {
        let uses = ast.get_uses().collect::<Vec<_>>();
        let interaces = ast.get_interfaces().collect::<Vec<_>>();
        let functions = ast.get_functions().collect::<Vec<_>>();
        let classes = ast.get_classes().collect::<Vec<_>>();
        let extends = ast.get_extends().collect::<Vec<_>>();

        simplelog::debug!(" - Resolving all uses");
        for use_item in uses {
            use_item.resolve(context, &module)?;
        }

        simplelog::debug!(" - Resolving all interfaces");
        for interace in interaces {
            if module.upgrade(context).unwrap().object_signatures.get(interace.name().as_ref()).is_none() {
                interace.resolve(context, &module)?;
            }
        }

        simplelog::debug!(" - Resolving all classes");
        for class in classes {
            if module.upgrade(context).unwrap().object_signatures.get(class.name().as_ref()).is_none() {
                class.resolve(context, &module)?;
            }
        }

        simplelog::debug!(" - Resolving all extends");
        for extend in extends {
            if module.upgrade(context).unwrap().object_signatures.get(extend.name().as_ref()).is_none() {
                extend.resolve(context, &module)?;
            }
        }

        simplelog::debug!(" - Resolving all functions");
        for function in functions {
            if module.upgrade(context).unwrap().object_signatures.get(function.name().as_ref()).is_none() {
                function.resolve(context, &module)?;
            }
        }
    }

    Ok(())
}

fn find_module<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Option<ModuleRef<'base>> {
    let mut parts = key.as_ref().split('.').peekable();
    let module_name = parts.next()?;
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    match module.imported_modules.get(module_name) {
        Some(found_module) => {
            let signature = context.ast_signatures.get_from_location(found_module.clone()).map(|module| module.value.as_ref());
            if let Some(AstSignatureValue::Module(found_module)) = signature {
                Some(found_module.clone())
            } else {
                None
            }
        }
        None => module.modules.get(module_name).cloned(),
    }
}


fn try_resolve_moduled_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Result<Option<SignatureLocation>, TirError<'base>> {
    // Check if the key is a module name
    let mut parts = key.as_ref().split('.').peekable();
    let module_name = match parts.next() {
        Some(module_name) => module_name,
        None => return Ok(None),
    };

    let found_module = match find_module(context, module, module_name) {
        Some(found_module) => found_module,
        None => return Ok(None),
    };

    let signature_name = parts.collect::<Vec<_>>().join(".");
    try_resolve_signature(context, &found_module, signature_name)
}

pub fn try_resolve_direct_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Result<Option<SignatureLocation>, TirError<'base>> {
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
    
    if let Some(location) = module.object_signatures.get(key.as_ref()) {
        return Ok(Some(location.clone()));
    }

    let signature_location = match module.imported_modules.get(key.as_ref()) {
        Some(location) => location.clone(),
        None => {
            match module.get_ast_signature(key.as_ref()) {
                Some(location) => location,
                None => return Ok(None),
            }
        },
    };

    let signature = match context.ast_signatures.get_from_location(signature_location.clone()) {
        Some(signature) => signature,
        None => return Ok(None),
    };

    if let Some(location) = signature.extra.as_ref().unwrap().upgrade(context).unwrap().object_signatures.get(signature.value.name().as_ref()) {
        return Ok(Some(location.clone()));
    }

    Ok(Some(context.resolve_from_location(signature_location)?))
}

pub fn try_resolve_signature<'base, K: AsRef<str>>(
    context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K,
) -> Result<Option<SignatureLocation>, TirError<'base>> {
    // Check if the key has a module name
    match key.as_ref().contains('.') {
        true => try_resolve_moduled_signature(context, module, key),
        false => try_resolve_direct_signature(context, module, key)
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
