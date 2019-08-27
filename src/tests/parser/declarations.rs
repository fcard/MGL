use crate::ast::*;
use crate::error::*;
use crate::tests::utility::*;
use std::str::FromStr;
use std::fmt::Debug;

use ResourceKind::*;

trait Ast = FromStr<Err=MglError> + Debug + PartialEq;
fn assert_parse_declaration<T: Ast>(code: &str, ast: T) {
  assert_eq!(parse_unwrap::<T>(code), ast)
}

#[test]
fn test_declaration_key_name() {
  assert_parse_declaration("x", Key::name("x"))
}

#[test]
fn test_declaration_key_index() {
  assert_parse_declaration("x[0]", Key::indexing("x", expr("0")));
}

#[test]
fn test_declaration_key_dot() {
  let x = Key::name("x");
  let y = Key::name("y");
  let z = Key::name("z");
  assert_parse_declaration("x.y",   Key::dot(x.clone(), y.clone()));
  assert_parse_declaration("x.y.z", Key::dot(x.clone(), Key::dot(y.clone(), z.clone())));
}

#[test]
fn test_declaration_key_mixed() {
  let x = Key::name("x");
  let y = Key::name("y");
  assert_parse_declaration("x[0].y", Key::dot(Key::indexing("x", expr("0")), y.clone()));
  assert_parse_declaration("x.y[0]", Key::dot(x.clone(), Key::indexing("y", expr("0"))));
}

#[test]
fn test_declaration_key_predicate() {
  assert!(key("x").is_name());
  assert!(key("x.y").is_dot());
  assert!(key("x[0]").is_indexing());

  assert!(!key("x").is_dot());
  assert!(!key("x").is_indexing());

  assert!(!key("x.y").is_name());
  assert!(!key("x.y").is_indexing());

  assert!(!key("x[0]").is_name());
  assert!(!key("x[0]").is_dot());
}

#[test]
fn test_declaration_key_methods() {
  assert_eq!(key("x").name_of(),    String::from("x"));
  assert_eq!(key("x[0]").name_of(), String::from("x"));
  assert_eq!(key("x.y").name_of(),  String::from("x"));

  assert_eq!(key("x").index_of(),    None);
  assert_eq!(key("x[0]").index_of(), Some(&expr("0")));

  assert_eq!(key("x").leftmost_index_of(),      None);
  assert_eq!(key("x.y").leftmost_index_of(),    None);
  assert_eq!(key("x[0]").leftmost_index_of(),   Some(&expr("0")));
  assert_eq!(key("x[0].y").leftmost_index_of(), Some(&expr("0")));

  assert_eq!(key("x").left_of(),   None);
  assert_eq!(key("x.y").left_of(), Some(&key("x")));
}

#[test]
fn test_declaration_key_value() {
  assert_parse_declaration("x:   1",      KeyValue::new(key("x"),    expr("1")));
  assert_parse_declaration("x.y: true",   KeyValue::new(key("x.y"),  expr("true")));
  assert_parse_declaration("x[0]: \"k\"", KeyValue::new(key("x[0]"), expr("\"k\"")));
}


#[test]
fn test_declaration_function() {
  let function = |name: &str, args: &[&str], body: &str| {
    Declaration::Function(FunctionDeclaration::new(name, args, statement(body)))
  };

  assert_parse_declaration(
    "function hi() { print(\"hi\")\n print(\"hello\")\n }",
    function("hi", &[], "{ print(\"hi\")\n print(\"hello\")\n }")
  );

  assert_parse_declaration(
    "function double(x) { return 2 * x\n }",
    function("double", &["x"], "{ return 2 * x\n }")
  );

  assert_parse_declaration(
    "function add(a,b) { return a + b\n }",
    function("add", &["a", "b"], "{ return a + b\n }")
  );
}

#[test]
fn test_declaration_instance() {
  let instance = |name: &str, obj: &str, keyvals: &[KeyValue]| {
    Declaration::Instance(InstanceDeclaration::new(expr(obj), name, keyvals))
  };

  assert_parse_declaration(
    "instance inst of obj {}",
    instance("inst", "obj", &keys![])
  );

  assert_parse_declaration(
    "instance keyhaver of key::haver {key: value\n}",
    instance("keyhaver", "key::haver", &keys![key: value])
  );
}

#[test]
fn test_declaration_resource_object_1() {
  assert_parse_declaration(
    "object hello { a: 1\n }",
    resource!(Object, "hello", &keys![a: 1], &[], &[])
  );
}

#[test]
fn test_declaration_resource_object_2() {
  assert_parse_declaration(
    "wrapper world { b: true\n alarm[0]: 100\n }",
    resource!(Wrapper, "world", &keys![b: true, alarm[0]: 100], &[], &[])
  );
}

#[test]
fn test_declaration_resource_object_3() {
  assert_parse_declaration(
    "object methodical {
      field: value + extra

      function method(argument) {
        return argument + self.field
      }
    }",

    resource!(
      Object,
      "methodical",
      &keys![field: value + extra],
      &[function("function method(argument) { return argument + self.field\n }")],
      &[]
    )
  );
}

#[test]
fn test_declaration_resource_misc_1() {
  use ResourceKind::*;

  let ball_sprite = resource!(Sprite, "ball",   &keys![radius: 21, is_bouncy: true], &[], &[]);
  let ball_sound  = resource!(Sound, "boing",   &keys![loops: false, length: 12.5],  &[], &[]);
  let ball_room   = resource!(Room, "ballroom", &keys![has: ball, many: true], &[], &[]);

  assert_parse_declaration("sprite ball { radius: 21\n is_bouncy: true\n }", ball_sprite);
  assert_parse_declaration("sound  boing { loops: false\n length: 12.5\n }", ball_sound);
  assert_parse_declaration("room   ballroom { has: ball\n many: true\n }",   ball_room);
}

#[test]
fn test_declaration_resource_module() {
  let resource_module = |code: &str| {
    let ResourceDeclaration { kind, .. } = code.parse::<ResourceDeclaration>().unwrap();
    kind.module()
  };

  assert_eq!(resource_module("object  a {}"), "object");
  assert_eq!(resource_module("wrapper a {}"), "wrapper");
  assert_eq!(resource_module("sprite  a {}"), "sprite");
  assert_eq!(resource_module("sound   a {}"), "sound");
  assert_eq!(resource_module("room    a {}"), "room");
}

