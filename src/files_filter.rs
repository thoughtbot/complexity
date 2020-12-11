use std::ffi::OsStr;

pub struct FilesFilter<'a> {
    ignored_extensions: Vec<&'a OsStr>,
    pub ignored_paths: Vec<String>,
    pub only_paths: Vec<String>,
}

impl<'a> Default for FilesFilter<'a> {
    fn default() -> Self {
        Self {
            ignored_extensions: vec![
                OsStr::new("json"),
                OsStr::new("lock"),
                OsStr::new("toml"),
                OsStr::new("yml"),
                OsStr::new("yaml"),
                OsStr::new("md"),
                OsStr::new("markdown"),
                OsStr::new("xml"),
                OsStr::new("svg"),
            ],
            ignored_paths: vec!["vendor".to_string()],
            only_paths: vec![],
        }
    }
}

impl<'a> FilesFilter<'a> {
    pub fn matches(&self, path: &std::path::Path) -> bool {
        self.approved_extension(path) && self.approved_path(path) && self.only_path(path)
    }

    fn approved_extension(&self, path: &std::path::Path) -> bool {
        match path.extension() {
            Some(ext) => !self.ignored_extensions.contains(&ext),
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
}
