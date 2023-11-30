use crate::{
    actions::TemplateAction,
    constants::CONFIG_FILE,
    template::{TemplateCaseType, TemplateFolder},
};

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    pub templates: Vec<TemplateFolder>,
    pub config: ConfigFile,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct ConfigFile {
    pub case_type: TemplateCaseType,
    pub open_editor_command: Option<String>,
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile {
            case_type: TemplateCaseType::new(),
            open_editor_command: None,
        }
    }

    pub fn load_config(directory: &Path, is_template_enabled: bool) -> ConfigFile {
        let config_path = directory.join(CONFIG_FILE);
        let config_content = fs::read_to_string(&config_path);

        if config_content.is_err() {
            if !is_template_enabled {
                return ConfigFile::new();
            }
            let config = TemplateAction::get_template_config();
            config.save_config(directory);
            return config;
        }
        let config_content = config_content.unwrap();
        let config: Result<ConfigFile, serde_json::Error> = serde_json::from_str(&config_content);

        if config.is_err() {
            let config = TemplateAction::get_template_config();
            fs::remove_file(&config_path).unwrap();
            config.save_config(directory);
            return config;
        }

        config.unwrap()
    }

    pub fn save_config(&self, directory: &Path) {
        let config_path = directory.join(CONFIG_FILE);
        let config_content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(config_path, config_content).unwrap();
    }

    pub fn merge(&mut self, config: &ConfigFile) {
        if self.open_editor_command.is_none() {
            self.open_editor_command = config.open_editor_command.clone();
        }
    }
}

impl Config {
    pub fn load_template_folders(directory: &PathBuf) -> Config {
        if !directory.exists() {
            fs::create_dir_all(directory).unwrap();
        }
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

        Config {
            templates: folders,
            config: ConfigFile::new(),
            path: directory.to_path_buf(),
        }
    }
}
