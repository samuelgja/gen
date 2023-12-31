use crate::{
    constants::CONFIG_FILE, template::TemplateConfig, template_variable::TemplateVariableInfo,
};
use indexmap::IndexMap;
use rust_search::SearchBuilder;
use std::path::{Path, PathBuf};

pub struct SearchFolder;

#[derive(Debug)]
pub struct SearchItem {
    pub path: PathBuf,
    pub template_path: PathBuf,
    pub is_config: bool,
}

#[derive(Debug)]
pub struct SearchResult {
    pub files: Vec<SearchItem>,
    pub variables: IndexMap<String, TemplateVariableInfo>,
    pub is_within_one_folder: bool,
    pub template_config: TemplateConfig,
}

impl SearchFolder {
    pub fn search_files(template_path: &PathBuf) -> Vec<SearchItem> {
        let search: Vec<String> = SearchBuilder::default()
            .location(template_path)
            .strict()
            .ignore_case()
            .hidden()
            .build()
            .collect();

        let mut files = search
            .iter()
            .filter_map(|item| {
                let path = Path::new(item);
                if path.is_file() {
                    let is_config = path.file_name().unwrap() == CONFIG_FILE;
                    let template_path = path.strip_prefix(template_path).unwrap().to_path_buf();
                    return Some(SearchItem {
                        path: path.to_path_buf(),
                        template_path,
                        is_config,
                    });
                }
                None
            })
            .collect::<Vec<_>>();

        // sort by path length
        files.sort_by(|a, b| {
            a.path
                .to_str()
                .unwrap()
                .len()
                .cmp(&b.path.to_str().unwrap().len())
        });
        files
    }

    pub fn search(template_path: &PathBuf) -> SearchResult {
        let files = SearchFolder::search_files(template_path);
        let mut variables = IndexMap::new();

        if files.is_empty() {
            return SearchResult {
                files,
                variables,
                is_within_one_folder: false,
                template_config: TemplateConfig::load_template_config_from_path(template_path),
            };
        }

        let start_dir_path = files[0].path.parent().unwrap().to_path_buf();
        let mut is_within_one_folder = true;
        for file in files.iter() {
            let content = std::fs::read_to_string(&file.path).unwrap();
            for variable in TemplateVariableInfo::parse_iter(&content) {
                let key = SearchFolder::get_key(&variable);
                variables.insert(key, variable);
            }

            if is_within_one_folder && !file.path.starts_with(&start_dir_path) {
                is_within_one_folder = false;
            }

            // iter over path parts
            for part in file.template_path.iter() {
                let part = part.to_str().unwrap();

                for variable in TemplateVariableInfo::parse_iter(part) {
                    let key = SearchFolder::get_key(&variable);
                    variables.insert(key, variable);
                }
            }
        }

        variables.sort_keys();

        SearchResult {
            files,
            variables,
            is_within_one_folder,
            template_config: TemplateConfig::load_template_config_from_path(template_path),
        }
    }

    pub fn get_key(variable: &TemplateVariableInfo) -> String {
        format!("{}_{}", variable.template_variable, variable.var_name,)
    }
}
