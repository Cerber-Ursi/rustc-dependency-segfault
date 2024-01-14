use std::path::PathBuf;
#[derive(Debug)]
struct Task {
    target: PathBuf,
    relative_to: PathBuf,
    source_roots: Vec<PathBuf>,
}
