use crate::*;
use ignore::{DirEntry, WalkBuilder, WalkState};
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

    builder.build_parallel().run(|| {
        Box::new(|result| {
            render_result(result);

            WalkState::Continue
        })
    });
}

fn render_result(result: Result<DirEntry, ignore::Error>) {
    if let Some(parsed_file) = result
        .ok()
        .and_then(|entry| ParsedFile::new(entry.path().to_path_buf()).ok())
    {
        println!(
            "{:>8} {}",
            format!("{:.2}", parsed_file.complexity_score),
            parsed_file.path.display()
        );
    }
}
