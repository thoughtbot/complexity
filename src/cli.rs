use crate::*;
use clap::Parser;
use crossbeam_channel::unbounded;
use ignore::{DirEntry, WalkBuilder, WalkState};
use serde_json;
use std::collections::HashMap;

pub fn run() {
    let flags = flags::Flags::parse();

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
    builder.threads(num_cpus::get());

    let mut results = vec![];
    let scorer = &flags.scorer;

    let (sender, receiver) = unbounded();

    builder.build_parallel().run(|| {
        let sender = sender.clone();

        Box::new(move |result| {
            let mut scorer = build_scorer(&scorer);
            if let Some(parsed_file) = parse_dir_entry(&mut scorer, result) {
                sender.send(parsed_file).unwrap();
            }

            WalkState::Continue
        })
    });

    drop(sender);

    while let Ok(parsed_file) = receiver.recv() {
        results.push(parsed_file);
    }

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
    use std::io::Write;
    let mut lock = std::io::stdout().lock();
    for parsed_file in results {
        writeln!(
            lock,
            "{:>8} {}",
            format!("{:.2}", parsed_file.complexity_score),
            parsed_file.path.display()
        )
        .unwrap();
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
