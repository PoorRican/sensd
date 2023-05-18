use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use crate::name::Name;

/// Interface for an object with a dedicated directory
///
/// A dedicated directory does not mean that any file should be created,
/// but the object may contain objects that use this dedicated directory.
pub trait Directory: Name {
    /// Getter for parent directory
    ///
    /// Parent directory should be stored as a field and is used to build [`Directory::full_path()`]
    ///
    /// # Returns
    ///
    /// An [`Option`] that is:
    ///
    /// - `Some` with [`PathBuf`] of parent directory
    /// - `None` if no parent directory has been set
    fn parent_dir(&self) -> Option<PathBuf>;

    /// Builder method for setting parent dir
    ///
    /// # Parameters
    ///
    /// - `path`: `PathBuf` returned from [`Directory::full_path()`] of parent object..
    fn set_parent_dir<P>(mut self, path: P) -> Self
        where
            Self: Sized,
            P: AsRef<Path>
    {
        self.set_parent_dir_ref(path);
        self
    }

    fn set_parent_dir_ref<P>(&mut self, path: P) -> &mut Self
        where
            P: AsRef<Path>;

    /// Generate or get directory name
    ///
    /// By default, the [`Name::name()`] is used as directory name
    ///
    /// # Returns
    ///
    /// A reference to `String` representing name.
    ///
    /// # See Also
    ///
    /// [`Name::name()`]
    fn dir_name(&self) -> &String {
        self.name()
    }

    /// Get full path to dedicated directory
    ///
    /// # Returns
    ///
    /// `PathBuf` of full path to dedicated directory
    ///
    /// # Panics
    ///
    /// A panic is thrown if no parent directory is set
    fn full_path(&self) -> PathBuf {
        self.parent_dir()
            .expect("No parent directory has been set")
            .join(self.dir_name())
    }

    /// Builder method that creates dedicated directory
    ///
    /// If directory already exists, then this method silently fails.
    ///
    /// # Panics
    ///
    /// This method panics if an error occurs when creating directory (other than directory
    /// already existing). This could happen if write permissions are misconfigured.
    ///
    /// # Returns
    ///
    /// Ownership of `Self`, allowing method chaining.
    fn init_dir(self) -> Self
        where
            Self: Sized
    {
        let path = self.full_path();
        match path.exists() {
            true => (),
            false => {
                create_dir_all(path).expect("Could not create dedicated directory");
            }
        };
        self
    }
}
