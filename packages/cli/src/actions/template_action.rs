use crate::{
    case_util::CaseType,
    cli_commands::CliCommands,
    config::{Config, ConfigFile},
    constants::{
        CONFIG_FILE, TEMPLATE_DOCS_URL, TEMPLATE_ROOT_FOLDER, TEMPLATE_SELECT, TEMPLATE_VARIABLE,
    },
    search_folder::SearchFolder,
    template::{TemplateConfig, TemplateFolder},
    template_file_content::TEMPLATE_FILE_CONTENT,
    template_variable::TemplateVariable,
};
use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct TemplateAction;

impl TemplateAction {
    pub fn new_template(config: &Config) {
        println!();
        println!("{}", "📂 Creating of a new template -> ".green());
        println!();
        let name = CliCommands::input_not_empty(
            "Enter template name",
            "Template name cannot be empty",
            None,
        );
        if name.is_err() {
            return;
        }
        let name = name.unwrap();
        println!();

        let description = CliCommands::input("Enter template description", None);
        if description.is_err() {
            return;
        }
        let description = description.unwrap();

        let template_folder = TemplateFolder::new(config, &name);
        let mut template_config = TemplateConfig::load_template_config(&template_folder);
        template_config.name = name;
        template_config.description = description;
        template_config.save_template_config(&template_folder);

        TemplateAction::template_edit(config, &template_folder, &mut template_config);
    }

    pub fn template_update_select_options(
        template_folder: &TemplateFolder,
        template_config: &mut TemplateConfig,
        ignore_existing: bool,
    ) {
        let result = SearchFolder::search(&template_folder.path);

        let select_variables = result
            .variables
            .iter()
            .filter(|(_, variable)| variable.template_variable == TemplateVariable::Select)
            .collect::<Vec<_>>();

        if !select_variables.is_empty() {
            println!();
            println!(
                "{}",
                "📝 Add some option values for __select__ variables".yellow(),
            );
            println!();
            println!(
                "{}",
                "Note: Enter select options values separated byt comma (basic, complex, other, etc...)".italic()
            );
            println!();
            let mut select_options = HashMap::new();
            let mut index = 0;
            let width = 15;
            for (_, variable) in select_variables.iter() {
                index += 1;
                if variable.is_auto {
                    continue;
                }

                let already_set_value =
                    if let Some(select_options) = &template_config.select_options {
                        select_options.get(&variable.raw_value)
                    } else {
                        None
                    };

                if already_set_value.is_some() {
                    let mut want_to_change = false;
                    if !ignore_existing {
                        want_to_change = CliCommands::confirm(&format!(
                            "📔 Do you want to update {} options? Current values: {}",
                            &variable.raw_value.cyan().bold().italic(),
                            already_set_value.unwrap().join(", ").bold().magenta()
                        ));
                    }

                    if !want_to_change {
                        println!(
                            "{} {:width$} with option values: {}",
                            "Skipping".magenta(),
                            variable.raw_value.bold().yellow(),
                            already_set_value.unwrap().join(", ").bold().cyan()
                        );

                        select_options.insert(
                            variable.raw_value.to_owned(),
                            already_set_value.unwrap().to_owned(),
                        );
                        continue;
                    }
                }
                let result = CliCommands::input_not_empty(
                    &format!(
                        "📔 {}/{} Enter options values for {}",
                        index,
                        select_variables.len(),
                        &variable.raw_value.cyan().bold().italic()
                    ),
                    "Select values cannot be empty",
                    None,
                );

                if result.is_err() {
                    continue;
                }

                let result_vec = result
                    .unwrap()
                    .split(',')
                    .map(|item| item.trim().to_owned())
                    .collect::<Vec<_>>();
                let result_set = result_vec.iter().collect::<HashSet<_>>();
                let result_vec = result_set
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>();

                println!(
                    "{} {:width$} with option values: {}",
                    "Updated".magenta(),
                    variable.raw_value.bold().yellow(),
                    result_vec.join(", ").bold().cyan()
                );

                select_options.insert(variable.raw_value.to_owned(), result_vec);
            }

            template_config.merge_select_options(&select_options);
            template_config.save_template_config(template_folder);
        }
    }
    pub fn template_edit(
        config: &Config,
        template_folder: &TemplateFolder,
        template_config: &mut TemplateConfig,
    ) {
        TemplateAction::print_content_file_info();
        TemplateAction::template_file_info();
        TemplateAction::new_template_files(config, template_folder);
        TemplateAction::template_update_select_options(template_folder, template_config, false);

        println!();
        println!(
            "{} {}",
            "Done. Template created at:".green(),
            template_folder.path.to_str().unwrap().bold().green()
        );
        println!();
    }

