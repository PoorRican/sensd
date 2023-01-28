use chrono::Duration;
use dotenv::dotenv;
use std::env::var;

/// Default values
const VERSION: &str = "0.0.1-alpha";
const INTERVAL: i64 = 10;

/// Struct containing settings loaded from ".env"
pub struct Settings {
    pub version: String,
    pub interval: Duration,
}

impl Settings {
    /// Read settings from .env file
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = var("VERSION").unwrap_or_else(|_| VERSION.to_string());
        let interval =  Duration::seconds(
            var("INTERVAL").unwrap_or(INTERVAL.to_string()).parse::<i64>().unwrap());

        Settings { version, interval }
    }
}
