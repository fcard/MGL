use crate::ast::*;
use crate::tests::utility::*;

macro assert_parse_declaration {
  ($code: expr, $ast: expr) => {{
    assert_eq!(declaration($code), $ast)
  }}
}

#[test]
fn test_declaration_function() {
  assert_parse_declaration!(
    "function double(x) { return 2 * x\n }",
    Declaration::function("double", &["x"], statement("{ return 2 * x\n }"))
  );

  assert_parse_declaration!(
    "function add(a,b) { return a + b\n }",
    Declaration::function("add", &["a", "b"], statement("{ return a + b\n }"))
  );
}

#[test]
fn test_declaration_object() {
  assert_parse_declaration!(
    "object hello { a: 1\n }",
    Declaration::object(
      "hello",
      &[KeyValue::new(Key::name("a"), expr("1"))],
      &[]
    )
  );

  assert_parse_declaration!(
    "object world { b: true\n alarm[0]: 100\n }",
    Declaration::object(
      "world",
      &[KeyValue::new(Key::name("b"), expr("true")),
        KeyValue::new(Key::indexing("alarm", "", &[expr("0")]), expr("100"))],
      &[]
    )
  );

  assert_parse_declaration!(
    "object methodical {
      field: value + extra

      function method(argument) {
        return argument + self.field
      }
    }",

    Declaration::object(
      "methodical",
      &[KeyValue::new(Key::name("field"), expr("value + extra"))],
      &[function("function method(argument) { return argument + self.field\n }")]
    )
  );
}

