use pest::{self, Parser};
use pest::iterators;
use pest_derive::Parser;

use crate::error::*;
use crate::parser::error::*;
use crate::parser::*;
use crate::parser::tokens::*;

pub type Pair<'a>  = iterators::Pair<'a, Rule>;
pub type Pairs<'a> = iterators::Pairs<'a, Rule>;
pub use pest::Span;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MglParser;

pub fn parse_top<'a, C: Ctx<'a>>(c: C) -> Result<InnerTokens<'a>> {
  Ok(parse_mgl(Rule::top, c.into())?.into_inner())
}

pub fn parse_mgl<'a, C: Ctx<'a>>(rule: Rule, c: C) -> Result<Tokens<'a>> {
  let c = c.into();
  match MglParser::parse(rule, &c.code.clone()) {
    Ok(pairs) => {
      for pair in pairs {
        if pair.as_rule() == rule {
          return Ok(Tokens::new(pair, c.file));
        }
      }
      unreachable!()
    }

    Err(err) => Err(parser_error(c, err))
  }
}
