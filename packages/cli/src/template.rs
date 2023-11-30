use crate::{
    case_util::CaseType, config::ConfigFile, constants::CONFIG_FILE, search_folder::SearchFolder,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateCaseType {
    pub content: CaseType,
    pub file: CaseType,
}
impl TemplateCaseType {
    pub fn new() -> TemplateCaseType {
        TemplateCaseType {
            content: CaseType::PascalCase,
            file: CaseType::KebabCase,
        }
    }
}

#[derive(Debug)]
pub struct TemplateFolder {
    pub name: String,
    pub path: PathBuf,
}

impl TemplateFolder {
    pub fn create_file(&self, path: &PathBuf) {
        let directory = path.parent().unwrap();
        if !directory.exists() {
            fs::create_dir_all(directory).unwrap();
        }
        fs::write(path, "").unwrap();
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
    pub case_type: Option<TemplateCaseType>,
}

impl TemplateConfig {
    pub fn new() -> TemplateConfig {
        TemplateConfig {
            name: "".to_string(),
            description: "".to_string(),
            case_type: None,
        }
    }

    pub fn load_template_config(template_folder: &TemplateFolder) -> TemplateConfig {
        let config_path = template_folder.path.join(CONFIG_FILE);
        let config_content = fs::read_to_string(config_path);

        if config_content.is_err() {
            return TemplateConfig::new();
        }
        let config_content = config_content.unwrap();
        let config: TemplateConfig = serde_json::from_str(&config_content).unwrap();
        config
    }

    pub fn save_template_config(&self, template_folder: &TemplateFolder) {
        let config_path = template_folder.path.join(CONFIG_FILE);
        let config_content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(config_path, config_content).unwrap();
    }
}
