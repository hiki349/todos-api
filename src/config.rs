use dotenv::dotenv;
use std::env;

pub struct Config {
    pub addr: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            addr: env::var("ADDR").unwrap_or("0.0.0.0:3000".to_string()),
        }
    }
}
