#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use complexity::*;
use ignore::{DirEntry, WalkBuilder, WalkState};

fn main() {
    let mut builder = WalkBuilder::new("./");
    builder.filter_entry(|e| FilesFilter::default().matches(e));

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
