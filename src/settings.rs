use dotenv::dotenv;

const VERSION: &str = "0.0.1-alpha";

pub struct Settings {
    pub version: String,
}

impl Settings {
    /// Read settings from .env file
    pub fn initialize() -> Self {
        dotenv().ok();
        let version = std::env::var("VERSION").unwrap_or_else(|_| VERSION.to_string());
        Settings { version }
    }
}
