use crate::{case_util::CaseType, config::Config, constants::CONFIG_FILE};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateFolder {
    pub name: String,
    pub path: PathBuf,
}
impl TemplateFolder {
    pub fn new(config: &Config, name: &str) -> TemplateFolder {
        let template_path = config.path.join(name);
        let is_exist = template_path.exists();
        if !is_exist {
            fs::create_dir_all(&template_path).unwrap();
        }

        TemplateFolder {
            name: name.to_string(),
            path: template_path.clone(),
        }
    }

    pub fn new_empty(config: &Config, name: &str) -> TemplateFolder {
        let template_path = config.path.join(name);

        TemplateFolder {
            name: name.to_string(),
            path: template_path.clone(),
        }
    }
}

impl Display for TemplateFolder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TemplateFolder {
    pub fn create_file(&self, path: &PathBuf, content: &str) {
        let directory = path.parent().unwrap();
        if !directory.exists() {
            fs::create_dir_all(directory).unwrap();
        }

        fs::write(path, content).unwrap();
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
    pub case_type: Option<TemplateCaseType>,
    pub select_options: Option<HashMap<String, Vec<String>>>,
}

impl TemplateConfig {
    pub fn new() -> TemplateConfig {
        TemplateConfig {
            name: "".to_string(),
            description: "".to_string(),
            case_type: None,
            select_options: None,
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

    pub fn merge_select_options(&mut self, select_options: &HashMap<String, Vec<String>>) {
        if self.select_options.is_none() {
            self.select_options = Some(HashMap::new());
        }

        let current_select_options = self.select_options.as_mut().unwrap();
        for (key, value) in select_options.iter() {
            current_select_options.insert(key.to_string(), value.to_vec());
        }
    }
}
