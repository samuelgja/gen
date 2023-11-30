use rust_search::SearchBuilder;
use std::path::{Path, PathBuf};

pub struct SearchFolder;

#[derive(Debug)]
pub struct SearchItem {
    pub path: PathBuf,
    pub template_path: PathBuf,
}

impl SearchFolder {
    pub fn search(template_path: &PathBuf) -> Vec<SearchItem> {
        let search: Vec<String> = SearchBuilder::default()
            .location(&template_path)
            .strict()
            .ignore_case()
            .hidden()
            .build()
            .collect();
        // map filter
        let files = search
            .iter()
            .filter_map(|item| {
                let path = Path::new(item);
                if path.is_file() {
                    return Some(SearchItem {
                        path: path.to_path_buf(),
                        template_path: path.strip_prefix(template_path).unwrap().to_path_buf(),
                    });
                }
                None
            })
            .collect::<Vec<_>>();

        files
    }
}
