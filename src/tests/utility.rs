use crate::ast::*;
use crate::parser::*;
use crate::parser::grammar::*;
use crate::parser::expressions::*;
use crate::parser::statements::*;

pub trait CloneAll<T> {
  fn clone_all(&self) -> T;
}

impl<T: Clone> CloneAll<Vec<T>> for Vec<&T> {
  fn clone_all(&self) -> Vec<T> {
    self.iter().map(|x| (*x).clone()).collect()
  }
}

pub fn expr(code: &str) -> Expression {
  parse_expression(parse_mgl(Rule::expression, code))
}

pub fn statement(code: &str) -> Statement {
  let pair = parse_mgl(Rule::statement_non_silent, code);
  let stat = pair.into_inner().next().unwrap();
  return parse_statement(stat)
}

pub fn declaration(code: &str) -> Declaration {
  let Top { declarations } = parse_code(code);
  return (*declarations.iter().next().unwrap()).clone();
}

pub fn function(code: &str) -> FunctionDeclaration {
  if let Declaration::Function(func) = declaration(code) {
    return func
  } else {
    panic!("Not a function declaration!\n '{}'", code)
  }
}
