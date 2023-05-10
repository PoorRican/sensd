use dotenv::dotenv;
use std::env::var;
use std::sync::Arc;

/// Default values
const VERSION: &str = "0.1.0";

/// Default Filename Prefixes
pub const LOG_FN_PREFIX: &str = "log_";

pub const DATA_ROOT: &str = "sensd";

pub type RootPath = Arc<String>;

#[derive(PartialEq, Debug)]
/// Struct containing settings loaded from ".env"
pub struct Settings {
    pub version: String,

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
    /// Read settings from .env file
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = var("VERSION").unwrap_or_else(|_| String::from(VERSION));
        let data_root = var("DATA_ROOT").unwrap_or_else(|_| String::from(DATA_ROOT));

        Settings {
            version,
            root_path: Arc::new(data_root),
        }
    }

    pub fn root_path(&self) -> RootPath {
        self.root_path.clone()
    }

    /// # Panics
    ///
    /// Panics is thrown if any objects are already using this path.
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
    fn set_root_panics() {
        let mut settings = Settings::default();
        let _root = settings.root_path();

        settings.set_root("A new string");
    }
}