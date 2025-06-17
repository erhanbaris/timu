use std::{borrow::{Borrow, Cow}, fmt::Debug, hash::Hash, ops::Range};

use indexmap::IndexMap;
use simplelog::debug;

use crate::file::SourceFile;

use super::TirError;

pub trait LocationTrait: Debug + From<usize> + Clone {
    fn get(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Signature<T: Debug + Clone + AsRef<T> + AsMut<T>, E: Debug + Clone> {
    #[allow(dead_code)]
    pub value: T,
    pub file: SourceFile,
    #[allow(dead_code)]
    pub position: Range<usize>,
    pub extra: Option<E>,
}

impl<T, E> Signature<T, E>
where
    T: Debug + Clone + AsRef<T> + AsMut<T>,
    E: Debug + Clone,
{
    pub fn new(value: T, file: SourceFile, position: Range<usize>, extra: Option<E>) -> Self {
        Self {
            value,
            file,
            position,
            extra,
        }
    }

    pub fn new_with_extra(value: T, file: SourceFile, position: Range<usize>, extra: E) -> Self {
        Self {
            value,
            file,
            position,
            extra: Some(extra),
        }
    }
}

#[derive(Debug)]
pub enum SignatureInfo<'base, T: Debug + Clone + AsRef<T> + AsMut<T>, U: Clone + Debug, E: Debug + Clone = ()> {
    Reserved(SignatureReservation<'base, U>),
    Value(Signature<T, E>),
}

#[derive(Debug, Clone)]
pub struct SignatureReservation<'base, U: Clone + Debug> {
    pub name: Cow<'base, str>,
    pub file: SourceFile,
    pub position: Range<usize>,
    pub type_shadow: U
}

#[derive(Debug)]
pub struct SignatureHolder<'base, T: Debug + Clone + PartialEq + AsRef<T> + AsMut<T>, U: Clone + Debug, L: LocationTrait, E: Debug + Clone = ()> {
    locations: IndexMap<SignaturePath<'base>, usize>,
    signatures: Vec<Option<SignatureInfo<'base, T, U, E>>>,
    _marker: std::marker::PhantomData<L>,
}

impl<T, U, E, L> Default for SignatureHolder<'_, T, U, L, E> where
    T: Debug + Clone + PartialEq + AsRef<T> + AsMut<T>,
    U: Clone + Debug,
    E: Debug + Clone, 
    L: LocationTrait {
    fn default() -> Self {
        Self::new()
    }
}

