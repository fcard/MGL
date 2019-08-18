use pest::error::{self, LineColLocation};
use crate::error::*;
use crate::parser::grammar::*;
use crate::parser::context::*;

type Error = error::Error<Rule>;
type ErrorVariant = error::ErrorVariant<Rule>;

pub fn parser_error(c: ParserContext, e: Error) -> MglError {
  use Rule::*;
  use ParserErrorKind::*;

  if let ErrorVariant::ParsingError {positives, ..} = e.variant.clone() {
    if let LineColLocation::Pos((line, column)) = e.line_col.clone() {
      println!("{}", line);
      let text = c.code.split("\n").nth(line-1).unwrap().clone();

      let err = |kind: ParserErrorKind| {
        MglError::Parser {
          error_kind: kind,
          verbose: c.verbose_errors,
          text: String::from(text),
          line,
          column,
          rules: positives.clone()
        }
      };

      if positives.contains(&top) ||
         positives.contains(&object_declaration) {
        return err(MissingDeclaration)

      } else if positives.contains(&function_declaration) {
        return err(MissingDeclarationItem)

      } else if positives == &[Rule::function_arguments_declaration] {
        return err(MissingArguments)

      } else if positives == &[call, indexing, op, ternary_op, ternary_op_call] {
        return err(IncompleteStatement)

      } else if positives == &[body] ||
                positives == &[body, call, indexing, op, ternary_op] {
        return err(MissingBody)

      } else if positives.contains(&return_statement) {
        return err(UnexpectedCharOrEof)

      } else if positives == &[Rule::name] {
        return err(MissingName)

      } else {
        return err(Unknown);
      }
    }
  }
  unreachable!()
}

