use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::storage::Directory;

#[derive(PartialEq, Clone, Debug)]
/// Specialized type for representing the root directory.
///
/// This type should be used to build root, not be used to represent any
/// sub-directory.
pub struct RootPath(Arc<PathBuf>);

impl RootPath {
    pub fn new() -> Self {
        RootPath(Arc::new(PathBuf::new()))
    }
    pub fn join<P>(&self, path: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.0.join(path)
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    pub fn deref(&self) -> PathBuf {
        self.0.to_path_buf()
    }
}

impl Into<PathBuf> for RootPath {
    fn into(self) -> PathBuf {
        self.deref()
    }
}

impl<S> From<S> for RootPath
    where S: AsRef<Path>
{
    fn from(value: S) -> Self {
        Self(Arc::new(PathBuf::from(value.as_ref())))
    }
}

pub trait RootDirectory: Directory {
    /// Getter for global root directory
    ///
    /// # Returns
    ///
    /// Path to root directory
    fn root_dir(&self) -> RootPath;

    /// Setter for `root_path` that can be used as a builder function.
    ///
    /// # Parameters
    ///
    /// - `root`: New path to global root dir
    ///
    /// # Returns
    ///
    /// Ownership of `Self`. This is to be used as a builder function using method chaining.
    fn set_root<P>(mut self, path: P) -> Self
        where
            Self: Sized,
            P: AsRef<Path>
    {
        self.set_root_ref(path);
        self
    }

    fn set_root_ref<P>(&mut self, path: P) -> &mut Self
        where
            P: AsRef<Path>;
}