use std::{borrow::Cow, fmt::Debug, ops::Range, rc::Rc};

use indexmap::IndexMap;
use simplelog::debug;

use crate::file::SourceFile;

use super::resolver::SignatureLocation;

#[derive(Debug)]
pub struct Signature<'base, T: Debug + AsRef<T>, E: Debug = ()> {
    #[allow(dead_code)]
    pub value: T,
    pub file: Rc<SourceFile<'base>>,
    #[allow(dead_code)]
    pub position: Range<usize>,
    pub extra: Option<E>,
}

impl<'base, T, E> Signature<'base, T, E>
where
    T: Debug + AsRef<T>,
    E: Debug,
{
    pub fn new(value: T, file: Rc<SourceFile<'base>>, position: Range<usize>) -> Self {
        Self {
            value,
            file,
            position,
            extra: None,
        }
    }

    pub fn new_with_extra(value: T, file: Rc<SourceFile<'base>>, position: Range<usize>, extra: E) -> Self {
        Self {
            value,
            file,
            position,
            extra: Some(extra),
        }
    }
}

#[derive(Debug)]
pub struct SignatureHolder<'base, T: Debug + AsRef<T>, E: Debug = ()> {
    locations: IndexMap<Cow<'base, str>, usize>,
    signatures: Vec<Option<Rc<Signature<'base, T, E>>>>,
}

impl<T, E> Default for SignatureHolder<'_, T, E> where
T: Debug + AsRef<T>,
E: Debug{
    fn default() -> Self {
        Self::new()
    }
}

impl<'base, T, E> SignatureHolder<'base, T, E>
where
    T: Debug + AsRef<T>,
    E: Debug,
{
    pub fn new() -> Self {
        Self {
            signatures: Default::default(),
            locations: IndexMap::new(),
        }
    }

    fn inner_add(&mut self, name: Cow<'base, str>, signature: Option<Rc<Signature<'base, T, E>>>) -> Result<SignatureLocation, SignatureLocation> {
        self.signatures.push(signature);
        let index = self.signatures.len() - 1;
        match self.locations.insert(name, index) {
            Some(_) => Err(SignatureLocation(index)),
            None => Ok(SignatureLocation(index))
        }
    }

    pub fn reserve(&mut self, name: Cow<'base, str>) -> Result<SignatureLocation, SignatureLocation> {
        debug!("Reserve signature: {}", name);
        self.inner_add(name, None)
    }

    pub fn update(&mut self, name: Cow<'base, str>, signature: Rc<Signature<'base, T, E>>) -> SignatureLocation {
        debug!("Update signature: {}", name);
        let index = self.locations.get(name.as_ref()).unwrap_or_else(|| panic!("Signature not found, but this is a bug"));
        self.signatures[*index] = Some(signature);
        SignatureLocation(*index)

    }

    pub fn add_signature(&mut self, name: Cow<'base, str>, signature: Rc<Signature<'base, T, E>>) -> Result<SignatureLocation, SignatureLocation> {
        debug!("Adding signature: <u><b>{}</b></u>", name);
        self.inner_add(name, Some(signature))

    }

    pub fn get(&self, name: &str) -> Option<Rc<Signature<'base, T, E>>> {
        self.locations.get(name).and_then(|index| self.signatures[*index].clone())
    }

    pub fn get_from_location(&self, location: SignatureLocation) -> Option<Rc<Signature<'base, T, E>>> {
        self.signatures.get(location.0).and_then(|signature| signature.clone())
    }

    pub fn location(&self, name: &str) -> Option<SignatureLocation> {
        self.locations.get(name).map(|index| SignatureLocation(*index))
    }
}

#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn signature_generation() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source".into()], " class testclass {} func testfunction(): testclass {} interface testinterface {}")?;
        let ast_2 = process_code(vec!["lib".into()], "use source; use source.testclass; use source.testfunction; use source.testinterface;")?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn dublicate_signatures() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], " class test {} func test(): void {} interface test {}")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
