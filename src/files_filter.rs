use serde::Deserialize;
use std::ffi::OsString;

pub struct FilesFilter {
    ignored_extensions: Vec<OsString>,
    pub ignored_paths: Vec<String>,
    pub only_paths: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct IgnoredFilter {
    #[serde(default)]
    ignored_extensions: Vec<String>,
    #[serde(default)]
    ignored_paths: Vec<String>,
}

impl IgnoredFilter {
    pub fn from_file(body: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(body)
    }
}

impl std::convert::From<IgnoredFilter> for FilesFilter {
    fn from(input: IgnoredFilter) -> Self {
        let ignored_extensions = input
            .ignored_extensions
            .into_iter()
            .map(|v| OsString::from(v))
            .collect::<Vec<_>>();

        let ignored_paths = input.ignored_paths;

        FilesFilter {
            ignored_extensions,
            ignored_paths,
            ..FilesFilter::default()
        }
    }
}

impl Default for FilesFilter {
    fn default() -> Self {
        Self {
            ignored_extensions: vec![],
            ignored_paths: vec![],
            only_paths: vec![],
        }
    }
}

impl FilesFilter {
    pub fn matches(&self, path: &std::path::Path) -> bool {
        self.approved_extension(path) && self.approved_path(path) && self.only_path(path)
    }

    fn approved_extension(&self, path: &std::path::Path) -> bool {
        match path.extension() {
            Some(ext) => !self.ignored_extensions.contains(&ext.to_os_string()),
            None => true,
        }
    }

    fn approved_path(&self, path: &std::path::Path) -> bool {
        self.ignored_paths
            .iter()
            .all(|ignored| !path.to_str().unwrap_or("").contains(ignored))
    }

    fn only_path(&self, path: &std::path::Path) -> bool {
        if path.is_dir() {
            return true;
        }

        if self.only_paths.len() > 0 {
            self.only_paths
                .iter()
                .any(|p| path.to_str().unwrap_or("").contains(p))
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use totems::assert_contains;

    #[test]
    fn only_path_allows_directories() {
        // if only_path is a directory and doesn't match the value, it will prevent that directory
        // from being traversed further
        let mut files_filter = FilesFilter::default();
        files_filter.only_paths.push("js".to_string());

        assert!(files_filter.only_path(Path::new("./src/")));
        assert!(files_filter.only_path(Path::new("./src/nested.js")));
        assert!(!files_filter.only_path(Path::new("./src/nested.rb")));
    }

    #[test]
    fn parsing_provided_yaml() {
        let provided_yaml = include_str!("templates/config.yml");

        let parsed_value = IgnoredFilter::from_file(&provided_yaml).unwrap();
        // assert_contains!(&parsed_value.ignored_paths, "vendor");
        assert_contains!(&parsed_value.ignored_extensions, "xml");
        assert_contains!(&parsed_value.ignored_extensions, "svg");
        assert_contains!(&parsed_value.ignored_extensions, "lock");
    }
}
