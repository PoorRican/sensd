use dotenv::dotenv;
use std::env::var;
use std::sync::Arc;
use crate::storage::RootPath;

/// Default values
const VERSION: &str = "0.1.0";

/// Default Filename Prefixes
pub const LOG_FN_PREFIX: &str = "log_";

/// Default for top-level directory
pub const DATA_ROOT: &str = "sensd";

#[derive(PartialEq, Debug)]
/// Global runtime settings
pub struct Settings {
    /// Version of `sensd`
    version: String,

    /// Top-level directory
    ///
    /// # See Also
    ///
    /// [`Settings::set_root()`] for mutability limitations.
    root_path: RootPath,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            root_path: Arc::new(DATA_ROOT.to_string()),
        }
    }
}

impl Settings {
    /// Read settings from ".env" file
    ///
    /// If values do not exist in ".env" file, then default values are used. However, ".env" is not
    /// updated.
    ///
    /// # Returns
    ///
    /// Fully initialized [`Settings`]
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = var("VERSION").unwrap_or_else(|_| String::from(VERSION));
        let data_root = var("DATA_ROOT").unwrap_or_else(|_| String::from(DATA_ROOT));

        Settings {
            version,
            root_path: Arc::new(data_root),
        }
    }

    /// Getter for `version`
    ///
    /// # Returns
    ///
    /// Immutable reference to internal [`String`] representing current version of "sensd"
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Getter for `root_path`
    ///
    /// # Returns
    /// A cloned reference to internal `root_path`. If this reference is not dropped (ie: stored
    /// as a field in a struct), then [`Settings::set_root()`] will panic.
    ///
    pub fn root_path(&self) -> RootPath {
        self.root_path.clone()
    }

    /// Setter for `root_path`.
    ///
    /// This method can only be called *before* initialization
    ///
    /// # Parameters
    ///
    /// - `path`: New path of top-level directory. Coerces values into [`String`].
    ///
    /// # Panics
    ///
    /// Panics is thrown if any objects are already using this path. This would
    /// happen if not called before initialization of [`crate::storage::Group`]'s or
    /// [`crate::storage::Log`]'s.
    pub fn set_root<S>(&mut self, path: S)
        where
            S: Into<String>
    {
        if Arc::strong_count(&self.root_path) > 1 {
            panic!("Cannot change `root` while in use")
        }
        self.root_path = Arc::new(path.into())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::settings::Settings;

    #[test]
    /// Asserts that `Settings::set_root()` properly converts using `Into<_>`
    fn set_root_into() {
        let mut settings = Settings::default();
        let new_str = "new path";

        assert_eq!(false, settings.root_path().deref().eq(new_str));

        settings.set_root(new_str);
        assert!(settings.root_path().deref().eq(new_str));

        settings.set_root(new_str.to_string());
        assert!(settings.root_path().deref().eq(new_str));
    }

    #[test]
    #[should_panic]
    /// Assert that panic is thrown if `root_path` has been used.
    fn set_root_panics() {
        let mut settings = Settings::default();
        let _root = settings.root_path();

        settings.set_root("A new string");
    }
}