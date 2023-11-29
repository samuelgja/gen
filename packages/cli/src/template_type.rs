use crate::case_util::CaseType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatePath<'a> {
    #[serde(borrow)]
    pub parts: Vec<Cow<'a, str>>,
    pub case_type: CaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateFile<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub content: Cow<'a, str>,
    pub case_type: CaseType,
    #[serde(borrow)]
    pub path: TemplatePath<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub description: Cow<'a, str>,
    #[serde(borrow)]
    pub files: Vec<TemplateFile<'a>>,
}

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template {
            name: Cow::Borrowed(""),
            description: Cow::Borrowed(""),
            files: Vec::new(),
        }
    }

    pub fn load_from_json(json: &str) -> Result<Template, serde_json::Error> {
        let templates: Template = serde_json::from_str(json)?;
        Ok(templates)
    }

    pub fn to_json(&self) -> Result<Cow<'a, str>, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(Cow::Owned(json))
    }
}
