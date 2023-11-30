use std::{fmt::format, fs, path::PathBuf};

use crate::{
    case_util::CaseType,
    cli_commands::CliCommands,
    config::{Config, ConfigFile},
    constants::{
        CONFIG_FILE, TEMPLATE_DOCS_URL, TEMPLATE_ROOT_FOLDER, TEMPLATE_SELECT, TEMPLATE_VARIABLE,
    },
    template::{TemplateConfig, TemplateFolder},
    template_file_content::TEMPLATE_FILE_CONTENT,
};

use colored::Colorize;
use regex::Regex;

pub struct TemplateAction;

impl TemplateAction {
    pub fn new(config: &Config) {
        println!();
        println!("{}", "📂 Creating of a new template -> ".green());
        println!();
        let name =
            CliCommands::input_not_empty("Enter template name", "Template name cannot be empty");
        if name.is_err() {
            return;
        }
        let name = name.unwrap();
        println!();

        let description = CliCommands::input("Enter template description");
        if description.is_err() {
            return;
        }
        let description = description.unwrap();

        TemplateAction::print_content_file_info();

        let template_path = &config.path.join(&name);
        let is_exist = template_path.exists();
        if !is_exist {
            fs::create_dir_all(template_path).unwrap();
        }

        let template_folder = TemplateFolder {
            name: name.clone(),
            path: template_path.to_path_buf(),
        };
        let mut template_config = TemplateConfig::load_template_config(&template_folder);
        template_config.name = name;
        template_config.description = description;
        template_config.save_template_config(&template_folder);

        TemplateAction::template_file_info();
        TemplateAction::new_template_files(&template_folder);

        println!();
        println!(
            "{} {}",
            "Done. Template created at:".green(),
            template_path.to_str().unwrap().bold().green()
        );
        println!();
    }

    pub fn print_content_file_info() {
        println!();
        println!("{}", "📁 Now let's add some files to the template".green());
        println!();
        println!("{}", "Template file contains:");
        println!();

        println!(
            "{} {} {}",
            "1. 🐍 Template file".magenta(),
            "path:".bold().magenta(),
            "relative path from project root directory, also with extensions"
        );
        println!(
            "{} {} {}",
            "2. 📄 Template file".magenta(),
            "content:".bold().magenta(),
            "can be in any format"
        );

        println!(
            "{} {} {}",
            "3. 🧙 Template".magenta(),
            "variables:".bold().magenta(),
            "available variables for both template file & content are ->",
        );
        println!();
        println!(
            "{} {} {} {}",
            TEMPLATE_VARIABLE.bold().magenta(),
            "for text inputs and",
            TEMPLATE_SELECT.bold().magenta(),
            "for dropdowns selections."
        );

        println!();
        println!(
            "{} {}",
            "If there is any issues, just visit:",
            TEMPLATE_DOCS_URL.yellow().underline().bold()
        );
    }

    pub fn template_path_example() -> String {
        format!(
            "src/utils/{}/{}/{}{}",
            TEMPLATE_SELECT.bold(),
            TEMPLATE_VARIABLE.bold(),
            TEMPLATE_VARIABLE.bold(),
            ".style.ts"
        )
    }
    pub fn template_file_info() {
        println!();
        println!("{}", "📄 Creating of a new template file -> ".green());
        println!();
        println!(
            "{} {}",
            "Example of path:".italic(),
            TemplateAction::template_path_example().cyan().italic()
        );
        println!();
        println!(
            "{}",
            "If file_name do not contain any variable, it will be in append mode. Useful for index files, mod files, headers, etc..".italic()
        );
        println!();
    }
    pub fn new_template_files(template_folder: &TemplateFolder) {
        loop {
            TemplateAction::new_template_file(template_folder);
            let is_done = CliCommands::confirm("Do you want to add new template file?");
            if !is_done {
                break;
            }
        }
    }
    pub fn new_template_file(template_folder: &TemplateFolder) {
        let path = CliCommands::input_path(&template_folder.path, "Enter template file path");

        if path.is_err() {
            return;
        }

        let path = path.unwrap();

        if path.exists() && path.is_file() {
            println!();
            println!(
                "{} {}",
                "🚨 File already exist at:".red(),
                path.to_str().unwrap().bold().red()
            );
            println!();
            let is_continue = CliCommands::confirm("Do you want to overwrite it?");
            if !is_continue {
                return;
            }

            fs::remove_file(&path).unwrap();
        }
        if path.exists() && path.is_dir() {
            println!();
            println!(
                "{} {}",
                "🚨 Path already exist and it's at:".red(),
                path.to_str().unwrap().bold().red()
            );
            return;
        }

        template_folder.create_file(&path, &TEMPLATE_FILE_CONTENT);
        println!();
        println!("{}", "For continue, open new created file in your favorite editor. Then edit, save & that's it!".bright_white());
        println!();
        println!(
            "{} {}",
            "✅ Template file created at:",
            path.to_str().unwrap().bold().green()
        );
        println!();
    }

    pub fn get_template_config() -> ConfigFile {
        println!();
        println!(
            "{}",
            "🕹️  Before start please select preferred case types:".green()
        );
        println!();
        println!(
            "{} {}",
            "Note: Case type can be changed it any point. Just edit:".italic(),
            format!("{}/{}", TEMPLATE_ROOT_FOLDER, CONFIG_FILE)
                .bold()
                .magenta()
                .italic()
        );
        println!(
            "{}",
            "Also each template variable can have separate case type.".italic()
        );
        println!();
        let file_case_type =
            CliCommands::case_type(Some(CaseType::KebabCase), "Case type for file names");
        let mut config_file = ConfigFile::new();
        if file_case_type.is_ok() {
            config_file.case_type.file = file_case_type.unwrap();
        }

        println!();
        let content_case_type =
            CliCommands::case_type(Some(CaseType::PascalCase), "Case type for template content");
        if content_case_type.is_ok() {
            config_file.case_type.content = content_case_type.unwrap();
        }

        return config_file;
    }
}
