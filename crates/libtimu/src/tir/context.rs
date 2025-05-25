use std::{borrow::Cow, ops::Range, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use indexmap::IndexMap;

use crate::file::SourceFile;

use super::{module::ModuleRef, resolver::{AstSignatureLocation, ResolveAst, TypeLocation}, signature::SignaturePath, AstSignature, AstSignatureHolder, Module, ObjectSignatureHolder, TirError, TypeSignature, TypeSignatureHolder};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: IndexMap<Cow<'base, str>, Module<'base>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    #[allow(dead_code)]
    pub types: TypeSignatureHolder<'base>,
    #[allow(dead_code)]
    pub objects: ObjectSignatureHolder<'base>,

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

    pub fn reserve_object_location(&mut self, object_name: Cow<'base, str>, module: &ModuleRef<'base>, position: Range<usize>, source: Rc<SourceFile<'base>>) -> Result<(SignaturePath<'base>, TypeLocation), TirError<'base>> {
        let module = self.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

        // create a new signature path
        let signature_path = SignaturePath::owned(format!("{}.{}", module.path, object_name));

        //add the signature to the context with full path
        let signature_location = self.types.reserve(signature_path.clone(), object_name.clone(), source.clone(), position.clone())
            .map_err(|_| TirError::already_defined(position, source))?;

        //add the signature to the module with only the name
        module.object_signatures.insert(SignaturePath::cow(object_name), signature_location.clone());
        Ok((signature_path, signature_location))
    }

    pub fn publish_object_location(&mut self, name: SignaturePath<'base>, signature: TypeSignature<'base>) {
        self.types.update(name, signature);
    }

    pub fn resolve<T: ResolveAst<'base, Result = TypeLocation>>(&mut self, signature: &T, module: &ModuleRef<'base>) -> Result<TypeLocation, TirError<'base>> {
        signature.resolve(self, module, None)
    }

    pub fn resolve_from_location(&mut self, signature_location: AstSignatureLocation) -> Result<TypeLocation, TirError<'base>> {
        let (signature, module_ref) = self.ast_signatures.get_from_location(signature_location).map(|signature| (signature.value.clone(), signature.extra.clone())).unwrap();
        self.resolve(&signature, module_ref.as_ref().unwrap())
    }
}
