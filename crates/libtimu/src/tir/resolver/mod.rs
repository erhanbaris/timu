use std::{borrow::Cow, fmt::Debug};

use crate::{ast::TypeNameAst, nom_tools::ToRange};

use super::{ast_signature::AstSignatureValue, context::TirContext, error::TirError, module::ModuleRef, signature::{LocationTrait, SignaturePath}};

pub mod class;
pub mod extend;
pub mod function;
pub mod interface;
pub mod module;
pub mod module_use;
pub mod statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeLocation(#[allow(dead_code)]pub usize);

impl TypeLocation {
    pub const UNDEFINED: Self = TypeLocation(usize::MAX);
}

impl From<usize> for TypeLocation {
    fn from(signature_location: usize) -> Self {
        TypeLocation(signature_location)
    }
}

impl LocationTrait for TypeLocation {
    fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLocation(#[allow(dead_code)]pub usize);

impl ObjectLocation {
    pub const UNDEFINED: Self = ObjectLocation(usize::MAX);
}

impl From<usize> for ObjectLocation {
    fn from(signature_location: usize) -> Self {
        ObjectLocation(signature_location)
    }
}

impl LocationTrait for ObjectLocation {
    fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstSignatureLocation(#[allow(dead_code)]pub usize);

impl From<usize> for AstSignatureLocation {
    fn from(signature_location: usize) -> Self {
        AstSignatureLocation(signature_location)
    }
}

impl LocationTrait for AstSignatureLocation {
    fn get(&self) -> usize {
        self.0
    }
}

pub trait ResolveAst<'base> {
    type Result: Debug + Clone;

    fn resolve(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, parent: Option<TypeLocation>) -> Result<Self::Result, TirError<'base>>;
    fn finish(&self, context: &mut TirContext<'base>, module: &ModuleRef<'base>, location: TypeLocation) -> Result<(), TirError<'base>>;
    fn name(&self) -> Cow<'base, str>;
}

fn build_type_name(type_name: &TypeNameAst) -> String {
    type_name.names.iter().map(|path| *path.fragment()).collect::<Vec<&str>>().join(".")
}

fn get_object_location_or_resolve<'base>(context: &mut TirContext<'base>, type_name: &TypeNameAst<'base>, module: &ModuleRef<'base>) -> Result<TypeLocation, TirError<'base>> {
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

pub fn build_signature_path<'base>(context: &TirContext<'base>, name: &str, module: &ModuleRef<'base>) -> SignaturePath<'base> {
    let module = context.modules.get(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    // create a new signature path
    SignaturePath::owned(format!("{}.{}", module.path, name))
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module: ModuleRef<'base>) -> Result<(), TirError<'base>> {
    simplelog::debug!("<on-red>Building file: {:?}</>", module.as_ref());
    
    if let Some(ast) = context.modules.get(module.as_ref()).and_then(|module| module.ast.clone()) {
        let uses = ast.statements.iter().filter(|statement| statement.is_use()).collect::<Vec<_>>();
        let interfaces = ast.statements.iter().filter(|statement| statement.is_interface()).collect::<Vec<_>>();
        let functions = ast.statements.iter().filter(|statement| statement.is_function()).collect::<Vec<_>>();
        let classes = ast.statements.iter().filter(|statement| statement.is_class()).collect::<Vec<_>>();
        let extends = ast.statements.iter().filter(|statement| statement.is_extend()).collect::<Vec<_>>();

        let mut uses_locations = Vec::new();
        let mut interfaces_locations = Vec::new();
        let mut functions_locations = Vec::new();
        let mut classes_locations = Vec::new();
        let mut extends_locations = Vec::new();

        simplelog::debug!(" - Resolving all uses");
        for use_item in uses.iter() {
            uses_locations.push((use_item, use_item.resolve(context, &module, None)?));
        }

        simplelog::debug!(" - Resolving all interfaces");
        for interface in interfaces.iter() {
            if module.upgrade(context).unwrap().object_signatures.get(interface.name().as_ref()).is_none() {
                interfaces_locations.push((interface, interface.resolve(context, &module, None)?));
            }
        }

        simplelog::debug!(" - Resolving all extends");
        for extend in extends.iter() {
            if module.upgrade(context).unwrap().object_signatures.get(extend.name().as_ref()).is_none() {
                extends_locations.push((extend, extend.resolve(context, &module, None)?));
            }
        }

        simplelog::debug!(" - Resolving all classes");
        for class in classes.iter() {
            if module.upgrade(context).unwrap().object_signatures.get(class.name().as_ref()).is_none() {
                classes_locations.push((class, class.resolve(context, &module, None)?));
            }
        }

        simplelog::debug!(" - Resolving all functions");
        for function in functions.iter() {
            if module.upgrade(context).unwrap().object_signatures.get(function.name().as_ref()).is_none() {
                functions_locations.push((function, function.resolve(context, &module, None)?));
            }
        }
        
        /* Finish */
        simplelog::debug!(" - Finishing all uses");
        for use_item in uses.iter() {
            use_item.finish(context, &module, TypeLocation::UNDEFINED)?;
        }

        simplelog::debug!(" - Finishing all interfaces");
        for interface in interfaces.iter() {
            // create a new signature path
            let module_object = context.modules.get(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
            let signature_path = SignaturePath::owned(format!("{}.{}", module_object.path, interface.name()));
    
            //add the signature to the context with full path
            let location = context.types.location(signature_path.get_raw_path()).unwrap();
            interface.finish(context, &module, location)?;
        }

        simplelog::debug!(" - Finishing all extends");
        for extend in extends.iter() {
            extend.finish(context, &module, TypeLocation::UNDEFINED)?;
        }

        simplelog::debug!(" - Finishing all classes");
        for class in classes.iter() {
            // create a new signature path
            let module_object = context.modules.get(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
            let signature_path = SignaturePath::owned(format!("{}.{}", module_object.path, class.name()));
    
            //add the signature to the context with full path
            let location = context.types.location(signature_path.get_raw_path()).unwrap();
            class.finish(context, &module, location)?;
        }

        simplelog::debug!(" - Finishing all functions");
        for function in functions.iter() {
            // create a new signature path
            let module_object = context.modules.get(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
            let signature_path = SignaturePath::owned(format!("{}.{}", module_object.path, function.name()));
    
            //add the signature to the context with full path
            let location = context.types.location(signature_path.get_raw_path()).unwrap();
            function.finish(context, &module, location)?;
        }
    }

    Ok(())
}

fn find_module<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Option<ModuleRef<'base>> {
    let mut parts = key.as_ref().split('.').peekable();
    let module_name = parts.next()?;
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    match module.ast_imported_modules.get(module_name) {
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


fn try_resolve_moduled_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Result<Option<TypeLocation>, TirError<'base>> {
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

pub fn try_resolve_direct_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Result<Option<TypeLocation>, TirError<'base>> {
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));
    
    if let Some(location) = module.object_signatures.get(key.as_ref()) {
        return Ok(Some(location.clone()));
    }

    let signature_location = match module.ast_imported_modules.get(key.as_ref()) {
        Some(location) => location.clone(),
        None => {
            match module.get_ast_signature(key.as_ref()) {
                Some(location) => location,
                None => match context.types.location(key.as_ref()) {
                    Some(location) => return Ok(Some(location)),
                    None => return Ok(None),
                },
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

pub fn find_ast_signature<'base>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: SignaturePath<'base>) -> Option<AstSignatureLocation> {
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    if let Some(location) = module.ast_signatures.get(key.get_name()) {
        return Some(location.clone());
    }

    match module.ast_imported_modules.get(key.get_name()) {
        Some(location) => Some(location.clone()),
        None => context.get_ast_location(key.get_raw_path()),
    }
}

pub fn try_resolve_signature<'base, K: AsRef<str>>(
    context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K,
) -> Result<Option<TypeLocation>, TirError<'base>> {
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