impl<'base, T, U, E, L> SignatureHolder<'base, T, U, L, E>
where
    T: Debug + Clone + PartialEq + AsRef<T> + AsMut<T>,
    U: Clone + Debug,
    E: Debug + Clone,
    L: LocationTrait
{
    pub fn new() -> Self {
        Self {
            signatures: Default::default(),
            locations: IndexMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn inner_add(&mut self, name: SignaturePath<'base>, value: SignatureInfo<'base, T, U, E>) -> Result<L, TirError> {
        self.signatures.push(Some(value));
        let index = self.signatures.len() - 1;
        match self.locations.insert(name, index) {
            Some(old) => {
                let old_position = match self.signatures[old].as_ref().unwrap() {
                    SignatureInfo::Reserved(reservation) => reservation.position.clone(),
                    SignatureInfo::Value(signature) => signature.position.clone(),
                };

                let (new_position, file) = match self.signatures[index].as_ref().unwrap() {
                    SignatureInfo::Reserved(reservation) => (reservation.position.clone(), reservation.file.clone()),
                    SignatureInfo::Value(signature) =>(signature.position.clone(), signature.file.clone()),
                };

                Err(TirError::already_defined(new_position, old_position, file))
            },
            None => Ok(index.into())
        }
    }

    pub fn reserve(&mut self, path: SignaturePath<'base>, name: Cow<'base, str>, type_shadow: U, file: SourceFile, position: Range<usize>) -> Result<L, TirError> {
        debug!("Reserve signature: {}", name.as_ref());
        self.inner_add(path, SignatureInfo::Reserved(SignatureReservation { name, file, position, type_shadow }))
    }

    pub fn update(&mut self, name: SignaturePath<'base>, signature: Signature<T, E>) -> L {
        debug!("Update signature: {}", name.get_name());
        let index = self.locations.get(&name).unwrap_or_else(|| panic!("Signature not found, but this is a bug"));
        self.signatures[*index] = Some(SignatureInfo::Value(signature));
        (*index).into()
        
    }

    pub fn add_signature(&mut self, name: SignaturePath<'base>, signature: Signature<T, E>) -> Result<L, TirError> {
        debug!("Adding signature: <u><b>{}</b></u>", name.get_name());
        self.inner_add(name, SignatureInfo::Value(signature))
    }

    pub fn find_by_value(&self, value: &T) -> Option<L> {
        for (index, signature) in self.signatures.iter().enumerate() {
            if let Some(SignatureInfo::Value(signature)) = signature {
                if &signature.value == value {
                    return Some(index.into())
                }
            }
        }

        None
    }

    pub fn get(&self, name: &str) -> Option<&Signature<T, E>> {
        match self.locations.get(name) {
            Some(index) => self.get_from_location((*index).into()),
            None => None,
        }
    }

    pub fn get_from_location(&self, location: L) -> Option<&Signature<T, E>> {
        match self.signatures.get(location.get()) {
            Some(Some(SignatureInfo::Value(signature))) => Some(signature),
            _ => None,
        }
    }

    pub fn get_reserve_from_location(&self, location: L) -> Option<&SignatureReservation<'_, U>> {
        match self.signatures.get(location.get()) {
            Some(Some(SignatureInfo::Reserved(reserve))) => Some(reserve),
            _ => None,
        }
    }

    pub fn from_location(&self, location: L) -> Option<&SignatureInfo<'_, T, U, E>> {
        match self.signatures.get(location.get()) {
            Some(Some(value)) => Some(value),
            _ => None,
        }
    }

    pub fn take_from_location(&mut self, location: L) -> Option<Signature<T, E>> {
        self.signatures.get(location.get())?;

        match self.signatures[location.get()].take() {
            Some(SignatureInfo::Value(signature)) => Some(signature),
            _ => None,
        }
    }

    pub fn get_signature_from_location(&self, location: L) -> Option<&SignatureInfo<'base, T, U, E>> {
        match self.signatures.get(location.get()) {
            Some(Some(inner)) => Some(inner),
            _ => None,
        }
    }

    pub fn get_mut_from_location(&mut self, location: L) -> Option<&mut Signature<T, E>> {
        match self.signatures.get_mut(location.get()) {
            Some(Some(SignatureInfo::Value(signature))) => Some(signature),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn location(&self, name: &str) -> Option<L> {
        self.locations.get(name).map(|index| (*index).into())
    }
}


/*
#[derive(Debug)]
pub struct Holder<K: AsRef<str> + Borrow<str> + Hash + Eq + Clone, T: Debug + PartialEq + AsRef<T> + AsMut<T>, L: LocationTrait> {
    locations: IndexMap<K, usize>,
    signatures: Vec<T>,
    _marker: std::marker::PhantomData<L>,
}

impl<K, T, L> Default for Holder<K, T, L> where
    K: AsRef<str> + Borrow<str> + Hash + Eq + Clone, 
    T: Debug + PartialEq + AsRef<T> + AsMut<T> + Clone,
    L: LocationTrait {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, T, L> Holder<K, T, L>
where
    K: AsRef<str> + Borrow<str> + Hash + Eq + Clone, 
    T: Debug + PartialEq + AsRef<T> + AsMut<T> + Clone,
    L: LocationTrait
{
    pub fn new() -> Self {
        Self {
            signatures: Default::default(),
            locations: IndexMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self, name: K, value: T) -> L {
        let index = self.locations.get(name.as_ref()).unwrap_or_else(|| panic!("Value not found, but this is a bug"));
        self.signatures[*index] = value;
        (*index).into()
    }

    pub fn find_or_insert(&mut self, value: &T) -> L {
        match self.find_by_value(value) {
            Some(location) => location,
            None => {
                self.signatures.push(value.clone());
                (self.signatures.len() - 1).into()
            }
        }
    }

    pub fn add_with_key(&mut self, name: K, value: &T) -> Result<L, L> {
        let index = self.find_or_insert(value);
        match self.locations.insert(name, index.get()) {
            Some(_) => Err(index),
            None => Ok(index)
        }
    }

    pub fn find_by_value(&self, search: &T) -> Option<L> {
        self.signatures
            .iter()
            .position(|value| search == value)
            .map(|index| index.into())
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        match self.locations.get(name) {
            Some(index) => self.get_from_location((*index).into()),
            None => None,
        }
    }

    pub fn get_from_location(&self, location: L) -> Option<&T> {
        self.signatures.get(location.get())
    }

    #[allow(dead_code)]
    pub fn location(&self, name: &str) -> Option<L> {
        self.locations.get(name).map(|index| (*index).into())
    }
}
*/

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
    use crate::{file::SourceFile, nom_tools::State, process_code, tir::{signature::SignaturePathType, TirError}};

    use super::SignaturePath;

    #[test]
    fn signature_generation() -> Result<(), TirError> {
        let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} func testfunction(): testclass {} interface testinterface {}".to_string()));
        let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source; use source.testclass; use source.testfunction; use source.testinterface;".to_string()));
        
        let ast_1 = process_code(&state_1)?;
        let ast_2 = process_code(&state_2)?;
        crate::tir::build(vec![ast_1.into(), ast_2.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn dublicate_signatures() -> Result<(), TirError> {
        let state = State::new(SourceFile::new(vec!["source".into()], " class test {} func test(): void {} interface test {}".to_string()));
        let ast = process_code(&state)?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn direct_signature_path() -> Result<(), TirError> {

        let path = SignaturePath::borrowed("test");
        assert_eq!(path.get_type(), SignaturePathType::Direct);
        assert_eq!(path.get_raw_path(), "test");
        assert_eq!(path.get_name(), "test");
        assert_eq!(path.get_modules(), &Vec::new());
        
        Ok(())
    }

    #[test]
    fn moduled_signature_path_1() -> Result<(), TirError> {

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
