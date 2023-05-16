use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Specialized type for representing the root directory.
///
/// This type should be used to build root, not be used to represent any
/// sub-directory.
pub type RootPath = Arc<String>;

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
    fn full_path(&self) -> PathBuf {
        let root = self.root_dir();
        let path = Path::new(root.as_str());

        path.join(self.dir_name().as_str())
    }
}
