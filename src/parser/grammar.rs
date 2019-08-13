use pest::Parser;
use pest::iterators;
use pest_derive::Parser;

use crate::parser::error::*;
use crate::parser::*;

pub type Pair<'a>  = iterators::Pair<'a, Rule>;
pub type Pairs<'a> = iterators::Pairs<'a, Rule>;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MglParser;

pub fn parse_top<'a, C: Ctx<'a>>(c: C) -> Pairs<'a> {
  parse_mgl(Rule::top, c.into()).into_inner()
}

pub fn parse_mgl<'a, C: Ctx<'a>>(rule: Rule, c: C) -> Pair<'a> {
  let c = c.into();
  match MglParser::parse(rule, &c.code.clone()) {
    Ok(pairs) => {
      for pair in pairs {
        if pair.as_rule() == rule {
          return pair;
        }
      }
      unreachable!()
    }

    Err(err) => panic!("{}", error_message(c, err))
  }
}
