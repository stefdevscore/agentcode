use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub fn walk_project(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .build();

    for result in walker {
        if let Ok(entry) = result {
            if entry
                .file_type()
                .map_or(false, |file_type| file_type.is_file())
            {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    files.sort();
    files
}
