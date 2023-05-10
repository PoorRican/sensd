use dotenv::dotenv;
use std::env::var;

/// Default values
const VERSION: &str = "0.1.0";

/// Default Filename Prefixes
pub const LOG_FN_PREFIX: &str = "log_";

pub const DATA_ROOT: &str = "sensd";

#[derive(PartialEq, Debug)]
/// Struct containing settings loaded from ".env"
pub struct Settings {
    pub version: String,

    pub data_root: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            data_root: DATA_ROOT.to_string(),
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
            data_root,
        }
    }

    pub fn set_root(&mut self, root: String) {
        self.data_root = root
    }
}
