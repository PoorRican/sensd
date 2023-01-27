use std::collections::HashMap;


/// A trait for creating a specialized `Container` instance
///
/// # Notes
/// Any objects that should be stored _shall_ implement `Containerized` where the intention is to reduce boilerplate
/// code and minimize type definitions.
///
/// # See Also
/// Reference implementations for `io::IOEvent<T>` and `dyn Sensor<T>`
///
/// Provide a specialized key-value container for agnostic to type of objects stored or key-value type.
/// Such stored objects are `Sensor` or `IOEvent` objects. The `Containerized` trait provides a wrapper around a
/// `HashMap` intended to reduce boilerplate code and minimize type definitions.
///
/// # Notes:
///     - Any objects that will be stored _shall_ implement the `Containerized` trait
///     - It is important to note that for objects that implement the `Sensor` trait, the objects should be stored as
///         `dyn Sensor<T>` in order to maintain their dynamic nature. It might also be necessary to use `Box<dyn Sensor<T>`.
///         This allows for a single container to store multiple types of sensors while still being able to call the trait's
///         methods on them.
///
/// # Type Parameters
///
/// * `T`: the type of the objects being stored in the container. This can be any type that implements the `Sensor` trait.
/// * `K`: the type of the keys used to index the objects in the container. This can be any type that implements the `Eq` and `Hash` traits.
///
/// # Examples
///
/// ```
/// struct MySensor {
///     // fields here
/// }
///
/// impl crate::Sensor for MySensor {
///     // implementation here
/// }
///
/// // Create a container to store MySensor objects
/// let container: crate::Container<Box<dyn crate::Sensor<T>>, String> = Containerized::container();
///
/// // Insert a MySensor object into the container
/// let sensor = MySensor { /* fields */ };
/// container.insert("sensor1", Box::new(sensor));
///
/// // Get a reference to the MySensor object in the container
/// let stored_sensor = container.get("sensor1").unwrap();
///
/// // Since Containerized is implemented for Sensor, any derived objects should be stored as `dyn Sensor<T>`
/// ```

pub trait Containerized<T, K>
    where K: Eq + std::hash::Hash
{
    // TODO: add type
    /// Returns a new instance of the `Container` struct for storing objects of type T
    /// which can be accessed by key-values of type K.
    fn container() -> Container<T, K>;
}


/// Define a basic interface to interact with underlying data.
/// T is the data type being stored and K is the key type to access stored data.
pub trait Collection<T, K> {
    // Add a key-value pair to the collection and return a boolean indicating if the key already existed in the collection.
    fn add(&mut self, key: K, data: T) -> bool;
    // Get the value associated with the key and return an Option<&T>
    fn get(&self, key: K) -> Option<&T>;
    // Remove the key-value pair associated with the key and return an Option<T>
    fn remove(&mut self, key: K) -> Option<T>;
    // Return a boolean indicating if the collection is empty.
    fn is_empty(&self) -> bool;
    // Return the number of elements in the collection
    fn length(&self) -> usize;
}

// Define a struct `Container` which takes in two types T and K.
pub struct Container<T, K> {
    // The inner field is a HashMap with key type K and value type T
    inner: HashMap<K, T>
}

impl<T, K> Container<T, K> {
    // A new DeviceLog struct is created with an empty HashMap
    pub fn new() -> Self {
        let inner: HashMap<K, T> = Default::default();
        Container { inner }
    }
}

// Implement the Collection trait for the Container struct
impl<T, K> Collection<T, K> for Container<T, K>
    where K: Eq + std::hash::Hash {
    // Add a key-value pair to the collection and return a boolean indicating if the key already existed in the collection.
    // Using `entry` method on the inner HashMap to check if the key already exists in the HashMap
    //  If the key already exists, the returned value is `std::collections::hash_map::Entry::Occupied`, which returns true.
    //  If the key does not exist, the returned value is `std::collections::hash_map::Entry::Vacant`, which inserts the key-value pair into the HashMap and returns false.
    fn add(&mut self, key: K, data: T) -> bool {
        match self.inner.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => true,
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(data);
                false
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
