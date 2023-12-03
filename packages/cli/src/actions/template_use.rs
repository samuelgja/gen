use crate::{
    cli_commands::CliCommands, config::Config, search_folder::SearchFolder,
    template::TemplateFolder, template_variable::TemplateVariableInfo,
};
use colored::Colorize;
use std::{collections::HashMap, fs};

pub struct TemplateUse;

impl TemplateUse {
    pub fn use_it(global_config: &Config, config: &Config, template_folder: &TemplateFolder) {
        let result = SearchFolder::search(&template_folder.path);
        println!();
        println!("Using template: {}", template_folder.name.green().bold());
        if result.template_config.description.len() > 0 {
            println!();
            println!("Description: {}", result.template_config.description);
        }
        println!();
        let mut values_for_keys = HashMap::new();

        for (key, variable) in result.variables {
            let var_name = &variable.raw_value;
            if variable.template_variable.is_var() {
                let text = format!("Add value for {}", &var_name.green().bold());
                let result = CliCommands::input_not_empty(&text, "Please provide some value", None);
                if result.is_err() {
                    std::process::exit(1);
                }
                let result = result.unwrap();
                values_for_keys.insert(key, result);
                println!();
            } else if variable.template_variable.is_select() {
                let text = format!("Select option for {}", &var_name.green().bold());
                if let Some(options_map) = &result.template_config.select_options {
                    if let Some(options) = options_map.get(&variable.raw_value) {
                        let result = CliCommands::select(&text, options);
                        if result.is_err() {
                            std::process::exit(1);
                        }
                        let result = result.unwrap();
                        values_for_keys.insert(key, result);
                        println!();
                    }
                }
            }
        }

        let cwd = std::env::current_dir().unwrap();
        for file in result.files.iter() {
            if file.is_config {
                continue;
            }
            let content = std::fs::read_to_string(&file.path).unwrap();
            let mut new_content = content.clone();
            for variable in TemplateVariableInfo::parse_iter(&content) {
                let key = variable.raw_value.clone();
                if let Some(value) = values_for_keys.get(&key) {
                    new_content = new_content.replace(&variable.raw_value, value);
                }
            }

            let mut new_path = Vec::new();
            println!(
                "what file: {}",
                file.template_path.to_str().unwrap().green()
            );
            for part in file.template_path.iter() {
                let part = part.to_str().unwrap();

                if let Some(variable) = TemplateVariableInfo::from_str(part) {
                    let key = variable.raw_value.clone();
                    if let Some(value) = values_for_keys.get(&key) {
                        new_path.push(value.to_owned());
                    } else {
                        new_path.push(part.to_string());
                    }
                } else {
                    new_path.push(part.to_string());
                }
            }
            let separator = if cfg!(windows) { "\\" } else { "/" };
            let path = cwd.join(new_path.join(separator));

            println!("Writing file: {}", path.to_str().unwrap().green());

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, new_content).unwrap();
        }
    }
}
