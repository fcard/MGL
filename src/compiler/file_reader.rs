use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use crate::utility::files::{valid_paths, path_string};
use crate::ast::Top;
use crate::parser::parse_code;
use crate::parser::context::code;


pub fn read_project_ast(project_file: Option<PathBuf>) -> Vec<(PathBuf, Top)> {
  if let Some(project) = project_file {
    read_directory(project.parent().unwrap().to_path_buf())

  } else {
    Vec::new()
  }
}


pub fn read_directory(directory: PathBuf) -> Vec<(PathBuf, Top)>  {
  let mut asts = Vec::new();

  if let Ok(entries) = read_dir(directory) {
    for file in valid_paths(entries) {
      if is_mgl_file(&file) {
        let buf = file.clone().to_path_buf();
        asts.push(read_ast_and_filename(&buf))

      } else if file.is_dir() {
        asts.append(&mut read_directory(file))
      }
    }
  }
  return asts;
}


fn is_mgl_file(file: &PathBuf) -> bool {
  file.is_file() && file.extension().unwrap() == "mgl"
}


pub fn read_ast_and_filename(path: &PathBuf) -> (PathBuf, Top) {
  let ast = read_file_as_ast(path);
  return ((*path).clone(), ast)
}


pub fn read_file_as_ast(path: &PathBuf) -> Top {
  match read_to_string(path) {
    Ok(mgl) => parse_code(code(&mgl)),

    Err(e) => {
      eprintln!("An error has occured while trying to read '{}': {}\n", path_string(&path), e);
      parse_code(code(""))
    }
  }
}