    pub fn print_file_steps() {
        let width = 48;
        println!();
        println!("{}", "Knowledge base:".bold());
        println!();
        let first_step = format!(
            "{} {}",
            "1. 🐍 Template file".magenta(),
            "path:".bold().magenta()
        );
        let second_step = format!(
            "{} {}",
            "2. 📄 Template file".magenta(),
            "content:".bold().magenta()
        );

        let third_step = format!(
            "{} {}",
            "3. 🧙 Template".magenta(),
            "variables:".bold().magenta(),
        );

        println!(
            "{:width$} relative path from project root directory, also with extensions",
            first_step,
        );
        println!("{:width$} can be in any format", second_step);

        println!(
            "{:width$} available variables for both.. template paths & template content are ->",
            third_step,
        );
        println!();
        println!(
            "{} for text inputs and {} for dropdowns selections.",
            TEMPLATE_VARIABLE.bold().magenta(),
            TEMPLATE_SELECT.bold().magenta()
        );
        println!();
        println!(
            "To move templates to another project, simply just copy {} folder",
            TEMPLATE_ROOT_FOLDER.bold().green()
        );
    }
    pub fn print_content_file_info() {
        println!();
        println!("{}", "📁 Now let's add some files to the template".green());
        println!();
        println!("Template file contains:");
        println!();

        TemplateAction::print_file_steps();
        println!();
        println!(
            "If there is any issues, just visit: {}",
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
        println!("{}", "📄 Add template file -> ".green());
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
    pub fn new_template_files(config: &Config, template_folder: &TemplateFolder) {
        loop {
            TemplateAction::new_template_file(config, template_folder);
            let is_done = CliCommands::confirm("Do you want to add new template file?");
            if !is_done {
                break;
            }
        }
    }
    pub fn new_template_file(config: &Config, template_folder: &TemplateFolder) {
        let path = CliCommands::input_path(&template_folder.path, "Enter template file path", None);

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

        template_folder.create_file(&path, TEMPLATE_FILE_CONTENT);

        if let Some(command) = &config.config.open_editor_command {
            // run command
            let full_command = format!("{} {}", command, path.to_str().unwrap());
            let is_ok = CliCommands::run_terminal_command(&full_command);
            if !is_ok {
                println!();
                println!(
                    "{} {}",
                    "🚨 Cannot open editor with command:".red(),
                    full_command.bold().red()
                );
                println!();
            }
        }
        println!();
        println!("{}", "For continue, open new created file in your favorite editor. Then edit, save & that's it!".bright_white());
        println!();
        println!(
            "✅ Template file created at: {}",
            path.to_str().unwrap().bold().green()
        );
        println!();
    }

    pub fn get_template_config() -> ConfigFile {
        println!();
        println!(
            "{}",
            "🕹️  Before start please select preferred case types & custom code to open editor:"
                .green()
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

        config_file
    }

    pub fn get_template_command_args() -> Option<String> {
        println!();
        let is_adding_terminal_command = CliCommands::confirm(
            "Do you want to add terminal command to open template file in editor?",
        );
        if is_adding_terminal_command {
            println!();
            let open_editor_command = CliCommands::input(
            "Enter any terminal command to open file in editor (example: code, atom, subl, vim, etc...)",None);

            if let Ok(open_editor_command) = open_editor_command {
                if !open_editor_command.is_empty() {
                    return Some(open_editor_command);
                }
            }
        }
        None
    }

    pub fn refresh_templates(config: &Config) {
        let template_folders = &config.template_folders;

        let mut index = 0;

        for template_folder in template_folders {
            index += 1;
            println!(
                "🔄 {}/{} {} {}",
                index,
                template_folders.len(),
                "Refreshing template:".green(),
                template_folder.name.bold().green()
            );
            let mut template_config = TemplateConfig::load_template_config(template_folder);
            TemplateAction::template_update_select_options(
                template_folder,
                &mut template_config,
                true,
            );
        }

        println!();
        println!("{}", "✅ All templates refreshed.".green());
        println!();
    }

    pub fn list_of_all_variables(config: &Config) {
        let template_folders = &config.template_folders;

        let mut index = 0;
        println!();
        println!("{}", "📚 List of all variables:".bold().green());
        println!();
        for template_folder in template_folders {
            index += 1;
            println!(
                "📝 {}/{} {} {}",
                index,
                template_folders.len(),
                "Template:".green(),
                template_folder.name.bold().green()
            );
            let result = SearchFolder::search(&template_folder.path);
            let variables = result.variables;
            if variables.is_empty() {
                println!(
                    "{} {}",
                    "🚨 No variables found.".red(),
                    "Check template file content or paths.".italic()
                );
                println!();
                continue;
            }
            for (variable, variable_info) in variables {
                println!(
                    "{} {} {} {}",
                    "📔".magenta(),
                    variable.bold().yellow(),
                    "type:".bold().magenta(),
                    variable_info.template_variable.to_string().bold().cyan()
                );
            }
            println!();
        }
    }
}
