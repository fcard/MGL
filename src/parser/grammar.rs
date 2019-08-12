use pest::Parser;
use pest::iterators;

pub type Pair<'a>  = iterators::Pair<'a, Rule>;
pub type Pairs<'a> = iterators::Pairs<'a, Rule>;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MglParser;

pub fn parse_top<'a>(code: &'a str) -> Pairs<'a> {
  parse_mgl(Rule::top, code).into_inner()
}

pub fn parse_mgl<'a>(rule: Rule, code: &'a str) -> Pair<'a> {
  match MglParser::parse(rule, code) {
    Ok(pairs) => {
      for pair in pairs {
        if pair.as_rule() == rule {
          return pair;
        }
      }
      panic!("rule '{:?}' not found", rule)
    }

    Err(err) => panic!("{}", err)
  }
}
