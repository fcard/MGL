pub mod grammar;
pub mod expressions;
pub mod statements;
pub mod declarations;

use crate::ast::Top;

use grammar::parse_to_pairs;
use declarations::parse_declaration;

pub fn parse_code(code: &str) -> Top {
  let top_expressions = parse_to_pairs(code);
  let mut declarations = Vec::new();

  for top_expression in top_expressions {
    if let Some(declaration) = parse_declaration(top_expression) {
      declarations.push(declaration);
    }
  }

  Top::new(&declarations)
}

