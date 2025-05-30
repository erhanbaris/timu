use std::{borrow::Cow, ops::Range, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use indexmap::IndexMap;
use simplelog::debug;

use crate::file::SourceFile;

use super::{module::ModuleRef, resolver::{AstSignatureLocation, ResolveAst, TypeLocation}, scope::{Scope, ScopeLocation}, signature::SignaturePath, AstSignature, AstSignatureHolder, Module, TirError, TypeSignature, TypeSignatureHolder};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: IndexMap<Cow<'base, str>, Module<'base>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    #[allow(dead_code)]
    pub types: TypeSignatureHolder<'base>,
    pub scopes: Vec<Scope<'base>>,
    pub types_scope: IndexMap<Cow<'base, str>, ScopeLocation>,
    pub tmp_type_indexer: AtomicUsize,
}

impl<'base> TirContext<'base> {
    pub fn create_tmp_type(&self) -> String {
        format!("$timu_type_{}$", self.tmp_type_indexer.fetch_add(1, Ordering::SeqCst))
    }

    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<&AstSignature<'base>> {
        self.ast_signatures.get(key.as_ref())
    }

    pub fn get_ast_location<T: AsRef<str>>(&self, key: T) -> Option<AstSignatureLocation> {
        self.ast_signatures.location(key.as_ref())
    }

    pub fn add_ast_signature(&mut self, key: Cow<'base, str>, signature: AstSignature<'base>) -> Result<AstSignatureLocation, AstSignatureLocation> {
        self.ast_signatures.add_signature(SignaturePath::cow(key), signature)
    }

    pub fn reserve_object_location(&mut self, object_name: Cow<'base, str>, signature_path: SignaturePath<'base>, module_ref: &ModuleRef<'base>, position: Range<usize>, source: Rc<SourceFile<'base>>) -> Result<(SignaturePath<'base>, TypeLocation), TirError<'base>> {
        let module = self.modules.get_mut(module_ref.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module_ref.as_ref()));

        debug!("Reserving object location: <u><b>{}</b></u> in module <u><b>{}</b></u>", object_name, module_ref.as_ref());
        //add the signature to the context with full path
        let signature_location = self.types.reserve(signature_path.clone(), object_name.clone(), source.clone(), position.clone())
            .map_err(|_| TirError::already_defined(position, source))?;

        //add the signature to the module with only the name
        module.types.insert(SignaturePath::cow(object_name), signature_location);
        Ok((signature_path, signature_location))
    }

    pub fn publish_object_location(&mut self, name: SignaturePath<'base>, signature: TypeSignature<'base>) {
        self.types.update(name, signature);
    }

    pub fn resolve<T: ResolveAst<'base>>(&mut self, signature: &T, scope_location: ScopeLocation) -> Result<TypeLocation, TirError<'base>> {
        signature.resolve(self, scope_location)
    }

    pub fn resolve_from_location(&mut self, signature_location: AstSignatureLocation, module_ref: &ModuleRef<'base>) -> Result<TypeLocation, TirError<'base>> {
        let signature = self.ast_signatures.get_from_location(signature_location).map(|signature| signature.value.clone()).unwrap();
        let type_name = format!("{}.{}", module_ref.as_ref(), signature.name()); // todo: maybe it will not work with class function
        let scope_location = self.create_scope(type_name.into(), module_ref.clone());
        self.resolve(&signature, scope_location)
    }

    fn inner_create_scope(&mut self, type_info: Cow<'base, str>, module_ref: ModuleRef<'base>, parent_scope: Option<ScopeLocation>, parent_type: Option<TypeLocation>, current_type: Option<TypeLocation>) -> ScopeLocation {
        let scope_location = ScopeLocation(self.scopes.len());
        let mut scope = Scope::new(module_ref, parent_scope, parent_type, scope_location);

        if let Some(current_type) = current_type {
            scope.current_type = current_type;
        }

        self.scopes.push(scope);
        self.types_scope.insert(type_info, scope_location);
        scope_location
    }

    pub fn create_scope(&mut self, type_info: Cow<'base, str>, module_ref: ModuleRef<'base>) -> ScopeLocation {
        self.inner_create_scope(type_info, module_ref, None, None, None)
    }

    pub fn create_child_scope(&mut self, type_info: Cow<'base, str>, parent_scope: ScopeLocation, current_type: Option<TypeLocation>) -> ScopeLocation {
        let parent_scope = self.get_scope(parent_scope).expect("Parent scope not found, it is a bug");
        self.inner_create_scope(type_info, parent_scope.module_ref.clone(), Some(parent_scope.location), Some(parent_scope.current_type), current_type)
    }

    pub fn get_scope(&self, key: ScopeLocation) -> Option<&Scope<'base>> {
        self.scopes.get(key.0)
    }

    pub fn get_mut_scope(&mut self, key: ScopeLocation) -> Option<&mut Scope<'base>> {
        self.scopes.get_mut(key.0)
    }
}
