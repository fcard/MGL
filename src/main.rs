#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]

mod utility;
mod parser;
mod ast;
mod resources;
mod command_line;
mod compiler;

#[cfg(test)]
mod tests;

use utility::files::*;
use command_line::{interpret_arguments, Action};
use compiler::file_reader::*;

fn main() {
  let command = interpret_arguments();

  match command.action {
    Action::Compile => {
      println!("Not functional yet, sorry!");
    }

    Action::ShowAst(pretty) => {
      let mut asts = read_project_ast(command.project_file);
      asts.append(&mut command.files.iter().map(read_ast_and_filename).collect());

      for (path, ast) in asts {
        println!("[{}]", path_string(&path));
        if pretty {
          println!("{:#?}\n", ast);
        } else {
          println!("{:?}\n", ast);
        }
      }
    }
  }
}
