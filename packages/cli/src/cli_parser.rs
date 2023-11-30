use crate::{
    actions::TemplateAction, cli_commands::CliCommands, config::Config,
    constants::TEMPLATE_ROOT_FOLDER,
};

use std::{collections::HashSet, env, path::Path};

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
    List,
    Edit,
    Delete,
    Publish,
    Unpublish,
    Global,
    Help,
    Version,
}
impl Commands {
    pub fn command_str(&self) -> String {
        match self {
            Commands::New => "new".to_owned(),
            Commands::List => "list".to_owned(),
            Commands::Edit => "edit".to_owned(),
            Commands::Delete => "delete".to_owned(),
            Commands::Publish => "publish".to_owned(),
            Commands::Unpublish => "unpublish".to_owned(),
            Commands::Help => "help".to_owned(),
            Commands::Version => "version".to_owned(),
            Commands::Global => "--global".to_owned(),
        }
    }

    pub fn command_str_short(&self) -> String {
        match self {
            Commands::New => "n".to_owned(),
            Commands::List => "l".to_owned(),
            Commands::Edit => "e".to_owned(),
            Commands::Delete => "d".to_owned(),
            Commands::Publish => "p".to_owned(),
            Commands::Unpublish => "u".to_owned(),
            Commands::Help => "h".to_owned(),
            Commands::Version => "v".to_owned(),
            Commands::Global => "-g".to_owned(),
        }
    }

    pub fn is_command(&self, arguments: &HashSet<String>) -> bool {
        arguments.contains(&self.command_str()) || arguments.contains(&self.command_str_short())
    }
}
impl CliParser {
    pub fn parse() {
        let arguments: Vec<String> = env::args().skip(1).collect();
        let cwd = env::current_dir().unwrap();

        let config_dir_path = Path::new(&cwd).join(TEMPLATE_ROOT_FOLDER).to_owned();
        let _global_config_dir_path = Path::new(&env::var("HOME").unwrap())
            .join(TEMPLATE_ROOT_FOLDER)
            .to_owned();

        let config = Config::load_template_folders(&config_dir_path);

        let arguments: HashSet<String> = HashSet::from_iter(arguments);

        let is_global = Commands::Global.is_command(&arguments);

        if Commands::Help.is_command(&arguments) || arguments.is_empty() {
            println!("DICK");
            return;
        }

        if Commands::New.is_command(&arguments) {
            TemplateAction::new_template(&config);
        }

        if Commands::List.is_command(&arguments) {
            let template_folders = config
                .templates
                .iter()
                .map(|item| item.name.to_owned())
                .collect::<Vec<_>>();
            println!();
            let selected = CliCommands::select("📝 Select template to use", &config.templates);
        }
    }
}
