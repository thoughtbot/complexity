use ignore::DirEntry;
use std::ffi::OsStr;

pub struct FilesFilter<'a> {
    ignored_extensions: Vec<&'a OsStr>,
    ignored_paths: Vec<&'a str>,
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
            ignored_paths: vec!["vendor"],
        }
    }
}

impl<'a> FilesFilter<'a> {
    pub fn matches(&self, entry: &DirEntry) -> bool {
        self.approved_extension(entry.path()) && self.approved_path(entry.path())
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
}
