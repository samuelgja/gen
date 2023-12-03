use crate::{
    actions::{TemplateAction, TemplateFetch, TemplateUse},
    cli_commands::CliCommands,
    commands::Commands,
    config::{Config, ConfigFile},
    constants::{CLI_VERSION, TEMPLATE_ROOT_FOLDER},
    template::{TemplateConfig, TemplateFolder},
};
use colored::Colorize;
use loading::Loading;
use std::{collections::HashSet, env, fs, path::Path};
/**
 * CLI TOOL - name gen - but gen is name of the tool, so it will be in bash profile.
 * Template arguments:
 * - empty - will show list of templates and after select, it will go to template name selection
 * - [template name] - will go to template name selection
 * - [template name] [custom name] - will go to template file select
 * Flags for template arguments:
 * --edit -e - will go to template editor
 * --delete -d - will delete template
 * --publish -p - will publish template to github
 * --unpublish -u - will unpublish template from github
 *
 *
 * Template new argument:
 * - new - will go new to template editor
 *
 * Flags for template new argument:
 * --cloud -c - will create template in cloud
 * --local -l - will create template locally
 * without argument it will save it to global folder - ~/.somewhere/templates
 *
 * Global flags:
 * --help -h - will show help
 * --version -v - will show version
 * --list -l - will show list of templates - but same as empty template argument
 * --search -s - will search for templates on github
 */

#[derive(Debug)]
pub struct CliParser {}

impl CliParser {
    pub fn parse() {
        let vec_arguments: Vec<String> = env::args().skip(1).collect();
        let vec_arguments_cloned = Commands::return_unknown_arguments(&vec_arguments);
        let second_argument = &vec_arguments_cloned.get(0);
        let arguments: HashSet<String> = HashSet::from_iter(vec_arguments);
        let is_global = Commands::Global.is_command_from_set(&arguments);
        let cwd = env::current_dir().unwrap();

        let local_config_dir_path = Path::new(&cwd).join(TEMPLATE_ROOT_FOLDER).to_owned();
        let global_config_dir_path = Path::new(&env::var("HOME").unwrap())
            .join(TEMPLATE_ROOT_FOLDER)
            .to_owned();

        let mut local_config = Config::load_template_folders(&local_config_dir_path);
        let mut global_config = Config::load_template_folders(&global_config_dir_path);

        local_config.config = ConfigFile::load_config(&local_config_dir_path, !is_global);
        global_config.config = ConfigFile::load_config(&global_config_dir_path, is_global);

        let config = if is_global {
            &global_config
        } else {
            local_config.config.merge(&global_config.config);
            &local_config
        };

        if config.config.open_editor_command.is_none() {
            global_config.config.open_editor_command = TemplateAction::get_template_command_args();
            local_config.config.open_editor_command =
                global_config.config.open_editor_command.clone();
            global_config.config.save_config(&global_config_dir_path);
        }

        let config = if is_global {
            &global_config
        } else {
            local_config.config.merge(&global_config.config);
            &local_config
        };

        if Commands::Help.is_command_from_set(&arguments) || arguments.is_empty() {
            Commands::print_help();
            return;
        }

        if Commands::Version.is_command_from_set(&arguments) {
            println!("{} {}", "Version:", CLI_VERSION.bold());
            return;
        }

        if Commands::New.is_command_from_set(&arguments) {
            if let Some(template_name) = second_argument {
                let template_folder = TemplateFolder::new(config, template_name);
                CliParser::edit_create_selected_template(config, &template_folder);
                return;
            }
            TemplateAction::new_template(&config);
            return;
        }

        if Commands::Edit.is_command_from_set(&arguments) {
            let template_folder = if let Some(template_name) = second_argument {
                Ok(TemplateFolder::new_empty(config, template_name))
            } else {
                CliParser::get_list(config)
            };
            if template_folder.is_err() {
                return;
            }
            let template_folder = template_folder.unwrap();

            if !CliParser::is_exist_and_prompt(config, &template_folder) {
                return;
            }

            CliParser::edit_create_selected_template(config, &template_folder);
            return;
        }

        if Commands::Refresh.is_command_from_set(&arguments) {
            TemplateAction::refresh_templates(config);
            return;
        }
        if Commands::VariablesList.is_command_from_set(&arguments) {
            TemplateAction::list_of_all_variables(config);
            return;
        }

        if Commands::Fetch.is_command_from_set(&arguments) {
            if let Some(url) = second_argument {
                TemplateFetch::fetch_github(config, url);
                return;
            } else {
                println!("{}", "🚨 Missing url, github url or path argument.".red());
            }

            return;
        }

        if Commands::Use.is_command_from_set(&arguments) {
            let template_folder = if let Some(template_name) = second_argument {
                Ok(TemplateFolder::new_empty(config, template_name))
            } else {
                CliParser::get_list(config)
            };
            if template_folder.is_err() {
                return;
            }
            let template_folder = template_folder.unwrap();
            if !CliParser::is_exist_and_prompt(config, &template_folder) {
                return;
            }

            TemplateUse::use_it(&global_config, &config, &template_folder);

            return;
        }

        if Commands::Delete.is_command_from_set(&arguments) {
            let template_folder = if let Some(template_name) = second_argument {
                Ok(TemplateFolder::new_empty(config, template_name))
            } else {
                CliParser::get_list(config)
            };
            if template_folder.is_err() {
                return;
            }
            let template_folder = template_folder.unwrap();
            let text = format!(
                "{} {}",
                "🚨 Are you sure you want to delete".red(),
                template_folder.name.bold().green()
            );
            let is_ok = CliCommands::confirm(&text);
            if is_ok {
                let loading = Loading::default();
                loading.text("Removing template...".blue());
                let result = fs::remove_dir_all(&template_folder.path);
                if result.is_err() {
                    println!();
                    println!(
                        "{}",
                        format!(
                            "🚨 Error while removing template: {}",
                            result.err().unwrap()
                        )
                        .red()
                    );
                    loading.end();
                    return;
                }
                println!();
                loading.success("Template removed.".green());
                loading.end();
            }
            return;
        }
    }

