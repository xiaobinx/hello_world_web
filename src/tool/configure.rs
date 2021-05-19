use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    port: i32,
    key: String,
    static_dir: Option<String>,
    log_level: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = serde_json::from_str(&content).unwrap();
        config
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn key(&self) -> &[u8] {
        self.key.as_bytes()
    }

    pub fn static_dir(&self) -> &str {
        match &self.static_dir {
            Some(static_dir) => static_dir.as_str(),
            None => "./static",
        }
    }

    pub fn log_level(&self) -> &str {
        match &self.log_level {
            Some(log_level) => log_level.as_str(),
            None => "info",
        }
    }
}

#[test]
fn tet() {
    let config = Config::from_file("./config.json");
    println!("{:?}", config);
}
