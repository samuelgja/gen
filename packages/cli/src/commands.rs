use std::collections::HashSet;

use colored::Colorize;

use crate::{actions::TemplateAction, constants::TEMPLATE_DOCS_URL};
pub enum Commands {
    New,
    Use,
    Edit,
    Delete,
    Publish,
    Unpublish,
    Global,
    Refresh,
    Help,
    Version,
    Fetch,
    VariablesList,
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
            Commands::Use => "use".to_owned(),
            Commands::Fetch => "fetch".to_owned(),
            Commands::VariablesList => "variables".to_owned(),
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
            Commands::Use => "u".to_owned(),
            Commands::Fetch => "f".to_owned(),
            Commands::VariablesList => "vv".to_owned(),
        }
    }

    pub fn command_description(&self) -> String {
        let support_global = "Support global flag, second argument [template_name]";
        match self {
            Commands::New => format!("Create new template. If running local template, template folder will be generated to current (cwd) path. {}", support_global).to_owned(),
            Commands::Edit => format!("Edit template. {}", support_global).to_owned(),
            Commands::Delete => format!("Delete template. {}", support_global).to_owned(),
            Commands::Publish => format!("Publish template. {}", support_global).to_owned(),
            Commands::Unpublish => format!("Unpublish template. {}", support_global).to_owned(),
            Commands::Help => "Show help".to_owned(),
            Commands::Version => "Show version".to_owned(),
            Commands::Global => "Global scope".to_owned(),
            Commands::Refresh => format!(
                "Refresh templates all templates.{}",
                support_global
            )
            .to_owned(),
            Commands::Use => format!("Use / template. {}", support_global).to_owned(),
            Commands::Fetch => format!(
                "Fetch templates from github url (todo any url). {}",
                support_global
            )
            .to_owned(),
            Commands::VariablesList => format!("List of all templates variables").to_owned(),
        }
    }

    pub fn print_usage_item(command: Commands) {
        let width = 10;
        let width2 = 2;
        println!(
            "{:width$} short: {:width2$} -> {}",
            command.command_str().bold().green(),
            command.command_str_short().bold().green(),
            command.command_description(),
        );
    }

    pub fn print_help() {
        println!();
        println!("{}", "🤷🏻 How it works?".bold().magenta());
        println!();
        println!(
            "{} {} {} {} {}",
            "Easy to use template generator for any code any project with support of having private / public github templates, website templates & more.\nTo quick start start with",  
            "new".bold().green(),
            "and then",
            "select".bold().green(),
            "commands. To see more, check out the usage below:",

        );

        let width = 14;
        println!();
        TemplateAction::print_file_steps();
        println!();
        println!("{}", "📚 Variables usage:".bold().magenta());
        println!();
        println!(
            "{}",
            "Template variables are used to replace __var__ part of template content or template path with any name.".italic()
        );
        println!();
        println!(
            "{:width$}: {}",
            "__var__".green().bold(),
            "Simple as __var__ or __var__something1__, __var__2__, __var__3__, ..., or __var__anything",
        );
        println!(
            "{:width$}: {}",
            "__select__".green().bold(),
            "Simple as __select__ or __select__something1__, __select__2__, __select__3__, ..., or __select___anything",
        );
        println!(
            "{:width$}: {}",
            "Case support".green().bold(),
            "__var__ and __select__ also support casing. kebab__, snake__, camel__, pascal__.\nFor example __var__kebab__ or __select___author__camel__. So case words are reserved.",
        );
        println!();
        println!("{}", "🧩 Commands:".bold().magenta());
        println!();
        Commands::print_usage_item(Commands::New);
        Commands::print_usage_item(Commands::Edit);
        Commands::print_usage_item(Commands::Delete);
        // Commands::print_usage_item(Commands::Publish);
        // Commands::print_usage_item(Commands::Unpublish);
        Commands::print_usage_item(Commands::Fetch);
        Commands::print_usage_item(Commands::Help);
        Commands::print_usage_item(Commands::Version);
        Commands::print_usage_item(Commands::Refresh);
        Commands::print_usage_item(Commands::Use);
        Commands::print_usage_item(Commands::VariablesList);
        Commands::print_usage_item(Commands::Global);

        println!();
        println!();
        println!(
            "For more info visit: {}",
            TEMPLATE_DOCS_URL.yellow().underline().bold()
        );
        println!();
    }

    pub fn is_command_from_set(&self, arguments: &HashSet<String>) -> bool {
        arguments.contains(&self.command_str()) || arguments.contains(&self.command_str_short())
    }

    pub fn is_command(argument: &str) -> bool {
        let commands = vec![
            Commands::New,
            Commands::Edit,
            Commands::Delete,
            Commands::Publish,
            Commands::Unpublish,
            Commands::Help,
            Commands::Version,
            Commands::Global,
            Commands::Refresh,
            Commands::Use,
            Commands::Fetch,
            Commands::VariablesList,
        ];
        for command in commands {
            if argument == command.command_str() || argument == command.command_str_short() {
                return true;
            }
        }
        false
    }

    pub fn return_unknown_arguments(arguments: &Vec<String>) -> Vec<String> {
        let mut unknown_arguments = vec![];
        for argument in arguments {
            if !Commands::is_command(argument) {
                unknown_arguments.push(argument.to_string());
            }
        }
        unknown_arguments
    }
}
