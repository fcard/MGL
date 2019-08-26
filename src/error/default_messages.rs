use crate::error::*;

pub struct DefaultErrorMessages;

impl ErrorMessageProvider for DefaultErrorMessages {
  const PARSER_ERROR: &'static str = "Parser Error";
  const PARSER_EXPECTED_GRAMMAR_RULES: &'static str = "Expected Grammar Rules";
  const TYPE_ERROR: &'static str = "Type Error";
  const UNKNOWN_FILE: &'static str = "??????????";

  fn parser_small_error_message(kind: ParserErrorKind) -> &'static str {
    use ParserErrorKind::*;

    match kind {
      MissingDeclaration     => "Expected object, function or resource declaration.",
      MissingDeclarationItem => "Expected method or key-value pair.",
      MissingArguments       => "Function is missing its argument names.",
      MissingBody            => "Missing statement body.",
      MissingName            => "Expected a name.",
      IncompleteStatement    => "Incomplete statement: expected a call/assignment, then a newline.",
      UnexpectedCharOrEof    => "Unexpected character or EOF while parsing a statement.",
      Unknown                => "Unexpected character.",
    }
  }

  fn type_small_error_message(ty: String) -> String {
    format!("Expression needed to be of type {}", ty)
  }
}
