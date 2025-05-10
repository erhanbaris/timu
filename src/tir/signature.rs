use std::{borrow::Cow, collections::HashMap, fmt::Debug, ops::Range, rc::Rc};

use log::debug;

use crate::file::SourceFile;

#[derive(Debug)]
pub struct Signature<'base, T: Debug> {
    #[allow(dead_code)]
    pub value: T,
    pub file: Rc<SourceFile<'base>>,
    #[allow(dead_code)]
    pub position: Range<usize>,
}

impl<'base, T> Signature<'base, T>
where
    T: Debug,
{
    pub fn new(value: T, file: Rc<SourceFile<'base>>, position: Range<usize>) -> Self {
        Self {
            value,
            file,
            position,
        }
    }
}

#[derive(Debug)]
pub struct SignatureHolder<'base, T: Debug> {
    signatures: HashMap<Cow<'base, str>, Rc<Signature<'base, T>>>,
}

impl<T> Default for SignatureHolder<'_, T>
where
    T: Debug,
{
    fn default() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }
}

impl<'base, T> SignatureHolder<'base, T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }

    pub fn add_signature(&mut self, name: Cow<'base, str>, signature: Rc<Signature<'base, T>>) -> Option<Rc<Signature<'base, T>>> {
        debug!("Adding signature: {}", name);
        self.signatures.insert(name, signature)
    }

    pub fn get(&self, name: &str) -> Option<Rc<Signature<'base, T>>> {
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
