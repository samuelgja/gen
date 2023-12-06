use crate::{
    cli_commands::CliCommands, config::Config, search_folder::SearchFolder,
    template::TemplateFolder, template_variable::TemplateVariableInfo,
};
use colored::Colorize;
use std::{collections::HashMap, fs, io::Write};

pub struct TemplateUse;

impl TemplateUse {
    fn get_case_value(
        is_file_path: bool,
        config: &Config,
        variable: &TemplateVariableInfo,
        value: &str,
    ) -> String {
        if variable.case_type.is_not_unknown() {
            println!(
                "Using case type: {} for value: {}",
                variable.case_type.to_str_name().magenta().bold(),
                value.cyan().bold()
            );
            return variable.case_type.from_string_to_case(value);
        };

        let case_type_from_config = if is_file_path {
            &config.config.case_type.file
        } else {
            &config.config.case_type.content
        };

        if case_type_from_config.is_not_unknown() {
            return case_type_from_config.from_string_to_case(value);
        }

        value.to_string()
    }

    pub fn use_it(global_config: &Config, _config: &Config, template_folder: &TemplateFolder) {
        let result = SearchFolder::search(&template_folder.path);
        println!();
        println!("Using template: {}", template_folder.name.green().bold());
        if !result.template_config.description.is_empty() {
            println!();
            println!("Description: {}", result.template_config.description);
        }
        println!();
        let mut values_for_keys = HashMap::new();

        for (key, variable) in result.variables {
            let var_name = SearchFolder::get_key(&variable);
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

            let mut replace_vec = Vec::new();
            for variable in TemplateVariableInfo::parse_iter(&content) {
                let key = SearchFolder::get_key(&variable);
                if let Some(value) = values_for_keys.get(&key) {
                    let case_value =
                        TemplateUse::get_case_value(false, global_config, &variable, value);

                    replace_vec.push((variable.raw_value.clone(), case_value));
                }
            }

            // need to sort by length to avoid replacing the wrong values
            // it start with the longest value
            replace_vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            for (old, new) in replace_vec {
                new_content = new_content.replace(&old, &new);
            }

            let mut new_path = Vec::new();
            let mut is_append_mode = false;

            let length = file.template_path.iter().count();
            for (index, part) in file.template_path.iter().enumerate() {
                let part = part.to_str().unwrap();
                let is_last_part = index == length - 1;
                if let Some(variable) = TemplateVariableInfo::from_str(part) {
                    let key = SearchFolder::get_key(&variable);
                    if let Some(value) = values_for_keys.get(&key) {
                        let case_value =
                            TemplateUse::get_case_value(true, global_config, &variable, value);

                        let new_part = part.replace(&variable.raw_value, &case_value);

                        new_path.push(new_part.to_owned());
                    } else {
                        if is_last_part {
                            is_append_mode = true;
                        }
                        new_path.push(part.to_string());
                    }
                } else {
                    if is_last_part {
                        is_append_mode = true;
                    }
                    new_path.push(part.to_string());
                }
            }
            let separator = if cfg!(windows) { "\\" } else { "/" };
            let path = cwd.join(new_path.join(separator));

            println!("Writing file: {}", path.to_str().unwrap().green());

            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).unwrap();
                }
            }

            if is_append_mode {
                if !path.exists() {
                    fs::write(&path, new_content).unwrap();
                } else {
                    let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
                    file.write_all(new_content.as_bytes()).unwrap();
                }

                continue;
            } else if path.exists() {
                let can_overwrite = CliCommands::confirm(&format!(
                    "File {} already exists. Do you want to overwrite?",
                    path.to_str().unwrap()
                ));
                if can_overwrite {
                    fs::write(&path, new_content).unwrap();
                }
            } else {
                fs::write(&path, new_content).unwrap();
            }
        }

        println!();
        println!("{}", "Done!".green());
    }
}
