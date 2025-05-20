use std::{borrow::{Borrow, Cow}, fmt::Debug, hash::Hash, ops::Range, rc::Rc};

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
    locations: IndexMap<SignaturePath<'base>, usize>,
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

    fn inner_add(&mut self, name: SignaturePath<'base>, signature: Option<Rc<Signature<'base, T, E>>>) -> Result<SignatureLocation, SignatureLocation> {
        self.signatures.push(signature);
        let index = self.signatures.len() - 1;
        match self.locations.insert(name, index) {
            Some(_) => Err(SignatureLocation(index)),
            None => Ok(SignatureLocation(index))
        }
    }

    pub fn reserve(&mut self, name: SignaturePath<'base>) -> Result<SignatureLocation, SignatureLocation> {
        debug!("Reserve signature: {}", name.get_name());
        self.inner_add(name, None)
    }

    pub fn update(&mut self, name: SignaturePath<'base>, signature: Rc<Signature<'base, T, E>>) -> SignatureLocation {
        debug!("Update signature: {}", name.get_name());
        let index = self.locations.get(&name).unwrap_or_else(|| panic!("Signature not found, but this is a bug"));
        self.signatures[*index] = Some(signature);
        SignatureLocation(*index)

    }

    pub fn add_signature(&mut self, name: SignaturePath<'base>, signature: Rc<Signature<'base, T, E>>) -> Result<SignatureLocation, SignatureLocation> {
        debug!("Adding signature: <u><b>{}</b></u>", name.get_name());
        self.inner_add(name, Some(signature))

    }

    pub fn get(&self, name: &str) -> Option<Rc<Signature<'base, T, E>>> {
        self.locations.get(name).and_then(|index| self.signatures[*index].clone())
    }

    pub fn get_from_location(&self, location: SignatureLocation) -> Option<Rc<Signature<'base, T, E>>> {
        self.signatures.get(location.0).and_then(|signature| signature.clone())
    }

    #[allow(dead_code)]
    pub fn location(&self, name: &str) -> Option<SignatureLocation> {
        self.locations.get(name).map(|index| SignatureLocation(*index))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SignaturePathType {
    Direct,
    Moduled,
}

#[derive(Debug, Hash, Clone)]
struct InnerSignaturePath<'base> {
    full_path: Cow<'base, str>, 
    signature_type: SignaturePathType,
    modules: Vec<Range<usize>>,
    name: Range<usize>
}

impl PartialEq for SignaturePath<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.full_path == other.0.full_path
    }
}

impl Hash for SignaturePath<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.full_path.hash(state);
    }
}

impl Eq for SignaturePath<'_> {}

#[derive(Debug, Clone)]
pub struct SignaturePath<'base>(InnerSignaturePath<'base>);

impl<'base> SignaturePath<'base> {
    fn build_path(full_path: Cow<'base, str>) -> InnerSignaturePath<'base> {
        match full_path.find('.') {
            Some(index) => {

                let mut position = 0;
                let mut start_index = 0;
                let mut end_index = index;

                let mut modules = Vec::new();
                modules.push(start_index..end_index);
                end_index += 1; // Skip the dot

                while let Some(new_index) = full_path[end_index..].find('.') {
                    start_index = end_index;
                    
                    position += new_index + 1;
                    end_index = position + new_index;

                    modules.push(start_index..end_index);
                    end_index += 1; // Skip the dot
                }
                
                let name = end_index..full_path.len();
                
                InnerSignaturePath {
                    full_path,
                    signature_type: SignaturePathType::Moduled,
                    modules,
                    name
                }
            },
            None => {
                let name = 0..full_path.len();
                InnerSignaturePath {
                    full_path,
                    signature_type: SignaturePathType::Direct,
                    modules: Vec::new(),
                    name
                }
            }
        }
    }

    pub fn cow(path: Cow<'base, str>) -> SignaturePath<'base> {
        SignaturePath(Self::build_path(path))
    }

    pub fn borrowed(path: &'base str) -> SignaturePath<'base> {
        SignaturePath(Self::build_path(Cow::Borrowed(path)))
    }

    pub fn owned(path: String) -> SignaturePath<'base> {
        SignaturePath(Self::build_path(Cow::Owned(path)))

    }

    #[allow(dead_code)]
    pub fn get_raw_path(&self) -> &Cow<'base, str> {
        &self.0.full_path
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> SignaturePathType {
        self.0.signature_type
    }

    #[allow(dead_code)]
    pub fn get_modules(&self) -> &Vec<Range<usize>> {
        &self.0.modules
    }

    #[allow(dead_code)]
    pub fn build_string(&self, range: Range<usize>) -> &str {
        &self.0.full_path[range]
    }

    pub fn get_name(&self) -> &str {
        &self.0.full_path[self.0.name.clone()]
    }
}

impl Borrow<str> for SignaturePath<'_> {
    fn borrow(&self) -> &str {
        self.0.full_path.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_code, tir::signature::SignaturePathType};

    use super::SignaturePath;

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

    #[test]
    fn direct_signature_path() -> Result<(), ()> {

        let path = SignaturePath::borrowed("test");
        assert_eq!(path.get_type(), SignaturePathType::Direct);
        assert_eq!(path.get_raw_path(), "test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.get_modules(), &Vec::new());
        
        Ok(())
    }

    #[test]
    fn moduled_signature_path_1() -> Result<(), ()> {

        let path = SignaturePath::borrowed("module.test");
        assert_eq!(path.get_type(), SignaturePathType::Moduled);
        assert_eq!(path.get_raw_path(), "module.test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.build_string(path.get_modules()[0].clone()), "module");
        
        let path = SignaturePath::borrowed("module1.module2.test");
        assert_eq!(path.get_type(), SignaturePathType::Moduled);
        assert_eq!(path.get_raw_path(), "module1.module2.test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.build_string(path.get_modules()[0].clone()), "module1");
        assert_eq!(path.build_string(path.get_modules()[1].clone()), "module2");
        
        let path = SignaturePath::borrowed("module1.module2.module3.test");
        assert_eq!(path.get_type(), SignaturePathType::Moduled);
        assert_eq!(path.get_raw_path(), "module1.module2.module3.test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.build_string(path.get_modules()[0].clone()), "module1");
        assert_eq!(path.build_string(path.get_modules()[1].clone()), "module2");
        assert_eq!(path.build_string(path.get_modules()[2].clone()), "module3");
        
        let path = SignaturePath::borrowed("module1.module2.module3.module4.test");
        assert_eq!(path.get_type(), SignaturePathType::Moduled);
        assert_eq!(path.get_raw_path(), "module1.module2.module3.module4.test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.build_string(path.get_modules()[0].clone()), "module1");
        assert_eq!(path.build_string(path.get_modules()[1].clone()), "module2");
        assert_eq!(path.build_string(path.get_modules()[2].clone()), "module3");
        assert_eq!(path.build_string(path.get_modules()[3].clone()), "module4");
        
        Ok(())
    }
}
