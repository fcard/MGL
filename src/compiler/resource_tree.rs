use std::path::PathBuf;

use crate::error::*;
use crate::compiler::file_reader::*;
use crate::resources::project::{Project, Module};

pub fn read_resource_tree(project_file: Option<PathBuf>) -> TopResult<Project> {
  let file_tree = AstFileTree::from_project(project_file)?;
  Project::from_ast_file_tree(file_tree, Module::new())
}

