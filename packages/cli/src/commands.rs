use std::collections::HashSet;

use colored::Colorize;

use crate::{actions::TemplateAction, constants::TEMPLATE_DOCS_URL};
pub enum Commands {
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
    Fetch,
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
            Commands::Fetch => "fetch".to_owned(),
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
            Commands::Fetch => "f".to_owned(),
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
                "Refresh templates. But this is also triggered when selecting template.{}",
                support_global
            )
            .to_owned(),
            Commands::Select => format!("Select template. {}", support_global).to_owned(),
            Commands::Fetch => format!(
                "Fetch templates from github, url or local path. {}",
                support_global
            )
            .to_owned(),
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
            "Template variables are used to replace #var part of template content or template path with any name.".italic()
        );
        println!();
        println!(
            "{:width$}: {}",
            "#var".green().bold(),
            "Simple as #var or #var1, #var2, #var3, ..., or #var_anything",
        );
        println!(
            "{:width$}: {}",
            "#select".green().bold(),
            "Simple as #select or #select1, #select2, #select3, ..., or #select_anything",
        );
        println!(
            "{:width$}: {}",
            "Case support".green().bold(),
            "#var and #select also support suffix with casing. _kebab, _snake, _camel, _pascal.\nFor example #var_kebab or #select_author_camel. So case words are reserved.",
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
        Commands::print_usage_item(Commands::Select);
        Commands::print_usage_item(Commands::Global);

        println!();
        println!();
        println!(
            "For more info visit: {}",
            TEMPLATE_DOCS_URL.yellow().underline().bold()
        );
        println!();
    }

    pub fn is_command(&self, arguments: &HashSet<String>) -> bool {
        arguments.contains(&self.command_str()) || arguments.contains(&self.command_str_short())
    }
}
