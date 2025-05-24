use std::borrow::Cow;

use indexmap::IndexMap;

#[allow(dead_code)]
pub struct Scope<'base> {
    pub variables: IndexMap<Cow<'base, str>, ()>
}