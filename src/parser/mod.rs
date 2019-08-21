//! Holds the functions that convert code to ASTs.

pub mod context;
pub mod grammar;
pub mod expressions;
pub mod statements;
pub mod declarations;
pub mod error;

use crate::ast::Top;
use crate::error::*;

use grammar::parse_top;
use context::ParserContext;
use declarations::parse_declaration;

pub trait Ctx<'a> = Into<ParserContext<'a>>;

pub fn parse_code<'a, C: Ctx<'a>>(c: C) -> Result<Top> {
  let top_expressions = parse_top(c)?;
  let mut declarations = Vec::new();

  for top_expression in top_expressions {
    if let Some(declaration) = parse_declaration(top_expression) {
      declarations.push(declaration);
    }
  }

  Ok(Top::new(&declarations))
}

