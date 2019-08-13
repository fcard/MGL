use std::path::PathBuf;
use crate::compiler::file_reader::*;

#[test]
fn test_file_reader_ast_examples() {
  // test if parsing is successful
  read_file_as_ast(&PathBuf::from("examples/hello_world.mgl"));
  read_project_ast(Some(PathBuf::from("examples/project/Project.mgl.toml")));
}

