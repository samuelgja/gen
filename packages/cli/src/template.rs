use crate::case_util::CaseType;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

pub const TEMPLATE_VARIABLE: &str = "$_NAME";
pub const TEMPLATE_FILE_VARIABLE: &str = "$_FILE_NAME";
pub const TEMPLATE_FOLDER_OPTION: &str = "$_FOLDER_OPTION";
pub const TEMPLATE_DOCS_URL: &str = "https://something.com";
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplatePath {
    pub path_parts: Vec<String>,
    pub path_case_type: CaseType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateFile {
    pub content: String,
    pub case_type: CaseType,
    pub is_append_mode: bool,
    pub paths: Vec<TemplatePath>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub files: Vec<TemplateFile>,
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
        }
    }

    pub fn load_from_json(json: &str) -> Result<Template, serde_json::Error> {
        let templates: Template = serde_json::from_str(json)?;
        Ok(templates)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}
