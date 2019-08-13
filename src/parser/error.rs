use titlecase::titlecase;
use pest::error::{self, LineColLocation};
use crate::parser::grammar::*;
use crate::parser::context::*;

type Error = error::Error<Rule>;
type ErrorVariant = error::ErrorVariant<Rule>;

pub fn error_message(c: ParserContext, e: Error) -> String {
  use Rule::*;

  if let ErrorVariant::ParsingError {positives, ..} = e.variant.clone() {
    if let LineColLocation::Pos((line, column)) = e.line_col.clone() {
      println!("{}", line);
      let text = c.code.split("\n").nth(line-1).unwrap().clone();

      let err = |msg: &str| {
        if c.verbose_errors {
          error_message_with_info(msg, &text, line, column, &positives)
        } else {
          String::from(msg)
        }
      };

      if positives.contains(&top) ||
         positives.contains(&object_declaration) {
        return err("Expected object, function or resource declaration.")

      } else if positives.contains(&function_declaration) {
        return err("Expected method or key-value pair.")

      } else if positives == &[Rule::function_arguments_declaration] {
        return err("Function is missing its argument names.")

      } else if positives == &[call, indexing, op, ternary_op, ternary_op_call] {
        return err("Incomplete statement: must be a call or assignment, ended by a newline.")

      } else if positives == &[body] ||
                positives == &[body, call, indexing, op, ternary_op] {
        return err("Missing statement body.")

      } else if positives.contains(&return_statement) {
        return err("Unexpected character or EOF while parsing a statement.")

      } else if positives == &[Rule::name] {
        return err("Expected a name.")

      } else {
        return err("Unexpected character.");
      }
    }
  }
  unreachable!()
}

fn error_message_with_info(msg: &str, text: &str, line: usize, column: usize, p: &[Rule]) -> String {
  let mut truncated_text = String::from(text);
  truncated_text.truncate(80);

  let rules: Vec<_> = p.iter().map(|r| titlecase(&format!("{:?}", r).replace("_", " "))).collect();

  format!("
   Parse Error:
              |
    {line:>9} | {text}{dots}
              | {dash}^
              -----------------------------------------------------------------
                {message}

   Expected Grammar Rules:
                         | {rules}

    ",
    line = line,
    text = truncated_text,
    dots = if text.len() > 77 { "..." } else { "" },
    dash = "-".repeat(column-1),
    message = msg,
    rules = rules.join(", ")
  )
}
