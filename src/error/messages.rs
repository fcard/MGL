use std::fs;
use crate::ast::*;
use crate::error::enums::*;
use crate::parser::grammar::*;
use titlecase::titlecase;

pub trait ErrorMessageProvider {

  // Required implementations

  const PARSER_ERROR: &'static str;
  const PARSER_EXPECTED_GRAMMAR_RULES: &'static str;

  const TYPE_ERROR: &'static str;
  const UNKNOWN_FILE: &'static str;

  fn parser_small_error_message(kind: ParserErrorKind) -> &'static str;
  fn type_small_error_message(ty: String) -> String;


  // Provided methods

  fn eprintln(e: MglError) {
    eprintln!("{}", Self::error_message(e));
  }

  fn error_message(e: MglError) -> String {
    match &e {
      MglError::Parser {..}            => Self::parser_error_message(e),
      MglError::ConvertExpression {..} => Self::type_error_message(e),
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

  fn show_ast_location<T>(ast: &AstDebugInfo<T>) -> Option<String> {
    let &AstDebugInfo { file, column_start, column_end, line_start, line_end, .. } = ast;
    file.as_path().map(|path| {
      let mut result = String::new();
      let string = fs::read_to_string(&path).unwrap_or(String::from(Self::UNKNOWN_FILE));
      let lines  = string.lines()
                         .enumerate()
                         .take(line_end)
                         .skip(line_start-1);

      result.push_str(&format!("      [{}]\n", path.to_str().unwrap_or(Self::UNKNOWN_FILE)));

      for (line, text) in lines {
        result.push_str("      |\n");
        result.push_str(&format!(" {:>4} | ", line+1));
        result.push_str(&text);
        result.push('\n');
        result.push_str("      | ");

        let start = if line + 1 == line_start { column_start - 1 } else { 0 };
        let end   = if line + 1 == line_end   { column_end   - 1 } else { text.len() };

        result.push_str(&format!("{spaces}{hats}\n",
                                 spaces=" ".repeat(start),
                                 hats="^".repeat(end - start)));
      }
      result
    })
  }


  fn type_error_message(e: MglError) -> String {
    if let MglError::ConvertExpression  { value, into_type } = e {
      let mut result = String::new();
      result.push_str(&format!("{}:\n", Self::TYPE_ERROR));
      result.push_str(&Self::show_ast_location(&value)
                      .unwrap_or(String::from(Self::UNKNOWN_FILE)));

      result.push_str(&Self::type_small_error_message(into_type));
      result.push('\n');
      result

    } else {
      unreachable!()
    }
  }
}

fn titlecase_rules(rules: &[Rule]) -> Vec<String> {
  rules.iter().map(|r| titlecase(&format!("{:?}", r).replace("_", " "))).collect()
}

