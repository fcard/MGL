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

  InvalidField {
    kind: InvalidFieldKind,
    field: String
  },

  ConvertExpression {
    value: IExpr,
    into_type: String,
  },

  NotResource,

  UnknownOption {
    value: String,
    options: Vec<String>,
  },

  Event {
    kind: EventErrorKind,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventErrorKind {
  InvalidName,
  InvalidIndexType(String),
  Dot,
  UnknownStepKind,
  UnknownAlarmKind,
  UnknownKeyCode,
  UnknownMouseKind,
  UnknownOtherKind,
  UnknownDrawKind,
}

impl MglError {
  pub fn event<T>(kind: EventErrorKind) -> Result<T> {
    Err(MglError::Event { kind })
  }

  pub fn invalid_field<T>(field: &str, kind: InvalidFieldKind) -> Result<T> {
    Err(
      MglError::InvalidField {
        kind,
        field: String::from(field)
      }
    )
  }

  pub fn convert_expression<T>(value: IExpr, into_type: &str) -> Result<T> {
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
