use crate::error::Result;
use crate::ast::*;
use crate::parser::grammar::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MglError {
  Parser {
    error_kind: ParserErrorKind,
    verbose: bool,
    text: String,
    line: usize,
    column: usize,
    rules: Vec<Rule>,
  },

  Field {
    value: Expression,
    field_name: String,
    value_type: String,
  },

  InvalidField {
    kind: InvalidFieldKind,
    field: String
  },

  ConvertExpression {
    value: Expression,
    into_type: String,
  },

  NotResource,

  UnknownOption {
    value: String,
    options: Vec<String>,
  },

  EventInvalidIndex {
    message: String,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserErrorKind {
  MissingDeclaration,
  MissingDeclarationItem,
  MissingArguments,
  MissingBody,
  MissingName,
  IncompleteStatement,
  UnexpectedCharOrEof,
  Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidFieldKind {
  NotFound,
  NotSimple(Key),
  NotSubResource(Key),
  NotArray(Key),
}

impl MglError {
  pub fn invalid_field<T>(field: &str, kind: InvalidFieldKind) -> Result<T> {
    Err(
      MglError::InvalidField {
        kind,
        field: String::from(field)
      }
    )
  }

  pub fn convert_expression<T>(value: Expression, into_type: &str) -> Result<T> {
    Err(
      MglError::ConvertExpression {
        value,
        into_type: String::from(into_type),
      }
    )
  }

  pub fn unknown_option<T>(value: &str, options: &[&str]) -> Result<T> {
    Err(
      MglError::UnknownOption {
        value: String::from(value),
        options: options.iter().map(|x| String::from(*x)).collect(),
      }
    )
  }
}
