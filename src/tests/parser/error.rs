use crate::parser::grammar::*;
use crate::parser::context::*;
use crate::parser::*;

macro v {
  ($code: expr) => {code($code).verbose_errors(false)}
}

#[test]
#[should_panic]
fn test_parser_error_wrong_rule() {
  parse_mgl(Rule::expression, "");
}

#[test]
#[should_panic]
fn test_parser_error_message_any() {
  parse_code("a");
}

#[test]
#[should_panic(expected = "Expected object, function or resource declaration.")]
fn test_parser_error_message_1() {
  parse_code(v!("a"));
}

#[test]
#[should_panic(expected = "Expected method or key-value pair.")]
fn test_parser_error_message_2() {
  parse_code(v!("object a {,}"));
}

#[test]
#[should_panic(expected = "Function is missing its argument names.")]
fn test_parser_error_message_3() {
  parse_code(v!("object a {function f {}\n}"));
}

#[test]
#[should_panic(expected = "Missing statement body.")]
fn test_parser_error_message_4() {
  parse_code(v!("object a {function f ()\n}"));
}

#[test]
#[should_panic(expected = "Incomplete statement: must be a call or assignment, ended by a newline.")]
fn test_parser_error_message_5() {
  parse_code(v!("object a {function f () {a\n}\n}"));
}

#[test]
#[should_panic(expected = "Unexpected character or EOF while parsing a statement.")]
fn test_parser_error_message_6() {
  parse_code(v!("object a { function f() {"));
}

#[test]
#[should_panic(expected = "Expected a name.")]
fn test_parser_error_message_7() {
  parse_code(v!("object {}"));
}

#[test]
#[should_panic(expected = "Unexpected character.")]
fn test_parser_error_message_default() {
  parse_code(v!("object a {a: b %* c}"));
}