    fn is_exist_and_prompt(config: &Config, template_folder: &TemplateFolder) -> bool {
        let is_template_folder_exist = config
            .template_folders
            .iter()
            .any(|item| item.name == template_folder.name);
        if is_template_folder_exist {
            return true;
        }
        println!();
        println!(
            "{} {}",
            "🚨 Template does not exist:".red(),
            template_folder.name
        );
        println!();
        let similar_word_match = CliParser::get_similar_word_match(config, &template_folder.name);
        if similar_word_match.is_some() {
            println!(
                "{}",
                format!(
                    "🤔 Did you mean {}?",
                    similar_word_match.unwrap().name.bold().green()
                )
                .yellow()
            );
        }
        return false;
    }
    fn get_list(config: &Config) -> Result<TemplateFolder, ()> {
        let template_folders = config
            .template_folders
            .iter()
            .map(|item| item.name.to_owned())
            .collect::<Vec<_>>();
        if template_folders.len() == 0 {
            println!();
            println!("{}", "🚨 There are no templates created yet.".red());
            println!();
            return Err(());
        }
        println!();
        let selected = CliCommands::select("📝 Select template to use", &config.template_folders);
        selected
    }

    fn edit_create_selected_template(config: &Config, template_folder: &TemplateFolder) {
        let text = format!(
            "{} {} {}",
            "🔧 Creating / Editing",
            template_folder.name.bold().green(),
            "template."
        );
        println!();
        println!("{}", text);
        let mut template_config = TemplateConfig::load_template_config(&template_folder);
        template_config.name = template_folder.name.to_owned();
        template_config.save_template_config(&template_folder);
        TemplateAction::template_edit(&config, &template_folder, &mut template_config);
    }

    fn get_template(config: &Config, template_name: &str) -> Option<TemplateFolder> {
        let template_folder = config
            .template_folders
            .iter()
            .find(|item| item.name == template_name);
        if template_folder.is_none() {
            println!();
            println!(
                "{}",
                format!("🚨 Template {} does not exist", template_name).red()
            );

            let similar_word_match = CliParser::get_similar_word_match(config, template_name);
            if similar_word_match.is_some() {
                println!(
                    "{}",
                    format!(
                        "🤔 Did you mean {}?",
                        similar_word_match.unwrap().name.bold().green()
                    )
                    .yellow()
                );
            }
            return None;
        }
        let template_folder = template_folder.unwrap();
        Some(template_folder.to_owned())
    }

    fn get_fuzzy_score(template_name: &str, template_name_to_match: &str) -> i64 {
        let mut score = 0;
        let mut template_name_chars = template_name.chars();
        let mut template_name_to_match_chars = template_name_to_match.chars();
        let mut template_name_char = template_name_chars.next();
        let mut template_name_to_match_char = template_name_to_match_chars.next();
        while template_name_char.is_some() && template_name_to_match_char.is_some() {
            if template_name_char.unwrap() == template_name_to_match_char.unwrap() {
                score += 1;
            }
            template_name_char = template_name_chars.next();
            template_name_to_match_char = template_name_to_match_chars.next();
        }

        score
    }

    fn get_similar_word_match(config: &Config, template_name: &str) -> Option<TemplateFolder> {
        // fuzzy score (sort) template folders by closest match to template_name
        let mut sorted_template_folders = config
            .template_folders
            .iter()
            .map(|item| (item, CliParser::get_fuzzy_score(&item.name, template_name)))
            .filter(|item| item.1 > 0)
            .collect::<Vec<_>>();
        sorted_template_folders.sort_by(|a, b| b.1.cmp(&a.1));
        let template_folder = sorted_template_folders.first();
        if template_folder.is_none() {
            return None;
        }
        let template_folder = template_folder.unwrap();
        Some(template_folder.0.to_owned())
    }
}
