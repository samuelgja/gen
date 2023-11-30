use lazy_static::lazy_static;
use regex::Regex;

pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CLI_NAME: &str = env!("CARGO_PKG_NAME");
pub const CLI_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const CLI_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const CLI_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const CONFIG_FILE: &str = "_.json";

pub const TEMPLATE_DOCS_URL: &str = "https://github.com/samuelgja/gen";
pub const TEMPLATE_VARIABLE: &str = "#var";
pub const TEMPLATE_SELECT: &str = "#select";
pub const TEMPLATE_ROOT_FOLDER: &str = ".gen";

lazy_static! {
    // it can match TEMPLATE_VARIABLE or TEMPLATE_VARIABLE + any number
    // it can also be in format - #var1 or #var2 or
    pub static ref TEMPLATE_VARIABLE_REGEX: Regex = Regex::new(r"\#var(\d+)?").unwrap();
    pub static ref TEMPLATE_SELECT_REGEX: Regex = Regex::new(r"\#select(\d+)?").unwrap();
}
