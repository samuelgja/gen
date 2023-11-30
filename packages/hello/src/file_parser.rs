use convert_case::{Case, Casing};
use rust_search::SearchBuilder;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::CaseType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsedName {
    pub name: String,
    pub last_part: Option<String>,
    pub first_part: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileParser {
    pub parsed_name: Option<ParsedName>,
    pub folder: String,
    pub not_same_name_folder: String,
    pub name: String,
    pub extension: String,
    pub parent_name: String,
    pub case: CaseType,
    pub path: String,
}

impl FileParser {
    fn parse_name(ignore_set: &HashSet<&&str>, value: &str) -> Option<ParsedName> {
        // remove from name common words
        // split value by - or _ or .

        let splitted = value.split(|c| c == '-' || c == '_' || c == '.');
        let items_length = &splitted.clone().count();
        let last = &splitted.clone().last();

        let last = last.unwrap();
        let last = last.to_lowercase();

        if let Some(remove) = ignore_set.get(&last.as_str()) {
            let new_name = splitted
                .clone()
                .take(items_length - 1)
                .collect::<Vec<&str>>()
                .join("");
            if items_length == &1 {
                return None;
            }
            if new_name.is_empty() {
                return None;
            }
            return Some(ParsedName {
                name: new_name,
                last_part: Some(remove.to_string()),
                first_part: None,
            });
        }
        let first = &splitted.clone().next();
        let first = first.unwrap();

        let first = first.to_lowercase();

        if let Some(remove) = ignore_set.get(&first.as_str()) {
            let new_name = splitted
                .clone()
                .skip(1)
                .take(items_length - 1)
                .collect::<Vec<&str>>()
                .join("");
            if items_length == &1 {
                return None;
            }
            if new_name.is_empty() {
                return None;
            }

            return Some(ParsedName {
                name: new_name,
                last_part: None,
                first_part: Some(remove.to_string()),
            });
        }

        Some(ParsedName {
            name: value.to_string(),
            last_part: None,
            first_part: None,
        })
    }
    pub fn get_files_in_root_folder(root_path: &str) -> Vec<FileParser> {
        let ignore_list = vec![
            "component",
            "container",
            "page",
            "view",
            "screen",
            "layout",
            "template",
            "index",
            "test",
            "controller",
            "db",
            "database",
            "model",
            "schema",
            "service",
            "util",
            "helper",
            "style",
            "styles",
            "config",
            "constant",
            "constants",
            "context",
            "hook",
            "hooks",
            "provider",
            "providers",
            "store",
            "stores",
            "action",
            "actions",
            "reducer",
            "reducers",
            "saga",
            "sagas",
            "selector",
            "selectors",
            "type",
            "types",
            "interface",
            "interfaces",
            "route",
            "routes",
            "router",
            "routers",
            "navigation",
            "navigations",
            "navigator",
            "navigators",
            "event",
            "events",
            "listener",
            "job",
            "jobs",
            "use",
            "state",
            "option",
            "options",
            "handler",
            "resolver",
            "test.utils",
            "table",
        ];
        let ignore_list_set = ignore_list.iter().collect::<HashSet<_>>();

        let accepted_extensions = vec![
            "js", "jsx", "ts", "tsx", "rs", "go", "py", "php", "css", "scss", "sass", "less",
            "html",
        ];
        let accepted_extensions_set = accepted_extensions.iter().collect::<HashSet<_>>();
        let mut search: Vec<FileParser> = SearchBuilder::default()
            .location(root_path)
            .strict()
            .ignore_case()
            .build()
            .filter(|search| {
                if search.contains("node_modules") {
                    return false;
                }
                if search.contains("build") {
                    return false;
                }
                if search.contains("dist") {
                    return false;
                }
                if search.contains("coverage") {
                    return false;
                }
                if search.contains("public") {
                    return false;
                }

                // filter out folders
                let path = Path::new(&search);
                let extension = path.extension();
                let parent_name = path.parent().unwrap().file_name();
                if extension.is_none() {
                    return false;
                }

                if !accepted_extensions_set.contains(&extension.unwrap().to_str().unwrap()) {
                    return false;
                }
                if parent_name.is_none() {
                    return false;
                }
                !search.is_empty()
            })
            .map(|full_path| {
                // replace root_path path with empty string
                let new_search = full_path.replace(root_path, "");
                let path = Path::new(&new_search);
                let extension = path.extension().unwrap().to_str().unwrap();
                let base_name = path.file_name().unwrap().to_str().unwrap();
                let base_name = base_name.replace(&format!(".{}", extension), "");
                let case = CaseType::from_str(base_name.as_str());
                let base_name = base_name.to_case(Case::Snake);
                let parent_path = path.parent().unwrap();
                let parent_name = parent_path.file_name();
                let folder = parent_path.to_str().unwrap().to_string();
                let parsed_name = FileParser::parse_name(&ignore_list_set, &base_name);
                if parent_name.is_none() {
                    return FileParser {
                        folder: folder.clone(),
                        not_same_name_folder: folder,
                        name: base_name.to_string(),
                        extension: extension.to_string(),
                        parent_name: "".to_string(),
                        parsed_name,
                        case,
                        path: full_path,
                    };
                }

                let base_name_full = if let Some(name) = &parsed_name {
                    name.name.to_string()
                } else {
                    base_name.to_string()
                };

                let parent_name = parent_name.unwrap().to_str().unwrap().to_string();

                let not_same_name_folder: String;
                if &base_name_full == &parent_name {
                    if let Some(parent_parent_name) = parent_path.parent() {
                        not_same_name_folder = parent_parent_name.to_str().unwrap().to_string();
                    } else {
                        not_same_name_folder = folder.clone();
                    }
                } else {
                    not_same_name_folder = folder.clone();
                }
                FileParser {
                    folder,
                    not_same_name_folder,
                    name: base_name.to_string(),
                    extension: extension.to_string(),
                    parent_name,
                    parsed_name,
                    case,
                    path: full_path,
                }
            })
            .collect();

        search.sort();

        search
    }

    pub fn get_same_files(root_path: &str) {
        let files = FileParser::get_files_in_root_folder(root_path);

        let mut hash_result = HashMap::new();
        for main_file in files.iter() {
            let mut same_files = files
                .iter()
                .filter(|next| {
                    if next.path == main_file.path {
                        return true;
                    }

                    let next_parsed_name = if let Some(parsed) = &next.parsed_name {
                        &parsed.name
                    } else {
                        &next.name
                    };
                    let parsed_name = if let Some(parsed) = &main_file.parsed_name {
                        &parsed.name
                    } else {
                        &main_file.name
                    };

                    if next_parsed_name != parsed_name {
                        return false;
                    }

                    true
                })
                .collect::<Vec<_>>();

            if same_files.is_empty() {
                continue;
            }

            same_files.sort();
            let generated_key = same_files
                .iter()
                .map(|file| file.path.to_owned())
                .collect::<Vec<_>>()
                .join("");
            hash_result.insert(generated_key, same_files);
        }

        for (_key, same_files) in hash_result.iter() {
            if same_files.len() > 2 {
                for same_file in same_files.iter() {
                    println!("{}", same_file.path);
                    println!("{:?}", same_file.not_same_name_folder);
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let root_path = "/Users/samuelgjabel/Documents/ROBOTA/jigx";
        // let root_path = "/Users/samuelgjabel/Documents/SUKROMNE/storage-engine-rust";

        FileParser::get_same_files(root_path);
        // println!("{:?}", result);
    }
}
