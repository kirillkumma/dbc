use std::collections::HashMap;
use std::error::Error;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub databases: HashMap<String, DatabaseConfig>,
}

impl Config {
    pub fn build(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let file_contents = fs::read_to_string(file_path)?;
        let conf: Config = toml::from_str(&file_contents)?;

        Ok(conf)
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub r#type: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}
