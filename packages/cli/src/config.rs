use std::path::{self, Path, PathBuf};

use crate::template::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub templates: Vec<Template>,
}

impl Config {
    pub fn new() -> Self {
        Config { templates: vec![] }
    }

    pub fn from_json(json: &str) -> Config {
        let config: Config = serde_json::from_str(&json).unwrap();
        config
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn load(directory: &PathBuf) -> Option<Config> {
        let path = directory.join("config.json");
        let json = std::fs::read_to_string(path);
        if json.is_err() {
            return None;
        }
        let json = json.unwrap();
        let config = Config::from_json(&json);
        Some(config)
    }

    pub fn load_or_new(directory: &PathBuf) -> Config {
        let config = Config::load(directory);
        if config.is_none() {
            return Config::new();
        }
        config.unwrap()
    }

    pub fn save(&self, directory: &PathBuf) -> Result<(), ()> {
        let path = directory.join("config.json");
        let json = self.to_json();
        if json.is_err() {
            return Err(());
        }
        let json = json.unwrap();
        let result = std::fs::create_dir_all(directory);
        if result.is_err() {
            return Err(());
        }

        let result = std::fs::write(path, json);
        if result.is_err() {
            return Err(());
        }
        Ok(())
    }
}
