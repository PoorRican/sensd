use std::path::{Path, PathBuf};

pub trait Document {
    /// Getter for path of dedicated directory for sister nodes
    ///
    /// # Returns
    ///
    /// An [`Option`] containing:
    ///
    /// - `Some` with [`PathBuf`] to dedicated directory
    /// - `None` if no dedicated directory has been associated
    fn dir(&self) -> Option<&PathBuf>;

    /// Builder method for setting directory
    ///
    /// # Parameters
    ///
    /// - `path`: Path to directory containing this document
    ///
    /// # Returns
    ///
    /// Ownership of object with `dir` field set
    fn set_dir<P>(mut self, path: P) -> Self
        where
            Self: Sized,
            P: AsRef<Path>
    {
        self.set_dir_ref(path);
        self
    }

    /// Setter for dedicated directory
    ///
    /// # Parameters
    ///
    /// - `path`: Path to directory containing this document
    ///
    /// # Returns
    ///
    /// Mutable reference to object with `dir` field set. Allows for method chaining.
    fn set_dir_ref<P>(&mut self, path: P) -> &mut Self
        where
            Self: Sized,
            P: AsRef<Path>;

    /// Getter for developing filename
    ///
    /// # Returns
    ///
    /// Fully formatted [`String`] to use as filename
    fn filename(&self) -> String;

    /// Getter for full path to file in filesystem
    ///
    /// # Returns
    ///
    /// Full path to access file including filename
    fn full_path(&self) -> PathBuf {
        self.dir()
            .expect("No directory is associated")
            .join(self.filename())
    }

    /// Check to see if file exists
    ///
    /// # Returns
    ///
    /// A `bool` which is:
    ///
    /// - `true` if file exists regardless of permissions.
    /// - `false` if file does not exist
    fn exists(&self) -> bool {
        self.full_path()
            .exists()
    }
}