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
mod source_files;

#[cfg(test)]
mod tests;

use error::{MglError, ErrorMessageProvider, DefaultErrorMessages};
use resources::script::Script;
use resources::project::{Project, Item};
use command_line::{interpret_arguments, Action, Command};
use compiler::file_reader::*;
use compiler::resource_tree::*;
use compiler::script::*;

fn main() {
  let command = interpret_arguments();

  match command.action {
    Action::Compile => {
      println!("Not functional yet, sorry!");
    }

    Action::Project(pretty) => {
      if let Ok(tree) = read_project(command) {
        if pretty {
          println!("{:#?}", tree);
        } else {
          println!("{:?}", tree);
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

    Action::Scripts => {
      if let Ok(tree) = read_project(command) {
        for script in tree.scripts {
          print_script(script);
        }
      }
    }
  }
}

fn print_script(script: Item<Script>) {
  match script {
    Item::Group(name, items) => {
      println!("[{}]", name);
      for item in items {
        print_script(item);
      }
    }
    Item::File(name, s) => {
      println!("[[{}]]", name);
      println!("{}", build_script(s));
    }
  }
}

fn read_project(command: Command) -> Result<Project, ()> {
  match read_resource_tree(command.project_file) {
    Ok(tree) => {
      Ok(tree)
    }

    Err(errors) => {
      eprintln!("There were errors reading the project!");
      for error in errors {
        DefaultErrorMessages::eprintln(error);
      }
      Err(())
    }
  }
}

