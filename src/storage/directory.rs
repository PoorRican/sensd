use std::path::PathBuf;
use crate::storage::RootPath;

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
