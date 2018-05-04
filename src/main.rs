extern crate failure;
extern crate walkdir;
mod file;

use walkdir::{ WalkDir, DirEntry };
use std::env;
use std::path::{Path, PathBuf};
use std::fs::metadata;
use failure::{Error, ResultExt};
use file::File;


fn main() {
    let path = collect_paths();
    match path {
        Ok(path_list) => format_paths(path_list),
        Err(_) => panic!()
    }
}

fn format_paths (paths: Vec<PathBuf>) {
    for path in paths {
        let file_entry = File::new(path);
        println!("{}", file_entry.to_string());
    };
}

fn not_hidden_or_opf(entry: &DirEntry) -> bool {
    let not_hidden = !entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false);

    let not_opf = !entry.file_name()
        .to_str()
        .map(|s| s.ends_with(".opf"))
        .unwrap_or(false);

    not_hidden && not_opf
}

fn is_file(path: &Path) -> bool {
    let not_file =
        match metadata(path) {
            Ok(meta) => meta.is_dir(),
            _ => false,
        };

    !not_file
}

fn collect_paths () -> Result<Vec<PathBuf>, Error> {
    let search_path = env::current_dir().context("Cannot access current directory.")?;
    let walker = WalkDir::new(&search_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(not_hidden_or_opf);

    let mut paths = Vec::new();

    for _entry in walker {
        let entry = _entry.context("Invalid entry.")?;
        let path = entry.path().strip_prefix(&search_path).context("Current directory changed.")?;
        if is_file(path) {
            paths.push(path.to_owned());
        }
    };

    Ok(paths)
}
