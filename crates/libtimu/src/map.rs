//! Hash map implementation with source location tracking for the Timu compiler.
//!
//! This module provides `TimuHashMap`, a specialized hash map that tracks source
//! locations for error reporting and duplicate definition detection. It wraps
//! `IndexMap` to maintain insertion order and provides validation methods for
//! compiler semantic analysis.

use std::{hash::Hash, marker::PhantomData};

use indexmap::{Equivalent, IndexMap};

use crate::{nom_tools::{Span, ToRange}, tir::TirError};

/// A specialized hash map that tracks source locations for compiler error reporting
/// 
/// This map maintains insertion order using `IndexMap` and stores source location
/// information with each value to enable precise error reporting for duplicate
/// definitions and other semantic errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimuHashMap<'base, K: Hash + Eq, V: ValueTrait<'base>> {
    map: IndexMap<K, Value<'base, V>>,
}

impl<'base, K, V> Default for TimuHashMap<'base, K, V>
where
    K: Hash + Eq,
    V: ValueTrait<'base>
{
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for values that can provide their source location span
/// 
/// This trait must be implemented by all values stored in `TimuHashMap`
/// to enable source location tracking for error reporting.
pub trait ValueTrait<'base> {
    /// Returns the source span where this value was defined
    fn get_span(&self) -> Span<'base>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Value<'base, V: ValueTrait<'base>> {
    value: V,
    position: std::ops::Range<usize>,
    marker: std::marker::PhantomData<&'base ()>,
}

impl<'base, K, V> TimuHashMap<'base, K, V> 
where
    K: Hash + Eq,
    V: ValueTrait<'base>
{
    /// Creates a new empty `TimuHashMap`
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    /// Inserts a key-value pair without validation, returning the previous value if any
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, Value { value, position: 0..0, marker: PhantomData }).map(|item| item.value)
    }

    /// Inserts a key-value pair with duplicate definition validation
    /// 
    /// Returns an error if the key already exists, providing source location
    /// information for both the new and existing definitions.
    pub fn validate_insert(&mut self, key: K, value: V) -> Result<(), TirError> {
        let span = value.get_span();

        match self.map.insert(key, Value { value, position: span.to_range(), marker: PhantomData }) {
            Some(old) => Err(TirError::already_defined(span.to_range(), old.position, span.state.file.clone())),
            None => Ok(())
        }
    }

    /// Returns the first key-value pair in insertion order
    pub fn first(&self) -> Option<(&K, &V)> {
        self.map.first().map(|item| (item.0, &item.1.value))
    }

    /// Returns the last key-value pair in insertion order
    pub fn last(&self) -> Option<(&K, &V)> {
        self.map.last().map(|item| (item.0, &item.1.value))
    }

    /// Gets a reference to the value corresponding to the key
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        K: std::hash::Hash + Eq,
    {
        self.map.get(key).map(|item| &item.value)
    }

    /// Removes a key from the map, returning the value if the key was present
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: std::hash::Hash + Eq,
    {
        self.map.shift_remove(key).map(|item| item.value)
    }

    /// Returns true if the map contains a value for the specified key
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: std::hash::Hash + Eq,
    {
        self.map.contains_key(key)
    }

    /// Returns an iterator over the keys in insertion order
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    /// Returns an iterator over the values in insertion order
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values().map(|item| &item.value)
    }
    
    /// Returns an iterator over key-value pairs in insertion order
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter().map(|item| (item.0, &item.1.value))
    }
    
    /// Returns the number of elements in the map
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    /// Returns true if the map contains no elements
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    
    /// Clears the map, removing all key-value pairs
    pub fn clear(&mut self) {
        self.map.clear();
    }
    
    /// Creates a consuming iterator over key-value pairs in insertion order
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.map.into_iter().map(|item| (item.0, item.1.value))
    }
}