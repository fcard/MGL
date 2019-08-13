use std::path::PathBuf;
use std::fs::read_dir;
use std::str::from_utf8_unchecked;
use crate::utility::files::*;

#[test]
fn test_utility_files_path_string() {
  unsafe {
    let p1 = PathBuf::from("");
    let p2 = PathBuf::from("a");
    let p3 = PathBuf::from(from_utf8_unchecked(&[128u8]));

    assert_eq!(path_string(&p1), String::from(""));
    assert_eq!(path_string(&p2), String::from("a"));
    assert_eq!(path_string(&p3), String::from(FILE_NAME_DEFAULT));
  }
}

pub fn test_utility_files_valid_paths() {
  let mut counter = 0;
  for path in valid_paths(read_dir("examples/project").unwrap()) {
    counter += 1;
  }
  assert_eq!(counter, 2);
}
