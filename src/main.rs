use ignore::Walk;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
struct ParsedFile {
    path: PathBuf,
    body: String,
}

fn main() {
    println!("hello world");

    Walk::new("./")
        .filter_map(|result| {
            result
                .ok()
                .and_then(|entry| get_file_contents(entry.path().to_path_buf()).ok())
        })
        .for_each(|v| println!("{:?}", v));
}

fn get_file_contents(path: PathBuf) -> std::io::Result<ParsedFile> {
    let file = File::open(&path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(ParsedFile {
        path,
        body: contents,
    })
}
