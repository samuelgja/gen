use crate::{
    actions::TemplateAction,
    cli_commands::CliCommands,
    config::{Config, ConfigFile},
    constants::TEMPLATE_ROOT_FOLDER,
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

enum Commands {
    New,
    Select,
    Edit,
    Delete,
    Publish,
    Unpublish,
    Global,
    Refresh,
    Help,
    Version,
}
impl Commands {
    pub fn command_str(&self) -> String {
        match self {
            Commands::New => "new".to_owned(),
            Commands::Edit => "edit".to_owned(),
            Commands::Delete => "delete".to_owned(),
            Commands::Publish => "publish".to_owned(),
            Commands::Unpublish => "unpublish".to_owned(),
            Commands::Help => "help".to_owned(),
            Commands::Version => "version".to_owned(),
            Commands::Global => "--global".to_owned(),
            Commands::Refresh => "refresh".to_owned(),
            Commands::Select => "select".to_owned(),
        }
    }

    pub fn command_str_short(&self) -> String {
        match self {
            Commands::New => "n".to_owned(),
            Commands::Edit => "e".to_owned(),
            Commands::Delete => "d".to_owned(),
            Commands::Publish => "p".to_owned(),
            Commands::Unpublish => "u".to_owned(),
            Commands::Help => "h".to_owned(),
            Commands::Version => "v".to_owned(),
            Commands::Global => "-g".to_owned(),
            Commands::Refresh => "r".to_owned(),
            Commands::Select => "s".to_owned(),
        }
    }

    pub fn is_command(&self, arguments: &HashSet<String>) -> bool {
        arguments.contains(&self.command_str()) || arguments.contains(&self.command_str_short())
    }
}
impl CliParser {
    pub fn parse() {
        let vec_arguments: Vec<String> = env::args().skip(1).collect();
        let vec_arguments_cloned = vec_arguments.clone();
        let second_argument = &vec_arguments_cloned.get(1);
        let arguments: HashSet<String> = HashSet::from_iter(vec_arguments);
        let is_global = Commands::Global.is_command(&arguments);
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

        if Commands::Help.is_command(&arguments) || arguments.is_empty() {
            println!("DICK");
            return;
        }

        if Commands::New.is_command(&arguments) {
            if let Some(template_name) = second_argument {
                let template_folder = TemplateFolder::new(config, template_name);
                CliParser::edit_create_selected_template(config, &template_folder);
                return;
            }
            TemplateAction::new_template(&config);
        }

        if Commands::Edit.is_command(&arguments) {
            let template_folder = CliParser::get_list(config);
            if template_folder.is_err() {
                return;
            }

            CliParser::edit_create_selected_template(config, &template_folder.unwrap());
        }

        if Commands::Select.is_command(&arguments) {
            let template_folder = if let Some(template_name) = second_argument {
                Ok(TemplateFolder::new(config, template_name))
            } else {
                CliParser::get_list(config)
            };
            if template_folder.is_err() {
                return;
            }
            let template_folder = template_folder.unwrap();
            // TODO
        }

        if Commands::Delete.is_command(&arguments) {
            let template_folder = if let Some(template_name) = second_argument {
                Ok(TemplateFolder::new(config, template_name))
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
        }
    }

    fn get_list(config: &Config) -> Result<TemplateFolder, ()> {
        let template_folders = config
            .templates
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
        let selected = CliCommands::select("📝 Select template to use", &config.templates);
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
            .templates
            .iter()
            .find(|item| item.name == template_name);
        if template_folder.is_none() {
            println!();
            println!(
                "{}",
                format!("🚨 Template {} does not exist", template_name).red()
            );
            return None;
        }
        let template_folder = template_folder.unwrap();
        Some(template_folder.to_owned())
    }
}
