use std::fs::ReadDir;
use std::path::{Path, PathBuf};

pub fn valid_paths(entries: ReadDir) -> impl Iterator<Item = PathBuf> {
  entries.filter_map(Result::ok).map(|entry| entry.path())
}

pub fn path_string(p: &Path) -> String {
  String::from(p.to_str().unwrap_or("{Cannot read this file's name}"))
}
