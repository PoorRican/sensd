use std::hash::Hash;
use crate::errors;

/// Define a basic interface to interact with underlying data.
/// T is the data type being stored and K is the key type to access stored data.
///
/// Generics should be agnostic to underlying implementation aside from the fact that key should
/// can be used as key in `HashMap`. `MappedCollection` sole purpose is a universal interface for interacting
/// with mapping stored data.
pub trait MappedCollection<T, K: Eq + Hash> {
    /// Add a key-value pair to the collection and return a boolean indicating if the addition was successful.
    /// If the key already existed, then `false` is returned.
    fn push(&mut self, key: K, data: T) -> errors::Result<()>;

    /// Access object by key
    /// Since key might not exist, an option is returned.
    fn get(&self, key: K) -> Option<&T>;

    /// Remove the key-value pair associated with the key.
    /// The removed data is returned.
    fn remove(&mut self, key: K) -> Option<T>;

    /// Return a boolean indicating if the collection is empty.
    fn is_empty(&self) -> bool;

    /// Return the number of elements in the collection
    fn length(&self) -> usize;
}
