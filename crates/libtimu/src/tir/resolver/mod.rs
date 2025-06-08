use std::{borrow::Cow, fmt::Debug};

use function::FunctionResolveError;
use libtimu_macros::TimuError;
use simplelog::debug;
use statement::FunctionCallError;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{ast::TypeNameAst, nom_tools::ToRange};

use super::{ast_signature::AstSignatureValue, context::TirContext, error::TirError, module::ModuleRef, scope::{ScopeError, ScopeLocation}, signature::{LocationTrait, SignaturePath}};

pub mod class;
pub mod extend;
pub mod function;
pub mod interface;
pub mod module;
pub mod module_use;
pub mod statement;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(PartialEq, Debug, Copy, Clone, Eq, PartialOrd, Ord, Hash)]
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

pub enum BuildFullNameLocater<'a, 'base> {
    Scope(ScopeLocation),
    Module(&'a ModuleRef<'base>),
}

pub trait ResolveAst<'base> {
    fn resolve(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<TypeLocation, TirError>;
    fn finish(&self, context: &mut TirContext<'base>, scope_location: ScopeLocation) -> Result<(), TirError>;
    fn name(&self) -> Cow<'base, str>;

    fn build_full_name<'a>(&self, context: &TirContext<'_>, locater: BuildFullNameLocater<'a, 'base>, parent: Option<TypeLocation>) -> String {
        let module = match locater {
            BuildFullNameLocater::Scope(scope_location) => {
                let module_ref = context.get_scope(scope_location).expect("Scope not found").module_ref.clone();
                module_ref.upgrade(context).unwrap()
            },
            BuildFullNameLocater::Module(module_ref) => module_ref.upgrade(context).unwrap(),
        };

        match parent {
            Some(parent) if parent != TypeLocation::UNDEFINED => {
                let parent_signature = context.types.get_from_location(parent).unwrap();
                format!("{}.{}.{}", module.path, parent_signature.value.get_name(), self.name())
            },
            _ => format!("{}.{}", module.path, self.name()),
        }
    }
}

fn build_type_name(type_name: &TypeNameAst) -> String {
    type_name.names.iter().map(|path| path.text).collect::<Vec<&str>>().join(".")
}

fn get_object_location_or_resolve<'base>(context: &mut TirContext<'base>, type_name: &TypeNameAst<'base>, module: &ModuleRef<'base>) -> Result<TypeLocation, TirError> {
    let type_name_str = build_type_name(type_name);
    let field_type = match try_resolve_signature(context, module, type_name_str.as_str())? {
        Some(field_type) => field_type,
        None => {
            return Err(TirError::type_not_found(context, type_name.to_string(), type_name.to_range(), type_name.names.last().unwrap().state.file.clone()));
        }
    };

    Ok(field_type)
}

