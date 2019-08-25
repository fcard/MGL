use std::path::PathBuf;
use crate::source_files::*;

#[test]
fn test_source_files() {
  let hello = || PathBuf::from("hello.mgl");

  assert_eq!(
    SourceFile::new(hello()),
    SourceFile::new(hello()),
  );

  assert_eq!(
    format!("{:?}", SourceFile::new(hello())),
    format!("File({:?})", hello()),
  );

  assert_eq!(
    format!("{:?}", SourceFile::None),
    String::from("None"),
  );
}

