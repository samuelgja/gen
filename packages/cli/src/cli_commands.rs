pub struct CliCommands;
use crate::case_util::CaseType;
use colored::Colorize;
use inquire::{Confirm, MultiSelect, Select, Text};
use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

impl CliCommands {
    pub fn input(text: &str, default: Option<&str>) -> Result<String, String> {
        // initial text will be green
        let result = Text::new(&format!("{}:", text)).prompt();

        if result.is_err() {
            std::process::exit(1);
        }
        Ok(result.unwrap())
    }

    pub fn input_not_empty(
        text: &str,
        error_msg: &str,
        default: Option<&str>,
    ) -> Result<String, String> {
        loop {
            let result = Text::new(&format!("{}:", text)).prompt();

            if result.is_err() {
                std::process::exit(1);
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

    pub fn input_path(
        template_path: &PathBuf,
        text: &str,
        default: Option<&str>,
    ) -> Result<PathBuf, String> {
        let mut loops_count = 0;
        loop {
            loops_count += 1;

            if loops_count % 5 == 0 {
                println!();
                let is_exit =
                    Confirm::new(&format!("Do you wish to exit?{} (y/n):", text)).prompt();
                if is_exit.is_err() {
                    std::process::exit(1);
                }
                let is_exit = is_exit.unwrap();
                if is_exit {
                    return Err("".to_string());
                }
            }
            let result = CliCommands::input_not_empty(text, "Path cannot be empty", default);
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

    pub fn run_terminal_command(command: &str) -> bool {
        let args = command.split(' ').collect::<Vec<_>>();

        let first_take = args[0].to_owned();
        let args = args[1..].to_owned();
        // let loading = Loading::default();
        // loading.text("Opening file".blue());
        let result = std::process::Command::new(first_take).args(args).output();

        if result.is_err() {
            println!("result: {:?}", result.err().unwrap());
            // loading.end();
            return false;
        }

        let result = result.unwrap();

        if !result.status.success() {
            println!("status: {:?}", result.status);
            println!("stdout: {:?}", String::from_utf8(result.stdout));
            println!("stderr: {:?}", String::from_utf8(result.stderr));

            return false;
        }
        // loading.success("OK");
        // loading.end();
        true
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

    pub fn multi_select<T: Clone + Display>(text: &str, items: &[T]) -> Result<Vec<T>, ()> {
        let result = MultiSelect::new(&format!("{}:", text), items.to_vec()).prompt();
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
