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
use crate::errors::{Error, ErrorKind, Result};
use crate::io::IdTraits;
use crate::storage::collection::MappedCollection;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

/// A trait for creating a specialized `Container` instance
///
/// # Notes
/// Any objects that should be stored _shall_ implement `Containerized` where the intention is to reduce boilerplate
/// code and minimize type definitions.
///
/// # See Also
/// Reference implementations for `io::IOEvent<T>` and `dyn input<T>`
///
/// Provide a specialized key-value container for agnostic to type of objects stored or key-value type.
/// Such stored objects are `input` or `IOEvent` objects. The `Containerized` trait provides a wrapper around a
/// `HashMap` intended to reduce boilerplate code and minimize type definitions.
///
/// # Notes:
///     - Any objects that will be stored _shall_ implement the `Containerized` trait
///     - It is important to note that for objects that implement the `input` trait, the objects should be stored as
///         `dyn input<T>` in order to maintain their dynamic nature. It might also be necessary to use `Box<dyn input<T>`.
///         This allows for a single container to store multiple types of inputs while still being able to call the trait's
///         methods on them.
///
/// # Type Parameters
///
/// * `T`: the type of the objects being stored in the container. This can be any type that implements the `input` trait.
/// * `K`: the type of the keys used to index the objects in the container. This can be any type that implements the `Eq` and `Hash` traits.
///
/// # Examples
///
/// ```
/// struct MyInput {
///     // fields here
/// }
///
/// impl crate::input for MyInput {
///     // implementation here
/// }
///
/// // Create a container to store MyInput objects
/// let container: crate::Container<Box<dyn crate::input<T>>, String> = Containerized::container();
///
/// // Insert a MyInput object into the container
/// let input = MyInput { /* fields */ };
/// container.insert("input1", Box::new(input));
///
/// // Get a reference to the MyInput object in the container
/// let stored_input = container.get("input1").unwrap();
///
/// // Since Containerized is implemented for input, any derived objects should be stored as `dyn input<T>`
/// ```
pub trait Containerized<T, K>
where
    K: IdTraits,
{
    // TODO: add type
    /// Returns a new instance of the `Container` struct for storing objects of type T
    /// which can be accessed by key-values of type K.
    fn container() -> Container<T, K>;
}

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
    // A new Container struct is created with an empty HashMap
    pub fn new() -> Self {
        let inner: HashMap<K, T> = Default::default();
        Container { inner }
    }

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
    fn push(&mut self, key: K, data: T) -> Result<()> {
        match self.inner.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(Error::new(ErrorKind::ContainerError, "Key already exists"))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(data);
                Ok(())
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
