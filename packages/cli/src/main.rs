mod case_util;
mod cli_parser;
mod commands;
mod template_type;

use cli_parser::CliParser;
use dialoguer::{theme::ColorfulTheme, Confirm, Select, Sort};

/**
 * CLI TOOL - name gen - but gen is name of the tool, so it will be in bash profile.
 * Template arguments:
 * - empty - will show list of templates and after select, it will go to template name selection
 * - [template name] - will go to template name selection
 * - [template name] [custom name] - will go to template file select
 * - new - will go new to template editor
 *
 * Flags for template arguments:
 * --edit -e - will go to template editor
 * --delete -d - will delete template
 * --publish -p - will publish template to github
 * --unpublish -u - will unpublish template from github
 *
 * Global flags:
 * --help -h - will show help
 * --version -v - will show version
 * --list -l - will show list of templates - but same as empty template argument
 * --search -s - will search for templates on github
 */
fn main() {
    let parsed = CliParser::parse();
    print!("{:?}", parsed);
}
