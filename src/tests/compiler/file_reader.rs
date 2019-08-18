use std::path::PathBuf;
use crate::compiler::file_reader::*;
use crate::compiler::resource_tree::*;

#[test]
fn test_file_reader_ast_examples() {
  // test if parsing is successful
  read_file_as_ast(&PathBuf::from("examples/hello_world.mgl")).unwrap();
  read_resource_tree(Some(PathBuf::from("examples/project/Project.mgl.toml"))).unwrap();
}

