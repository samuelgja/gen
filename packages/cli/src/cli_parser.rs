use std::env;

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
pub struct CliParser<'a> {
    pub template_name: &'a str,
}

impl<'a> CliParser<'a> {
    pub fn parse() -> CliParser<'a> {
        let arguments: Vec<String> = env::args().skip(1).collect();

        println!("{:?}", arguments);
        CliParser { template_name: "" }
    }
}
