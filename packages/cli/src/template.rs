use crate::{case_util::CaseType, config::ConfigFile, search_folder::SearchFolder};
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
pub struct TemplateFile {
    pub content: String,
    pub case_type: TemplateCaseType,
    pub is_append_mode: bool,
    pub path: Vec<String>,
    pub alternative_paths: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct TemplateFolder {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
    pub case_type: TemplateCaseType,
}

impl TemplateConfig {
    pub fn new() -> TemplateConfig {
        TemplateConfig {
            name: "".to_string(),
            description: "".to_string(),
            case_type: TemplateCaseType::new(),
        }
    }

    pub fn load_template_config(template_folder: &TemplateFolder) -> TemplateConfig {
        let config_path = template_folder.path.join("config.json");
        let config_content = fs::read_to_string(config_path);

        if config_content.is_err() {
            return TemplateConfig::new();
        }
        let config_content = config_content.unwrap();
        let config: TemplateConfig = serde_json::from_str(&config_content).unwrap();
        config
    }

    pub fn save_template_config(&self, template_folder: &TemplateFolder) {
        let config_path = template_folder.path.join("config.json");
        let config_content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(config_path, config_content).unwrap();
    }
}

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub files: Vec<TemplateFile>,
    pub is_global: bool,
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}: {}", "Name".cyan(), self.name));
        let description_max_100 = if self.description.len() > 100 {
            Cow::Owned(format!("{}...", &self.description[..100]))
        } else {
            Cow::Borrowed(&self.description)
        };
        result.push_str(&format!(
            "{}: {}",
            "Description".cyan(),
            description_max_100
        ));
        write!(f, "{} {}", self.name.green().bold(), self.description,)
    }
}

impl Template {
    pub fn new() -> Template {
        Template {
            name: "".to_string(),
            description: "".to_string(),
            files: Vec::new(),
            is_global: false,
        }
    }

    pub fn load_template(config_file: &ConfigFile, template_folder: &TemplateFolder) {
        let name = template_folder.name.to_owned();
        let template_config = TemplateConfig::load_template_config(template_folder);

        let search = SearchFolder::search(&template_folder.path);
        println!("");
        println!("search: {:?}", search);
    }
}
