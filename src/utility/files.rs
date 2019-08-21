use std::fs::ReadDir;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub const FILE_NAME_DEFAULT: &str = "{Cannot read this file's name}";

pub fn valid_paths(entries: ReadDir) -> impl Iterator<Item = PathBuf> {
  entries.filter_map(Result::ok).map(|entry| entry.path())
}

pub fn path_string(p: &Path) -> String {
  String::from(p.to_str().unwrap_or(FILE_NAME_DEFAULT))
}

pub fn path_file_name(p: &Path) -> String {
  String::from(
    p.file_name()
     .unwrap_or(OsStr::new(""))
     .to_str()
     .unwrap_or("")
  )
}
