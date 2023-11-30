use crate::{actions::TemplateAction, commands::Commands, config::Config};

use std::{collections::HashSet, env, path::Path, process::Command};

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
        let arguments: Vec<String> = env::args().skip(1).collect();
        let cwd = env::current_dir().unwrap();

        let config_dir_path = Path::new(&cwd).join(".gen").to_owned();
        let global_config_dir_path = Path::new(&env::var("HOME").unwrap())
            .join(".gen")
            .to_owned();

        let mut config = Config::load_or_new(&config_dir_path);

        let arguments: HashSet<String> = HashSet::from_iter(arguments.into_iter());

        let is_global = arguments.contains("--global") || arguments.contains("-g");

        if arguments.contains("--help") || arguments.contains("-h") || arguments.len() == 0 {
            println!("DICK");
            return;
        }

        if arguments.contains("new") {
            let template = TemplateAction::new();
            // remove template with same name
            config.templates = config
                .templates
                .iter()
                .filter(|item| item.name != template.name)
                .map(|item| item.to_owned())
                .collect();
            config.templates.push(template);
            config.save(&config_dir_path).unwrap();
            return;
        }

        if arguments.contains("list") {
            let template_names = config
                .templates
                .iter()
                .map(|item| item.name.to_owned())
                .collect::<Vec<_>>();

            let selected = Commands::select("📝 Select template to use", &config.templates);
        }
    }
}
