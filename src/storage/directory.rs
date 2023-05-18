use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
        self.0.deref().into()
    }
}

impl<S> From<S> for RootPath
where S: Into<String>{
    fn from(value: S) -> Self {
        Self(Arc::new(PathBuf::from(value.into())))
    }
}

/// Interface for an object with a dedicated directory
///
/// A dedicated directory does not mean that any file should be created,
/// but the object may contain objects that use this dedicated directory.
pub trait Directory {
    /// Getter for global root directory
    fn root_dir(&self) -> RootPath;

    /// Generate or get directory name
    fn dir_name(&self) -> &String;

    /// Get full path to dedicated directory
    fn full_path(&self) -> PathBuf;
}
