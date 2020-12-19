use crate::*;
use ignore::{DirEntry, WalkBuilder, WalkState};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

pub fn run() {
    let flags = flags::Flags::from_args();

    match flags.cmd {
        Some(flags::Command::InstallConfiguration) => match configuration::install() {
            Some(path) => println!("Installed configuration successfully at {}", path.display()),
            None => println!("Error installing configuration"),
        },
        None => calculate_complexity(flags),
    }
}

fn calculate_complexity(flags: flags::Flags) {
    let mut builder = WalkBuilder::new("./");
    let parsed_value =
        configuration::load_and_parse_config().and_then(|v| IgnoredFilter::from_file(&v).ok());
    let mut files_filter: FilesFilter = parsed_value
        .map(|v| v.into())
        .unwrap_or(FilesFilter::default());

    flags
        .ignore
        .into_iter()
        .for_each(|i| files_filter.ignored_paths.push(i));

    flags
        .only
        .into_iter()
        .for_each(|i| files_filter.only_paths.push(i));

    builder.filter_entry(move |e| files_filter.matches(e.path()));

    let results = Arc::new(Mutex::new(vec![]));
    let scorer = flags.scorer;

    builder.build_parallel().run(|| {
        Box::new(|result| {
            let mut scorer = build_scorer(&scorer);
            if let Some(parsed_file) = parse_dir_entry(&mut scorer, result) {
                let mut results = results.lock().unwrap();
                results.push(parsed_file);
            }

            WalkState::Continue
        })
    });

    let results = results.lock().unwrap();

    match flags.format {
        flags::Format::Standard => render_standard(&results),
        flags::Format::Csv => render_csv(&results),
        flags::Format::Json => render_json(&results),
    }
}

fn build_scorer(algorithm: &flags::ScoringAlgorithm) -> Box<dyn scoring::ScoreVisitor> {
    match algorithm {
        flags::ScoringAlgorithm::Standard => Box::new(scoring::Standard::default()),
        flags::ScoringAlgorithm::Length => Box::new(scoring::Length::default()),
    }
}

fn render_standard(results: &[ParsedFile]) {
    for parsed_file in results {
        println!(
            "{:>8} {}",
            format!("{:.2}", parsed_file.complexity_score),
            parsed_file.path.display()
        );
    }
}

fn render_csv(results: &[ParsedFile]) {
    for parsed_file in results {
        println!(
            "{},{}",
            parsed_file.complexity_score,
            parsed_file.path.display()
        );
    }
}

fn render_json(results: &[ParsedFile]) {
    let mut json = HashMap::new();
    for parsed_file in results {
        json.insert(
            parsed_file.path.display().to_string(),
            parsed_file.complexity_score,
        );
    }

    println!("{}", serde_json::to_string(&json).unwrap());
}

fn parse_dir_entry(
    mut scorer: &mut Box<dyn scoring::ScoreVisitor>,
    result: Result<DirEntry, ignore::Error>,
) -> Option<ParsedFile> {
    result
        .ok()
        .and_then(|entry| ParsedFile::new(&mut scorer, entry.path().to_path_buf()).ok())
}
