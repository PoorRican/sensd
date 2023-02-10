use crate::errors::{Result};

// trait that expresses an interface to save or load from disk
pub trait Persistent {
    // save data to disk
    fn save(&self, path: &Option<String>) -> Result<()>;

    // load from disk
    fn load(&mut self, path: &Option<String>) -> Result<()>;
}