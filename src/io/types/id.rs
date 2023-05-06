use serde::Serialize;
use std::hash::Hash;

/// Traits required to be implemented for a type to be usable as an `id`
pub trait IdTraits: Eq + Hash + Default + Serialize {}

/// Type used to index and identify I/O device objects
///
/// # Notes
/// Eventually this will be converted to a tuple for storing complex data.
/// This is just a placeholder to establish throughout the codebase.
pub type IdType = u32;

impl IdTraits for IdType {}
