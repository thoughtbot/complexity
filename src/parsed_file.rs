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
    IncompleteParse,
    FailedParse,
}

impl From<std::io::Error> for ParsedFileError {
    fn from(err: std::io::Error) -> Self {
        ParsedFileError::IoError(err)
    }
}

impl ParsedFile {
    pub fn new(path: PathBuf) -> Result<Self, ParsedFileError> {
        let contents = get_file_contents(&path)?;
        let stats = match parser::parse_file(&contents) {
            Ok(("", stats)) => Ok(stats),
            Ok(_) => Err(ParsedFileError::IncompleteParse),
            Err(_) => Err(ParsedFileError::FailedParse),
        }?;
        let mut scorer = scoring::Standard::default();
        let complexity_score = scoring::score(&mut scorer, &stats);

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
