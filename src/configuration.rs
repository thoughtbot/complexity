use dirs_next;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn install() -> Option<PathBuf> {
    let path = config_path().map(PathBuf::from)?;

    path.parent()
        .and_then(|path| fs::create_dir_all(path).ok())?;
    fs::write(&path, include_str!("templates/config.yml")).ok()?;

    Some(path)
}

pub fn load_and_parse_config() -> Option<String> {
    config_path().and_then(|path| read_file(&path).ok())
}

fn config_path() -> Option<String> {
    file_path_in_home_dir(".config/complexity/complexity.yml")
}

fn file_path_in_home_dir(file_name: &str) -> Option<String> {
    dirs_next::home_dir()
        .and_then(|ref p| Path::new(p).join(file_name).to_str().map(|v| v.to_owned()))
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}