pub fn build_signature_path<'base>(context: &TirContext<'base>, name: &str, module: &ModuleRef<'base>) -> SignaturePath<'base> {
    let module = context.modules.get(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    // create a new signature path
    SignaturePath::owned(format!("{}.{}", module.path, name))
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module_ref: ModuleRef<'base>) -> Result<(), TirError> {
    simplelog::debug!("<on-red>Building file: {:?}</>", module_ref.as_ref());
    
    if let Some(ast) = context.modules.get(module_ref.as_ref()).and_then(|module| module.ast.clone()) {
        let uses = ast.statements.iter().filter(|statement| statement.is_use()).collect::<Vec<_>>();
        let interfaces = ast.statements.iter().filter(|statement| statement.is_interface()).collect::<Vec<_>>();
        let functions = ast.statements.iter().filter(|statement| statement.is_function()).collect::<Vec<_>>();
        let classes = ast.statements.iter().filter(|statement| statement.is_class()).collect::<Vec<_>>();
        let extends = ast.statements.iter().filter(|statement| statement.is_extend()).collect::<Vec<_>>();

        /* Resolving */
        simplelog::debug!(" - Resolving all uses");
        execute_vector_resolve(context, module_ref.clone(), &uses)?;

        simplelog::debug!(" - Resolving all interfaces");
        execute_vector_resolve(context, module_ref.clone(), &interfaces)?;

        simplelog::debug!(" - Resolving all extends");
        execute_vector_resolve(context, module_ref.clone(), &extends)?;

        simplelog::debug!(" - Resolving all classes");
        execute_vector_resolve(context, module_ref.clone(), &classes)?;

        simplelog::debug!(" - Resolving all functions");
        execute_vector_resolve(context, module_ref.clone(), &functions)?;
        
        /* Finish */
        simplelog::debug!(" - Resolving all uses");
        execute_vector_finish(context, module_ref.clone(), uses)?;

        simplelog::debug!(" - Finishing all interfaces");
        execute_vector_finish(context, module_ref.clone(), interfaces)?;

        simplelog::debug!(" - Finishing all extends");
        execute_vector_finish(context, module_ref.clone(), extends)?;

        simplelog::debug!(" - Finishing all classes");
        execute_vector_finish(context, module_ref.clone(), classes)?;

        simplelog::debug!(" - Finishing all functions");
        execute_vector_finish(context, module_ref.clone(), functions)?;
    }

    Ok(())
}

fn execute_vector_resolve<'base, T: ResolveAst<'base>>(context: &mut TirContext<'base>, module_ref: ModuleRef<'base>, asts: &Vec<&T>) -> Result<(), TirError> {
    for item in asts.iter() {
        execute_resolve(context, module_ref.clone(), *item)?;
    }
    Ok(())
}

fn execute_resolve<'base, T: ResolveAst<'base>>(context: &mut TirContext<'base>, module_ref: ModuleRef<'base>, ast: &T) -> Result<(), TirError> {
    if module_ref.upgrade(context).unwrap().types.get(ast.name().as_ref()).is_none() {
        let type_name = format!("{}.{}", module_ref.as_ref(), ast.name());
        let scope_location = context.create_scope(type_name.into(), module_ref.clone());
        ast.resolve(context, scope_location)?;
    }
    Ok(())
}

fn execute_vector_finish<'base, T: ResolveAst<'base>>(context: &mut TirContext<'base>, module_ref: ModuleRef<'base>, asts: Vec<&T>) -> Result<(), TirError> {
    for item in asts.into_iter() {
        execute_finish(context, module_ref.clone(), item)?;
    }
    Ok(())
}

fn execute_finish<'base, T: ResolveAst<'base>>(context: &mut TirContext<'base>, module_ref: ModuleRef<'base>, ast: &T) -> Result<(), TirError> {
    let type_name = format!("{}.{}", module_ref.as_ref(), ast.name());
    let scope_location = context.types_scope[type_name.as_str()];
    ast.finish(context, scope_location)?;
    Ok(())
}

fn find_module<'base, K: AsRef<str> + ?Sized>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: &K) -> Option<ModuleRef<'base>> {
    let mut parts = key.as_ref().split('.').peekable();
    let module_name = parts.next()?;
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    match module.ast_imported_modules.get(module_name) {
        Some(found_module) => {
            let signature = context.ast_signatures.get_from_location(*found_module).map(|module| module.value.as_ref());
            if let Some(AstSignatureValue::Module(found_module)) = signature {
                Some(found_module.clone())
            } else {
                None
            }
        }
        None => match module.modules.get(module_name).cloned() { // Lets search module in the current module
            Some(found_module) => Some(found_module),
            None => context.modules.get(module_name).map(|module| module.get_ref().clone()), // Lets search module in the context
        },
    }
}


fn try_resolve_moduled_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K) -> Result<Option<TypeLocation>, TirError> {
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

pub fn try_resolve_direct_signature<'base, K: AsRef<str>>(context: &mut TirContext<'base>, module_ref: &ModuleRef<'base>, key: K) -> Result<Option<TypeLocation>, TirError> {
    let module = context.modules.get_mut(module_ref.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module_ref.as_ref()));
    debug!("Searching <on-blue>{}</> in: <on-red>{}</> module", key.as_ref(), module.path.as_ref());
    
    if let Some(location) = module.types.get(key.as_ref()) {
        debug!("Found <on-blue>{}</> in: <on-red>{}</> module", key.as_ref(), module.path.as_ref());
        return Ok(Some(*location));
    }

    let signature_location = match module.ast_imported_modules.get(key.as_ref()) {
        Some(location) => *location,
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

    let signature = match context.ast_signatures.get_from_location(signature_location) {
        Some(signature) => signature,
        None => return Ok(None),
    };

    let module = signature.extra.as_ref().unwrap().upgrade(context).unwrap();
    debug!("Module: <on-red>{}</>, Type values: <on-blue>{:?}</>", module.path.as_ref(), module.types.values());

    if let Some(location) = module.types.get(signature.value.name().as_ref()) {
        debug!("Found <on-blue>{}</> in: <on-red>{}</> module", key.as_ref(), module.path.as_ref());
        return Ok(Some(*location));
    }

    Ok(Some(context.resolve_from_location(signature_location, &module.get_ref())?))
}

