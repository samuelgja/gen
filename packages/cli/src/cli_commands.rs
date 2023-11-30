pub struct CliCommands;
use colored::Colorize;
use inquire::{Confirm, Select, Text};
use std::{
    fmt::Display,
    fs,
    io::Error,
    path::{Path, PathBuf},
    process::Output,
};

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

    pub fn input_not_empty(text: &str, error_msg: &str) -> Result<String, String> {
        loop {
            let result = Text::new(&format!("{}:", text)).prompt();

            if result.is_err() {
                let text_formatted = format!("{} cannot be empty", text);
                return Err(text_formatted);
            }

            let result = result.unwrap();
            if !result.is_empty() {
                return Ok(result);
            }
            println!();
            println!("{}", format!("🚨 {}", error_msg).red());
            println!();
        }
    }

    pub fn input_path(template_path: &PathBuf, text: &str) -> Result<PathBuf, String> {
        let mut loops_count = 0;
        loop {
            loops_count += 1;

            if loops_count % 5 == 0 {
                println!();
                let is_exit =
                    Confirm::new(&format!("Do you wish to exit?{} (y/n):", text)).prompt();
                if is_exit.is_err() {
                    return Err("".to_string());
                }
                let is_exit = is_exit.unwrap();
                if is_exit {
                    return Err("".to_string());
                }
            }
            let result = CliCommands::input_not_empty(text, "Path cannot be empty");
            if result.is_err() {
                println!();
                println!("{}", "🚨 Invalid path".red());
                continue;
            }
            let result = result.unwrap();
            // result string contain back path (..)
            // so we need to check if it is valid path
            if result.contains("..") {
                println!();
                println!("{}", "🚨 Path cannot contain '..'".red());
                continue;
            }

            let path = Path::new(&result);
            let full_path = template_path.join(path);

            let mut wrapped_parent = full_path.to_owned();
            while let Some(parent) = wrapped_parent.to_owned().parent() {
                if parent == template_path {
                    break;
                }
                if parent.is_file() {
                    println!();
                    println!(
                        "{}",
                        format!(
                            "🚨 Parent path {} is file and not directory.",
                            parent.to_str().unwrap()
                        )
                        .red()
                    );
                    let can_over_write = CliCommands::confirm("Do you want to overwrite it?");
                    if !can_over_write {
                        return Err("".to_string());
                    }

                    fs::remove_file(parent).unwrap();
                    break;
                }

                if !parent.exists() {
                    break;
                }
                wrapped_parent = parent.to_path_buf();
            }

            if full_path.exists() {
                println!();
                println!("{}", format!("🚨 Path {} already exists", result).red());
                let can_over_write = CliCommands::confirm("Do you want to overwrite it?");
                if can_over_write {
                    if path.is_dir() {
                        fs::remove_dir_all(&full_path).unwrap();
                        return Ok(full_path);
                    }
                    fs::remove_file(&full_path).unwrap();
                    return Ok(full_path);
                }
            }

            return Ok(full_path);
        }
    }

    pub fn run_terminal_command(command: &str) -> Result<Output, Error> {
        let args = command.split(' ').collect::<Vec<_>>();
        let result = std::process::Command::new("sh").args(args).output();

        result
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
        if result.is_err() {
            return Err("Case type cannot be empty".to_string());
        }
        let result = result.unwrap();
        let case_type = CaseType::from_str(result);
        Ok(case_type)
    }

    pub fn select<T: Clone + Display>(text: &str, items: &[T]) -> Result<T, ()> {
        let result = Select::new(&format!("{}:", text), items.to_vec()).prompt();
        if result.is_err() {
            return Err(());
        }
        let result = result.unwrap();
        Ok(result)
    }

    pub fn confirm(text: &str) -> bool {
        let result = Confirm::new(&format!("{} (y/n):", text)).prompt();
        if result.is_err() {
            return false;
        }
        
        result.unwrap()
    }
}
