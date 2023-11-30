use crate::{
    actions::TemplateAction,
    case_util::CaseType,
    cli_commands::CliCommands,
    constants::CONFIG_FILE,
    template::{TemplateCaseType, TemplateFolder},
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
    pub path: PathBuf,
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
        let config_path = directory.join(CONFIG_FILE);
        let config_content = fs::read_to_string(config_path);

        if config_content.is_err() {
            let config = TemplateAction::get_template_config();
            config.save_template_config(directory);
            return config;
        }
        let config_content = config_content.unwrap();
        let config: ConfigFile = serde_json::from_str(&config_content).unwrap();
        config
    }

    pub fn save_template_config(&self, directory: &PathBuf) {
        let config_path = directory.join(CONFIG_FILE);
        let config_content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(config_path, config_content).unwrap();
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

        let config_file = ConfigFile::load_template_config(directory);

        Config {
            templates: folders,
            config: config_file,
            path: directory.to_path_buf(),
        }
    }
}
