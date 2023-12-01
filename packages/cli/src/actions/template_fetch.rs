use crate::{
    cli_commands::CliCommands, config::Config, constants::TEMPLATE_ROOT_FOLDER,
    template::TemplateFolder,
};
use colored::Colorize;
use fs_extra::dir::{move_dir, CopyOptions};
use std::{collections::HashMap, env::temp_dir, fs, path::Path};
pub struct TemplateFetch;

impl TemplateFetch {
    pub fn fetch_github(config: &Config, github_url: &str) {
        // partially search for github url and scan data
        let github_url = github_url.trim();

        let github_tmp_path = temp_dir().join("github_tmp");
        println!("github_tmp_path: {:?}", github_tmp_path);

        if github_tmp_path.exists() {
            fs::remove_dir_all(&github_tmp_path).unwrap();
        }

        fs::create_dir_all(&github_tmp_path).unwrap();

        let is_ok = CliCommands::run_terminal_command(&format!(
            "git clone {} {}",
            github_url,
            github_tmp_path.to_str().unwrap()
        ));
        if !is_ok {
            println!("{}", "🚨 Error while cloning repository".red());
            return;
        }

        let repo_path = Path::new(&github_tmp_path);
        let template_folder_path = repo_path.join(TEMPLATE_ROOT_FOLDER);

        if !template_folder_path.exists() {
            println!("{}", "🚨 Repository does not contain template folder".red());
            return;
        }

        let git_config = Config::load_template_folders(&template_folder_path);

        let git_template_folders = CliCommands::multi_select(
            "Select templates to be copied:",
            &git_config.template_folders,
        );

        if git_template_folders.is_err() {
            println!("{}", "🚨 No templates selected".red());
            return;
        }

        let git_template_folders = git_template_folders.unwrap();

        if git_template_folders.is_empty() {
            println!("{}", "🚨 No templates selected".red());
            return;
        }

        let config_template_folders_hash_map: HashMap<String, TemplateFolder> = config
            .template_folders
            .iter()
            .map(|item| (item.name.clone(), item.clone()))
            .collect();

        for git_template_folder in git_template_folders.iter() {
            if let Some(item) = config_template_folders_hash_map.get(&git_template_folder.name) {
                println!();
                println!(
                    "{} {}",
                    "🚨 Template folder already exists.".red(),
                    item.name
                );
                println!();
                let can_delete = CliCommands::confirm("Do you want to delete it before fetch new?");
                if can_delete {
                    fs::remove_dir_all(&item.path).unwrap();
                } else {
                    continue;
                }
            }

            let destination = config.path.join(&git_template_folder.name);

            if !destination.exists() {
                fs::create_dir_all(&destination).unwrap();
            }

            let move_result = move_dir(
                &git_template_folder.path,
                &config.path.join(&git_template_folder.name),
                &CopyOptions::new(),
            );
            if move_result.is_err() {
                println!("{}", "🚨 Error while moving template folder".red());
                return;
            }

            println!(
                "{} {}",
                "✅ Template folder copied successfully:",
                git_template_folder.name.green().bold()
            );
        }
        println!();
        println!(
            "{} {}",
            "✅ All templates copied successfully to: ".green(),
            config.path.to_str().unwrap()
        );
        println!();
    }
}
