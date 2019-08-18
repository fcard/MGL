use crate::error::*;

pub struct DefaultErrorMessages;

impl ErrorMessageProvider for DefaultErrorMessages {
  const PARSER_ERROR: &'static str = "Parser Error";
  const PARSER_EXPECTED_GRAMMAR_RULES: &'static str = "Expected Grammar Rules";

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

  fn field_type_error(e: MglError) -> String {
    if let MglError::Field { field_name, value_type, .. } = e {
      format!(
        "Invalid value for the field {}: must be a {}",
        field_name,
        value_type,
      )

    } else {
      unreachable!()
    }
  }

  fn field_value_error(e: MglError) -> String {
    if let MglError::Field { field_name, .. } = e {
      format!(
        "Invalid value for the field {}: unknown value",
        field_name,
      )

    } else {
      unreachable!()
    }
  }
}