pub fn find_ast_signature<'base>(context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: SignaturePath<'base>) -> Option<AstSignatureLocation> {
    let module = context.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

    if let Some(location) = module.ast_signatures.get(key.get_name()) {
        return Some(*location);
    }

    match module.ast_imported_modules.get(key.get_name()) {
        Some(location) => Some(*location),
        None => context.get_ast_location(key.get_raw_path()),
    }
}

pub fn try_resolve_signature<'base, K: AsRef<str>>(
    context: &mut TirContext<'base>, module: &ModuleRef<'base>, key: K,
) -> Result<Option<TypeLocation>, TirError> {
    // Check if the key has a module name
    match key.as_ref().contains('.') {
        true => try_resolve_moduled_signature(context, module, key),
        false => try_resolve_direct_signature(context, module, key)
    }
}


#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum ResolverError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionCall(#[from] Box<FunctionCallError>),  

    #[error(transparent)]
    #[diagnostic(transparent)]
    Scope(#[from] Box<ScopeError>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    FunctionResolve(#[from] Box<FunctionResolveError>),
}

impl From<ResolverError> for TirError {
    fn from(value: ResolverError) -> Self {
        TirError::ResolverError(Box::new(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};

    #[test]
    fn found_type() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], "class a {} func test(variable: a): a {} ".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn cross_reference1() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use source1.testclass1;
    func testfunction1(): testclass1 {}"#.to_string(),
        ));
        
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;

        Ok(())
    }

    #[test]
    fn cross_reference2() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use source1;
    func testfunction1(): source1.testclass1 {}"#.to_string()));

        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;

        Ok(())
    }

    #[test]
    fn cross_reference3() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use test1;
    func testfunction1(): test1.source1.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference4() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use base1;
    func testfunction1(): base1.test1.source1.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference5() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1;
    func testfunction1(): test1.source1.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn cross_reference6() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1;
    func testfunction1(): source1.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias1() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use source1 as abc;
    func testfunction1(): abc.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias2() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1 as test;
    func testfunction1(): test.testclass1 {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }

    #[test]
    fn import_alias3() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["base1".into(), "test1".into(), "source1".into()], " class testclass1 {} ".to_string()));
        let state_9 = State::new(SourceFile::new(
            vec!["sub".into(), "source9".into()],
            r#"use base1.test1.source1.testclass1 as test;
func testfunction1(a: test): test {}"#.to_string()));
        let ast_1 = process_code(&state_1)?;
        let ast_9 = process_code(&state_9)?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;
        Ok(())
    }
}
