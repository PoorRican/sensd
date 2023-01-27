use std::collections::HashMap;


// Define a trait Collection, which takes in two types T and K. T is the data type and K is the key type.
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
