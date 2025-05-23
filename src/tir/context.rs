use std::{borrow::Cow, ops::Range, rc::Rc};

use indexmap::IndexMap;

use crate::file::SourceFile;

use super::{module::ModuleRef, object_signature::ObjectSignatureValue, resolver::{AstLocation, ResolveSignature, ObjectLocation}, signature::{Signature, SignaturePath}, AstSignature, AstSignatureHolder, Module, ObjectSignatureHolder, TirError};

#[derive(Debug, Default)]
pub struct TirContext<'base> {
    pub modules: IndexMap<Cow<'base, str>, Module<'base>>,
    pub ast_signatures: AstSignatureHolder<'base>,
    #[allow(dead_code)]
    pub object_signatures: ObjectSignatureHolder<'base>,
}

impl<'base> TirContext<'base> {
    pub fn get_ast_signature<T: AsRef<str>>(&self, key: T) -> Option<&AstSignature<'base>> {
        self.ast_signatures.get(key.as_ref())
    }

    pub fn get_ast_location<T: AsRef<str>>(&self, key: T) -> Option<AstLocation> {
        self.ast_signatures.location(key.as_ref())
    }

    pub fn add_ast_signature(&mut self, key: Cow<'base, str>, signature: AstSignature<'base>) -> Result<AstLocation, AstLocation> {
        self.ast_signatures.add_signature(SignaturePath::cow(key), signature)
    }

    pub fn reserve_object_location(&mut self, object_name: Cow<'base, str>, module: &ModuleRef<'base>, position: Range<usize>, source: Rc<SourceFile<'base>>) -> Result<(SignaturePath<'base>, ObjectLocation), TirError<'base>> {
        let module = self.modules.get_mut(module.as_ref()).unwrap_or_else(|| panic!("Module({}) not found, but this is a bug", module.as_ref()));

        // create a new signature path
        let signature_path = SignaturePath::owned(format!("{}.{}", module.path, object_name));

        //add the signature to the context with full path
        let signature_location = self.object_signatures.reserve(signature_path.clone())
            .map_err(|_| TirError::already_defined(position, source))?;

        //add the signature to the module with only the name
        module.object_signatures.insert(SignaturePath::cow(object_name), signature_location.clone());
        Ok((signature_path, signature_location))
    }

    pub fn update_object_location(&mut self, name: SignaturePath<'base>, signature: Signature<'base, ObjectSignatureValue<'base>>) {
        self.object_signatures.update(name, signature);
    }

    pub fn resolve<T: ResolveSignature<'base>>(&mut self, signature: &T, module: &ModuleRef<'base>) -> Result<ObjectLocation, TirError<'base>> {
        signature.resolve(self, module, None)
    }

    pub fn resolve_from_location(&mut self, signature_location: AstLocation) -> Result<ObjectLocation, TirError<'base>> {
        let (signature, module_ref) = self.ast_signatures.get_from_location(signature_location).map(|signature| (signature.value.clone(), signature.extra.clone())).unwrap();
        self.resolve(&signature, module_ref.as_ref().unwrap())
    }
}
