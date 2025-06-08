use std::hash::Hash;

use indexmap::{Equivalent, IndexMap};

use crate::{nom_tools::{Span, ToRange}, tir::TirError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimuHashMap<K: Hash + Eq, V> {
    map: IndexMap<K, Value<V>>,
}

impl<K: Hash + Eq, V> Default for TimuHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Value<V> {
    value: V,
    positon: std::ops::Range<usize>,
}

impl<K: Hash + Eq, V> TimuHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, Value { value, positon: 0..0 }).map(|item| item.value)
    }

    pub fn validate_insert<'base>(&mut self, key: K, value: V, span: &Span<'base>) -> Result<(), TirError> {
        match self.map.insert(key, Value { value, positon: span.to_range() }) {
            Some(old) => Err(TirError::already_defined(span.to_range(), old.positon, span.state.file.clone())),
            None => Ok(())
        }
    }

    pub fn first(&self) -> Option<(&K, &V)> {
        self.map.first().map(|item| (item.0, &item.1.value))
    }

    pub fn last(&self) -> Option<(&K, &V)> {
        self.map.last().map(|item| (item.0, &item.1.value))
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        K: std::hash::Hash + Eq,
    {
        self.map.get(key).map(|item| &item.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: std::hash::Hash + Eq,
    {
        self.map.shift_remove(key).map(|item| item.value)
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: std::hash::Hash + Eq,
    {
        self.map.contains_key(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values().map(|item| &item.value)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter().map(|item| (item.0, &item.1.value))
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.map.into_iter().map(|item| (item.0, item.1.value))
    }
}