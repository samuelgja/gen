use crate::{
    case_util::CaseType,
    template::{Template, TemplateCaseType, TemplateFolder},
};

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{self, Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    pub templates: Vec<TemplateFolder>,
    pub config: ConfigFile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct ConfigFile {
    pub case_type: TemplateCaseType,
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile {
            case_type: TemplateCaseType::new(),
        }
    }

    pub fn load_template_config(directory: &PathBuf) -> ConfigFile {
        let config_path = directory.join("config.json");
        let config_content = fs::read_to_string(config_path);

        if config_content.is_err() {
            return ConfigFile::new();
        }
        let config_content = config_content.unwrap();
        let config: ConfigFile = serde_json::from_str(&config_content).unwrap();
        config
    }

    pub fn save_template_config(&self, directory: &PathBuf) {
        let config_path = directory.join("config.json");
        let config_content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(config_path, config_content).unwrap();
    }
}

impl Config {
    pub fn load_template_folders(directory: &PathBuf) -> Config {
        // search directory and get all folders
        let entries = fs::read_dir(directory).unwrap();
        let folders = entries
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    return Some(TemplateFolder {
                        name: path.file_name().unwrap().to_str().unwrap().to_string(),
                        path,
                    });
                }
                None
            })
            .collect::<Vec<_>>();

        let config_file = ConfigFile::load_template_config(directory);

        Config {
            templates: folders,
            config: config_file,
        }
    }
}
