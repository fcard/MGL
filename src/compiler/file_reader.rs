use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::ffi::OsStr;

use crate::utility::files::{valid_paths, path_string, path_file_name};
use crate::ast::Top;
use crate::error::*;
use crate::parser::parse_code;

#[derive(Debug, Clone)]
pub enum AstFileTree {
  Root(Box<AstFileTree>),
  Node(String, Vec<AstFileTree>),
  Leaf(String, Top)
}

pub fn read_file_as_ast(path: &PathBuf) -> Result<Top> {
  match read_to_string(path) {
    Ok(mgl) => parse_code(&*mgl),

    Err(e) => {
      eprintln!("An error has occured while trying to read '{}': {}\n", path_string(&path), e);
      parse_code("")
    }
  }
}

fn is_mgl_file(file: &PathBuf) -> bool {
  file.is_file() && file.extension().unwrap_or(OsStr::new("")) == "mgl"
}

impl AstFileTree {
  pub fn from_project(project_file: Option<PathBuf>) -> TopResult<AstFileTree> {
    Ok(
      AstFileTree::Root(box
        if let Some(project) = project_file {
          let mut source_directory = project.parent().unwrap().to_path_buf();
          source_directory.push("src");
          AstFileTree::from_directory(&source_directory)?

        } else {
          AstFileTree::Node(String::new(), Vec::new())
        },
      )
    )
  }


  pub fn from_directory(directory: &PathBuf) -> TopResult<AstFileTree>  {
    let mut asts = Vec::new();
    let mut errors = Vec::new();
    let directory_name = path_file_name(directory);

    if let Ok(entries) = read_dir(directory) {
      for file in valid_paths(entries) {
        if is_mgl_file(&file) {
          match AstFileTree::from_file(&file) {
            Ok(ast) => asts.push(ast),
            Err(e)  => errors.push(e),
          }

        } else if file.is_dir() {
          match AstFileTree::from_directory(&file) {
            Ok(ast)    => asts.push(ast),
            Err(mut e) => errors.append(&mut e),
          }
        }
      }
    }
    if errors.is_empty() {
      Ok(AstFileTree::Node(directory_name, asts))
    } else {
      Err(errors)
    }
  }


  pub fn from_file(path: &PathBuf) -> Result<AstFileTree> {
    let name = path_file_name(path);
    let ast = read_file_as_ast(path)?;
    Ok(AstFileTree::Leaf(name, ast))
  }

  pub fn is_root(&self) -> bool {
    match &self {
      &AstFileTree::Root(_) => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn children(&self) -> Vec<AstFileTree> {
    match &self {
      &AstFileTree::Root(box subtree) => subtree.children(),
      &AstFileTree::Node(_, children) => children.clone(),
      &AstFileTree::Leaf(_, _) => vec![]
    }
  }

  pub fn print(&self, pretty: bool) {
    self.print_from_path(&PathBuf::new(), pretty);
  }

  pub fn print_from_path(&self, path: &PathBuf, pretty: bool) {
    let mut new_path = path.clone();
    match &self {
      &AstFileTree::Root(box subtree) => {
        subtree.print(pretty);
      }

      &AstFileTree::Node(directory, subtrees) => {
        new_path.push(directory);

        for subtree in subtrees {
          subtree.print_from_path(&new_path, pretty);
        }
      }

      &AstFileTree::Leaf(file_name, ast) => {
        new_path.push(file_name);

        println!("[{}]", path_string(&new_path));
        if pretty {
          println!("{:#?}\n", ast);
        } else {
          println!("{:?}\n", ast);
        }
      }
    }
  }
}

