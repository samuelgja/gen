use crate::{
    case_util::CaseType,
    commands::Commands,
    template::{
        Template, TemplateFile, TemplatePath, TEMPLATE_DOCS_URL, TEMPLATE_FILE_VARIABLE,
        TEMPLATE_FOLDER_OPTION, TEMPLATE_VARIABLE,
    },
};

use colored::Colorize;
use regex::Regex;

pub struct TemplateAction;

impl TemplateAction {
    pub fn new() -> Template {
        println!();
        println!("{}", "📂 Creating of a new template -> ".green());
        println!();
        let name = Commands::input("Enter the name of template").unwrap();
        println!();
        let description = Commands::input("Enter template description").unwrap();
        println!("{}", "📄 Now let's add some files to the template".green());
        println!();
        TemplateAction::print_content_file_info();
        println!(
            "{} {}",
            "If there is any issues with that, just visit:",
            TEMPLATE_DOCS_URL.yellow().underline().bold()
        );
        println!();

        let files = TemplateAction::new_template_files();

        Template {
            name,
            description,
            files,
        }
    }

    pub fn print_content_file_info() {
        println!();
        println!("{}", "Template file contains:");
        println!();
        println!(
            "{} {} {} {} {} {}",
            "1. Template content:".cyan(),
            "copy / paste anything & replace repeated actions with",
            TEMPLATE_VARIABLE.bold().underline().magenta(),
            "for content names or",
            TEMPLATE_FILE_VARIABLE.bold().underline().magenta(),
            "for file names.",
        );

        println!(
            "{} {} {} {} {}{}",
            "2. Template Path:".cyan(),
            "replace repeated actions with",
            TEMPLATE_FILE_VARIABLE.bold().underline().magenta(),
            "or for folder options",
            TEMPLATE_FOLDER_OPTION.bold().underline().magenta(),
            ". 🤭 Do not forgot about file extension.",
        );
        println!();
    }

    pub fn new_template_files() -> Vec<TemplateFile> {
        let mut template_files = vec![];
        let mut previous_file_case_type = None;
        let mut previous_content_case_type = None;

        loop {
            let template_file =
                TemplateAction::new_file(previous_content_case_type, previous_file_case_type);

            previous_content_case_type = Some(template_file.case_type.clone());
            previous_file_case_type =
                Some(template_file.paths.last().unwrap().path_case_type.clone());

            print!("{} ", "✅ Adding template file done.".green());
            println!();
            println!();
            let is_not_done = Commands::confirm("Do you want to add more template files?");
            println!();

            template_files.push(template_file);
            if !is_not_done {
                break;
            }
        }
        template_files
    }

    pub fn new_file(
        previous_case_type: Option<CaseType>,
        previous_file_case_type: Option<CaseType>,
    ) -> TemplateFile {
        let path_separator_regex = Regex::new(r"\\|/").unwrap();
        let template_file_path_regex = Regex::new(r"\$_FILE_NAME(\d*)").unwrap();
        println!();

        let is_append_mode =
            Commands::select("📝 Select mode for template file", &["Replace", "Append"])
                .unwrap_or("")
                == "Append";

        // Select::with_theme(&ColorfulTheme::default())
        //     .with_prompt("📝 Select mode for template file")
        //     .default(0)
        //     .items(&["Replace", "Append"])
        //     .interact()
        //     .unwrap_or(0)
        //     == 1;
        println!();
        let content = Commands::editor("📝 Add template content").unwrap();
        println!();
        let content_case_type =
            Commands::case_type(previous_case_type, "💼 Add case type for template content")
                .unwrap();
        println!();
        let mut paths = vec![];
        let mut path = String::new();

        loop {
            loop {
                path =
                    Commands::input("🐍 Add project relative path to the template file").unwrap();
                println!();
                if template_file_path_regex.is_match(&path) {
                    break;
                } else {
                    println!(
                        "{} {}",
                        "🚨 Path must contain".red(),
                        TEMPLATE_FILE_VARIABLE.bold().underline().magenta()
                    );
                    println!();
                }
            }

            let file_case_type = Commands::case_type(
                previous_file_case_type.clone(),
                "Also select case type for file names which can be different as content case type",
            )
            .unwrap();

            let path_parts: Vec<String> = path
                .split(path_separator_regex.as_str())
                .map(|part| part.to_string())
                .collect();
            paths.push(TemplatePath {
                path_parts,
                path_case_type: file_case_type.clone(),
            });

            let is_not_done = Commands::confirm("Do you want to add more relative paths?");

            if !is_not_done {
                break;
            }
        }

        TemplateFile {
            content,
            paths,
            case_type: content_case_type,
            is_append_mode,
        }
    }
}
