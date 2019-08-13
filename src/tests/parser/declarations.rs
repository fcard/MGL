use crate::ast::*;
use crate::tests::utility::*;

macro assert_parse_declaration {
  ($code: expr, $ast: expr) => {{
    assert_eq!(declaration($code), $ast)
  }}
}

macro key {
  ($name: ident, $expr: expr) => { KeyValue::new(Key::name(stringify!($name)), expr($expr)) },
  ($name: ident[$($v: expr),+], $expr: expr) => {
    KeyValue::new(Key::indexing(stringify!($name), "", &[$(expr($v)),*]), expr($expr))
  }
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
    Declaration::object("hello", &[key!(a, "1")], &[], false)
  );

  assert_parse_declaration!(
    "wrapper world { b: true\n alarm[0]: 100\n }",
    Declaration::object("world", &[key!(b, "true"), key!(alarm["0"], "100")], &[], true)
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
      &[key!(field, "value + extra")],
      &[function("function method(argument) { return argument + self.field\n }")],
      false,
    )
  );
}

#[test]
fn test_declaration_resource() {
  let ball_sprite = Declaration::resource(
    ResourceKind::Sprite, "ball", &[key!(radius, "21"), key!(is_bouncy, "true")]);

  let ball_sound = Declaration::resource(
    ResourceKind::Sound, "boing", &[key!(loops, "false"), key!(length, "12.5")]);

  let ball_room = Declaration::resource(
    ResourceKind::Room, "ballroom", &[key!(has, "ball"), key!(many, "true")]);

  assert_parse_declaration!("sprite ball { radius: 21\n is_bouncy: true\n }", ball_sprite);
  assert_parse_declaration!("sound  boing { loops: false\n length: 12.5\n }", ball_sound);
  assert_parse_declaration!("room   ballroom { has: ball\n many: true\n }",   ball_room);
}

