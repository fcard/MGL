use std::env;
use std::path::{Path, PathBuf};
use clap::{App, AppSettings, ArgMatches, Arg, SubCommand, crate_version};

pub struct Command {
  pub action: Action,
  pub files: Vec<PathBuf>,
  pub project_file: Option<PathBuf>,
}

pub enum Action {
  Compile,
  ShowAst(bool),
  Project(bool),
  Scripts,
}


fn generate_app<'a, 'b>() -> App<'a, 'b> {
  let pretty = Arg::with_name("pretty")
               .long("pretty")
               .takes_value(true)
               .value_name("value")
               .possible_values(&["yes", "true", "no", "false"])
               .help("Pretty printing");

  App::new("MGL")
    .version(&crate_version!()[..])
    .about("Compiles code into a Game Maker project")
    .setting(AppSettings::GlobalVersion)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::ArgRequiredElseHelp)

    .arg(Arg::with_name("no-project")
         .short("n")
         .long("no-project")
         .takes_value(false)
         .conflicts_with("project-file")
         .help("Don't read any project"))

    .arg(Arg::with_name("project-file")
         .short("p")
         .long("project-file")
         .value_name("FILE")
         .takes_value(true)
         .help("Select project file"))

    .arg(Arg::with_name("input")
         .short("i")
         .long("input")
         .value_name("FILE(,FILE)*")
         .takes_value(true)
         .help("Select additional files as input"))


    .subcommand(SubCommand::with_name("compile")
                .about("Compile current project (still non-functional)"))

    .subcommand(SubCommand::with_name("ast")
                .about("Show AST for all input files")
                .arg(pretty.clone()))

    .subcommand(SubCommand::with_name("project")
                .about("Show final project output as text")
                .arg(pretty))

    .subcommand(SubCommand::with_name("scripts")
                .about("compile just the scripts and print them"))

}


pub fn interpret_arguments() -> Command {
  let matches = generate_app().get_matches();

  Command {
    project_file: interpret_project_argument(&matches),
    files:        interpret_input_argument(&matches),
    action:       interpret_subcommand(&matches),
  }
}


fn interpret_subcommand(matches: &ArgMatches) -> Action {
  match matches.subcommand() {
    ("compile", _) => Action::Compile,
    ("project", m) => Action::Project(interpret_pretty(&m.unwrap())),
    ("ast",     m) => Action::ShowAst(interpret_pretty(&m.unwrap())),
    ("scripts", _) => Action::Scripts,
    _ => unreachable!()
  }
}

fn interpret_pretty(matches: &ArgMatches) -> bool {
  match matches.value_of("pretty").unwrap_or("yes") {
    "yes" | "true"  => true,
    "no"  | "false" => false,
    _     => unreachable!()
  }
}


fn interpret_input_argument(matches: &ArgMatches) -> Vec<PathBuf> {
  let mut input = Vec::new();

  if let Some(files) = matches.value_of("input") {
    for file in files.split(",") {
      input.push(Path::new(file).to_path_buf());
    }
  }

  return input;
}


fn interpret_project_argument(matches: &ArgMatches) -> Option<PathBuf> {
  if matches.is_present("no-project") {
    return None
  }

  match matches.value_of("project-file") {
    None => find_project_file(),

    Some(file) => {
      let path = Path::new(file).to_path_buf();

      if path.exists() {
        Some(path)

      } else {
        eprintln!("NOTE: The given project file was not found, looking for one...\n");
        find_project_file()
      }
    }
  }
}


fn find_project_file() -> Option<PathBuf> {
  if let Ok(mut directory) = env::current_dir() {
    if let Some(path) = find_project_file_in(&directory) {
      return Some(path)

    } else {
      while let Some(parent) = directory.parent() {
        directory = parent.to_path_buf();
        if let Some(path) = find_project_file_in(&directory) {
          return Some(path)
        }
      }
      return None
    }
  } else {
    return None
  }
}


fn find_project_file_in(directory: &PathBuf) -> Option<PathBuf> {
  let mut path = directory.clone();
  path.push("Project.mgl.toml");

  if path.exists() {
    return Some(path)
  } else {
    return None
  }
}

