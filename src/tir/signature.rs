use std::{borrow::Cow, collections::HashMap, fmt::Debug, ops::Range, rc::Rc};

use log::debug;

use crate::file::SourceFile;

#[derive(Debug)]
pub struct Signature<'base, T: Debug, E: Debug = ()> {
    #[allow(dead_code)]
    pub value: T,
    pub file: Rc<SourceFile<'base>>,
    #[allow(dead_code)]
    pub position: Range<usize>,
    pub extra: Option<E>,
}

impl<'base, T, E> Signature<'base, T, E>
where
    T: Debug,
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
pub struct SignatureHolder<'base, T: Debug, E: Debug = ()> {
    signatures: HashMap<Cow<'base, str>, Rc<Signature<'base, T, E>>>,
}

impl<T, E> Default for SignatureHolder<'_, T, E>
where
    T: Debug,
    E: Debug,
{
    fn default() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }
}

impl<'base, T, E> SignatureHolder<'base, T, E>
where
    T: Debug,
    E: Debug,
{
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }

    pub fn add_signature(&mut self, name: Cow<'base, str>, signature: Rc<Signature<'base, T, E>>) -> Option<Rc<Signature<'base, T, E>>> {
        debug!("Adding signature: {}", name);
        self.signatures.insert(name, signature)
    }

    pub fn get(&self, name: &str) -> Option<Rc<Signature<'base, T, E>>> {
        self.signatures.get(name).cloned()
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
