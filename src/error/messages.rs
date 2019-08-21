use crate::error::enums::*;
use crate::parser::grammar::*;
use titlecase::titlecase;

pub trait ErrorMessageProvider {
  const PARSER_ERROR: &'static str;
  const PARSER_EXPECTED_GRAMMAR_RULES: &'static str;

  fn parser_small_error_message(kind: ParserErrorKind) -> &'static str;
  fn field_type_error(e: MglError) -> String;
  fn field_value_error(e: MglError) -> String;

  fn eprintln(e: MglError) {
    eprintln!("{}", Self::error_message(e));
  }

  fn error_message(e: MglError) -> String {
    match &e {
      MglError::Parser {..} => Self::parser_error_message(e),
      _ => unimplemented!()
    }
  }

  fn parser_error_message(e: MglError) -> String {
    let long;
    let kind;

    if let &MglError::Parser { error_kind, verbose, .. } = &e {
      long = verbose;
      kind = error_kind;

    } else {
      unreachable!()
    }

    if long {
      Self::parser_long_error_message(e)
    } else {
      String::from(Self::parser_small_error_message(kind))
    }
  }

  fn parser_long_error_message(e: MglError) -> String {
    if let MglError::Parser {error_kind, text, line, column, rules, .. } = e {
      let rules = titlecase_rules(&rules);
      let mut truncated_text = String::from(&text);
      truncated_text.truncate(80);


      format!(
"   {parser_error}:
              |
    {line:>9} | {text}{dots}
              | {dash}^
               -----------------------------------------------------------------
                {message}

    {expected_grammar}:
              | {rules}
",
        line = line,
        text = truncated_text,
        dots = if text.len() > 80 { "..." } else { "" },
        dash = "-".repeat(column-1),
        message = Self::parser_small_error_message(error_kind),
        rules = rules.join(", "),
        parser_error = Self::PARSER_ERROR,
        expected_grammar = Self::PARSER_EXPECTED_GRAMMAR_RULES,
      )
    } else {
      unreachable!()
    }
  }
}

fn titlecase_rules(rules: &[Rule]) -> Vec<String> {
  rules.iter().map(|r| titlecase(&format!("{:?}", r).replace("_", " "))).collect()
}

