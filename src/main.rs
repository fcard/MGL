#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![feature(trait_alias)]

mod utility;
mod error;
mod parser;
mod ast;
mod event;
mod resources;
mod command_line;
mod compiler;

#[cfg(test)]
mod tests;

use error::*;
use command_line::{interpret_arguments, Action};
use compiler::file_reader::*;
use compiler::resource_tree::*;

fn main() {
  let command = interpret_arguments();

  match command.action {
    Action::Compile => {
      println!("Not functional yet, sorry!");
    }

    Action::Project(pretty) => {
      match read_resource_tree(command.project_file) {
        Ok(tree) => {
          if pretty {
            println!("{:#?}", tree);
          } else {
            println!("{:?}", tree);
          }
        }

        Err(errors) => {
          eprintln!("There were errors reading the project!");
          for error in errors {
            DefaultErrorMessages::eprintln(error);
          }
        }
      }
    }

    Action::ShowAst(pretty) => {
      (|| {
        let asts  = AstFileTree::from_project(command.project_file)?;
        let other = command.files.iter().map(AstFileTree::from_file);
        asts.print(pretty);
        other.for_each(|ast| ast.unwrap().print(pretty));
        Ok(())

      })().err().map(|es: Vec<MglError>| for e in es { DefaultErrorMessages::eprintln(e) });
    }
  }
}

