use crate::parser::grammar::*;
use crate::parser::*;
use crate::error::*;

use ParserErrorKind::*;

fn test_parser_error(code: &str, err: ParserErrorKind) {
  if let Err(MglError::Parser { error_kind, .. }) = parse_code(code) {
    assert_eq!(error_kind, err)

  } else {
    panic!("Not a parser error")
  }
}

#[test]
#[should_panic]
fn test_parser_error_wrong_rule() {
  parse_mgl(Rule::expression, "").unwrap();
}

#[test]
#[should_panic]
fn test_parser_error_any() {
  parse_code("a").unwrap();
}

#[test]
fn test_parser_error_missing_declaration() {
  test_parser_error("a", MissingDeclaration);
}

#[test]
fn test_parser_error_missing_declaration_item() {
  test_parser_error("object a {,}", MissingDeclarationItem);
}

#[test]
fn test_parser_error_missing_arguments() {
  test_parser_error("object a {function f {}\n}", MissingArguments);
}

#[test]
fn test_parser_error_missing_body() {
  test_parser_error("object a {function f ()\n}", MissingBody);
}

#[test]
fn test_parser_error_incomplete_statement() {
  test_parser_error("object a {function f () {a\n}\n}", IncompleteStatement);
}

#[test]
fn test_parser_error_unexpected_eof() {
  test_parser_error("object a { function f() {", UnexpectedCharOrEof);
}

#[test]
fn test_parser_error_missing_name() {
  test_parser_error("object {}", MissingName);
}

#[test]
fn test_parser_error_unknown() {
  test_parser_error("object a {a: b %* c}", Unknown);
}

