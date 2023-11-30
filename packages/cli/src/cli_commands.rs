pub struct CliCommands;
use std::{ffi::OsStr, fmt::Display};

use inquire::{Confirm, Editor, Select, Text};

use crate::case_util::CaseType;

impl CliCommands {
    pub fn input(text: &str) -> Result<String, String> {
        // initial text will be green
        let result = Text::new(&format!("{}:", text)).prompt();

        if result.is_err() {
            let text_formatted = format!("{} cannot be empty", text);
            return Err(text_formatted);
        }
        Ok(result.unwrap())
    }

    pub fn editor(text: &str) -> Result<String, String> {
        // initial text will be green
        // let editor_command = OsStr::new("vim");
        let result = Editor::new(&format!("{}:", text)).prompt();

        if result.is_err() {
            let text_formatted = format!("{} cannot be empty", text);
            return Err(text_formatted);
        }
        Ok(result.unwrap())
    }

    pub fn case_type(case_type: Option<CaseType>, text: &str) -> Result<CaseType, String> {
        let items = vec![
            CaseType::SnakeCase.to_str_name(),
            CaseType::KebabCase.to_str_name(),
            CaseType::CamelCase.to_str_name(),
            CaseType::PascalCase.to_str_name(),
        ];

        let default = if let Some(case_type) = case_type {
            match case_type {
                CaseType::SnakeCase => 0,
                CaseType::KebabCase => 1,
                CaseType::CamelCase => 2,
                CaseType::PascalCase => 3,
                _ => 0,
            }
        } else {
            0
        };
        let result = Select::new(&format!("{}:", text), items)
            .with_starting_cursor(default)
            .prompt();
        if !result.is_ok() {
            return Err("Case type cannot be empty".to_string());
        }
        let result = result.unwrap();
        let case_type = CaseType::from_str(result);
        Ok(case_type)
    }

    pub fn select<T: Clone + Display>(text: &str, items: &[T]) -> Result<T, ()> {
        let result = Select::new(&format!("{}:", text), items.to_vec()).prompt();
        if !result.is_ok() {
            return Err(());
        }
        let result = result.unwrap();
        Ok(result)
    }

    pub fn confirm(text: &str) -> bool {
        let result = Confirm::new(&format!("{} (y/n):", text)).prompt();
        if !result.is_ok() {
            return false;
        }
        let result = result.unwrap();
        result
    }
}
