/// Data structures and interfaces to store data
///
/// The main workhorses that provide functionality are `Containerized` and `Container`. The `Containerized`
/// trait is used to define a container type, `Container`, that can store a collection of objects of a specific
/// type. The trait is implemented for various types, such as `dyn input<T>` and `IOEvent<T>`.
///
/// The `Containerized` trait defines a single method, `container()`, which returns a new instance of the
/// `Container` struct, specific to the type that the trait is implemented for. For example, when the trait is
/// implemented for `dyn input<T>` (any object that implements `input<T>`), the `container()` method returns a
/// new instance of `Container<Box<dyn input<T>>, K>`.
///
/// The `Container` struct is generic over two types, `T` and `K`. `T` represents the type of the objects that
/// will be stored within the container, and `K` represents the type of the key used to identify the objects
/// within the container. In the case of the `dyn input<T>` implementation, `T` is `Box<dyn input<T>>` and `K`
/// is an arbitrary type.
///
/// In summary, the `Containerized` trait allows for the creation of a `Container` which can
/// store a collection of objects of a specific type `T`, and identified by a specific key type `K`. The relationship
/// between `Containerized` and `Container` is that `Containerized` defines how the `Container` should be created
/// and used for a specific type, while `Container` actually holds the collection of objects.
use crate::errors::{Error, ErrorKind, ErrorType};
use crate::io::IdTraits;
use crate::storage::collection::MappedCollection;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

/// Define a struct `Container` which takes in two types T and K.
/// This container is meant to store any complex type and is stored with an arbitrary key.
/// The key only needs to be hashable.
#[derive(Debug, Serialize, Deserialize)]
pub struct Container<T, K>
where
    K: IdTraits,
{
    // The inner field is a HashMap with key type K and value type T
    pub inner: HashMap<K, T>,
}

impl<T, K: IdTraits> Default for Container<T, K> {
    fn default() -> Self {
        let inner: HashMap<K, T> = Default::default();
        Self { inner }
    }
}

impl<T, K: IdTraits> Container<T, K> {
    /// Return a readonly reference to stored HashMap
    pub fn iter(&self) -> Iter<'_, K, T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, T> {
        self.inner.iter_mut()
    }
}

/// Generic interface for interacting with mapped data
impl<T, K: IdTraits> MappedCollection<T, K> for Container<T, K> {
    /// Add a key-value pair to the collection and return a boolean indicating if the value has been added to the collection.
    /// Using `entry` method on the inner HashMap to check if the key already exists in the HashMap
    ///  - If the key already exists, the returned value is `std::collections::hash_map::Entry::Occupied`, which returns false.
    ///  - If the key does not exist, the returned value is `std::collections::hash_map::Entry::Vacant`, which inserts the key-value pair into the HashMap and returns true.
    fn push(&mut self, key: K, data: T) -> Result<&mut T, ErrorType> {
        match self.inner.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(Error::new(ErrorKind::ContainerError, "Key already exists"))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                Ok(entry.insert(data))
            }
        }
    }

    // Get the value associated with the key and return an Option<&T>
    fn get(&self, key: K) -> Option<&T> {
        self.inner.get(&key)
    }

    // Remove the key-value pair associated with the key and return an Option<T>
    fn remove(&mut self, key: K) -> Option<T> {
        self.inner.remove(&key)
    }

    // Return a boolean indicating if the collection is empty.
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // Return the number of elements in the collection
    fn length(&self) -> usize {
        self.inner.len()
    }
}
