use std::env;

pub struct Config {
    pub addr: String,
    pub db_conn: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            addr: env::var("ADDR").unwrap_or("0.0.0.0:3000".to_string()),
            db_conn: env::var("DB_CONN").unwrap_or("db connection is required".to_string()),
        }
    }
}
