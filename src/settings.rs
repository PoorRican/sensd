use dotenv::dotenv;

const VERSION: &str = "0.0.1-alpha";

pub struct Settings {
    version: String
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();
        let version = std::env::var("VERSION").unwrap_or_else(|_| VERSION.to_string());
        Settings { version }
    }
}
