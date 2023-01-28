use chrono::Duration;
use dotenv::dotenv;

/// Default values
const VERSION: &str = "0.0.1-alpha";
const INTERVAL: i32 = 10;

/// Struct containing settings loaded from ".env"
pub struct Settings {
    pub version: String,
    pub interval: Duration,
}

impl Settings {
    /// Read settings from .env file
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = std::env::var("VERSION").unwrap_or_else(|_| VERSION.to_string());
        let interval =  Duration::seconds(i64::from(std::env::var("INTERVAL").unwrap_or(INTERVAL.to_string())));
        Settings { version, interval }
    }
}
