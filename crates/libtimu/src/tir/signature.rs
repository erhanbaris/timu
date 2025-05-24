use std::{borrow::{Borrow, Cow}, fmt::Debug, hash::Hash, ops::Range, rc::Rc};

use indexmap::IndexMap;
use simplelog::debug;

use crate::file::SourceFile;

pub trait LocationTrait: Debug + From<usize> + Clone {
    fn get(&self) -> usize;
}

#[derive(Debug)]
pub struct Signature<'base, T: Debug + AsRef<T> + AsMut<T>, E: Debug = ()> {
    #[allow(dead_code)]
    pub value: T,
    pub file: Rc<SourceFile<'base>>,
    #[allow(dead_code)]
    pub position: Range<usize>,
    pub extra: Option<E>,
}

impl<'base, T, E> Signature<'base, T, E>
where
    T: Debug + AsRef<T> + AsMut<T>,
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
pub enum InnerValue<'base, T: Debug + AsRef<T> + AsMut<T>, E: Debug = ()> {
    Reserved(SignatureReservation<'base>),
    Value(Signature<'base, T, E>),
}

#[derive(Debug, Clone)]
pub struct SignatureReservation<'base> {
    pub name: Cow<'base, str>,
    pub file: Rc<SourceFile<'base>>,
    pub position: Range<usize>,
}

#[derive(Debug)]
pub struct SignatureHolder<'base, T: Debug + AsRef<T> + AsMut<T>, L: LocationTrait, E: Debug = ()> {
    locations: IndexMap<SignaturePath<'base>, usize>,
    signatures: Vec<Option<InnerValue<'base, T, E>>>,
    _marker: std::marker::PhantomData<L>,
}

impl<T, E, L> Default for SignatureHolder<'_, T, L, E> where
    T: Debug + AsRef<T> + AsMut<T>,
    E: Debug, 
    L: LocationTrait {
    fn default() -> Self {
        Self::new()
    }
}

impl<'base, T, E, L> SignatureHolder<'base, T, L, E>
where
    T: Debug + AsRef<T> + AsMut<T>,
    E: Debug,
    L: LocationTrait
{
    pub fn new() -> Self {
        Self {
            signatures: Default::default(),
            locations: IndexMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn inner_add(&mut self, name: SignaturePath<'base>, value: InnerValue<'base, T, E>) -> Result<L, L> {
        self.signatures.push(Some(value));
        let index = self.signatures.len() - 1;
        match self.locations.insert(name, index) {
            Some(_) => Err(index.into()),
            None => Ok(index.into())
        }
    }

    pub fn reserve(&mut self, path: SignaturePath<'base>, name: Cow<'base, str>, file: Rc<SourceFile<'base>>, position: Range<usize>) -> Result<L, L> {
        debug!("Reserve signature: {}", name.as_ref());
        self.inner_add(path, InnerValue::Reserved(SignatureReservation { name, file, position }))
    }

    pub fn update(&mut self, name: SignaturePath<'base>, signature: Signature<'base, T, E>) -> L {
        debug!("Update signature: {}", name.get_name());
        let index = self.locations.get(&name).unwrap_or_else(|| panic!("Signature not found, but this is a bug"));
        self.signatures[*index] = Some(InnerValue::Value(signature));
        (*index).into()
        
    }

    pub fn add_signature(&mut self, name: SignaturePath<'base>, signature: Signature<'base, T, E>) -> Result<L, L> {
        debug!("Adding signature: <u><b>{}</b></u>", name.get_name());
        self.inner_add(name, InnerValue::Value(signature))

    }

    pub fn get(&self, name: &str) -> Option<&Signature<'base, T, E>> {
        match self.locations.get(name) {
            Some(index) => self.get_from_location((*index).into()),
            None => None,
        }
    }

    pub fn get_from_location(&self, location: L) -> Option<&Signature<'base, T, E>> {
        match self.signatures.get(location.get()) {
            Some(Some(InnerValue::Value(signature))) => Some(signature),
            Some(Some(InnerValue::Reserved(_))) => None,
            _ => None,
        }
    }

    pub fn empty_from_location(&mut self, location: L) -> Option<Signature<'base, T, E>> {
        self.signatures.get(location.get())?;

        match self.signatures[location.get()].take() {
            Some(InnerValue::Value(signature)) => Some(signature),
            Some(InnerValue::Reserved(_)) => None,
            _ => None,
        }
    }

    pub fn get_inner_value_from_location(&self, location: L) -> Option<&InnerValue<'base, T, E>> {
        match self.signatures.get(location.get()) {
            Some(Some(inner)) => Some(inner),
            _ => None,
        }
    }

    pub fn get_mut_from_location(&mut self, location: L) -> Option<&mut Signature<'base, T, E>> {
        match self.signatures.get_mut(location.get()) {
            Some(Some(InnerValue::Value(signature))) => Some(signature),
            Some(Some(InnerValue::Reserved(_))) => None,
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn location(&self, name: &str) -> Option<L> {
        self.locations.get(name).map(|index| (*index).into())
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
