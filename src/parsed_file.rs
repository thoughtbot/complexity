use crate::{parser, scoring};
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub complexity_score: f32,
}

pub enum ParsedFileError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for ParsedFileError {
    fn from(err: std::io::Error) -> Self {
        ParsedFileError::IoError(err)
    }
}

impl ParsedFile {
    pub fn new(
        scorer: &mut Box<dyn scoring::ScoreVisitor>,
        path: PathBuf,
    ) -> Result<Self, ParsedFileError> {
        let contents = get_file_contents(&path)?;
        let stats = parser::parse_file(&contents);
        let complexity_score = scoring::score(scorer, &stats);

        Ok(ParsedFile {
            path,
            complexity_score,
        })
    }
}

fn get_file_contents(path: &PathBuf) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
