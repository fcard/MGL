use pest::Parser;
use pest::iterators;

pub type Pair<'a>  = iterators::Pair<'a, Rule>;
pub type Pairs<'a> = iterators::Pairs<'a, Rule>;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct MglParser;

pub fn parse_to_pairs<'a>(code: &'a str) -> Pairs<'a> {
  match MglParser::parse(Rule::top, code) {
    Ok(top_pairs) => {
      for top_pair in top_pairs {
        match top_pair.as_rule() {
          Rule::top => {
            return top_pair.into_inner()
          }
          _ => ()
        }
      }
      panic!("top rule not found");
    }
    Err(err) => panic!("{}", err)
  }
}
