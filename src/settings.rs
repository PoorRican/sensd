use chrono::Duration;
use dotenv::dotenv;
use std::env::var;

/// Default values
const VERSION: &str = "0.1.0";
const INTERVAL: i64 = 1;

/// Default Filename Prefixes
const LOG_FN_PREFIX: &str = "log_";
const SENSORS_FN_PREFIX: &str = "sensors_";

const DATA_ROOT: &str = "sensd";

/// Struct containing settings loaded from ".env"
pub struct Settings {
    pub version: String,
    interval: Duration,

    pub log_fn_prefix: String,
    pub sensors_fn_prefix: String,

    pub data_root: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            interval: Duration::seconds(INTERVAL),
            log_fn_prefix: LOG_FN_PREFIX.to_string(),
            sensors_fn_prefix: SENSORS_FN_PREFIX.to_string(),
            data_root: DATA_ROOT.to_string(),
        }
    }
}

impl Settings {
    /// Read settings from .env file
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = var("VERSION").unwrap_or_else(|_| String::from(VERSION));
        let interval = Duration::seconds(
            var("INTERVAL")
                .unwrap_or(INTERVAL.to_string())
                .parse::<i64>()
                .unwrap(),
        );
        let log_fn_prefix = var("LOG_FN_PREFIX").unwrap_or_else(|_| String::from(LOG_FN_PREFIX));
        let sensors_fn_prefix =
            var("SENSORS_FN_PREFIX").unwrap_or_else(|_| String::from(SENSORS_FN_PREFIX));
        let data_root = var("DATA_ROOT").unwrap_or_else(|_| String::from(DATA_ROOT));

        Settings {
            version,
            interval,
            log_fn_prefix,
            sensors_fn_prefix,
            data_root,
        }
    }

    pub fn set_root(&mut self, root: String) {
        self.data_root = root
    }

    #[inline]
    pub fn interval(&self) -> &Duration {
        &self.interval
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval
    }
}
