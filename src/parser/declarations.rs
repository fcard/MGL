use crate::ast::*;
use crate::parser::grammar::*;
use crate::parser::statements::*;
use crate::parser::expressions::*;

pub fn parse_declaration(pair: Pair) -> Option<Declaration> {
  match pair.as_rule() {
    Rule::function_declaration => {
      Some(parse_function(pair))
    }

    Rule::object_declaration => {
      Some(parse_object(pair, false))
    }

    Rule::wrapper_declaration => {
      Some(parse_object(pair, true))
    }

    Rule::resource_declaration => {
      Some(parse_resource(pair))
    }

    _ => return None
  }
}

pub fn parse_function(pair: Pair) -> Declaration {
  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();
  let args = parts.next().unwrap().into_inner().map(|p| p.as_str()).collect::<Vec<_>>();
  let body = parts.next().unwrap();

  Declaration::function(name, &args, parse_body(body))
}


pub fn parse_object(pair: Pair, wrapper: bool) -> Declaration {
  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();
  let mut methods = Vec::new();
  let mut keyvals = Vec::new();

  for item in parts {
    match item.as_rule() {
      Rule::function_declaration => {
        if let Declaration::Function(function) = parse_function(item) {
          methods.push(function)
        } else {
          unreachable!()
        }
      }

      Rule::key_value => {
        keyvals.push(parse_key_value(item));
      }

      _ => unreachable!()
    }
  }

  Declaration::object(name, &keyvals, &methods, wrapper)
}


pub fn parse_resource(pair: Pair) -> Declaration {
  let sub = pair.into_inner().next().unwrap();
  let rule = sub.as_rule();
  let mut sub_parts = sub.into_inner();
  let name = sub_parts.next().unwrap().as_str();
  let keys = sub_parts.map(parse_key_value).collect::<Vec<_>>();
  let kind = match rule {
    Rule::sprite_declaration => ResourceKind::Sprite,
    Rule::sound_declaration  => ResourceKind::Sound,
    Rule::room_declaration   => ResourceKind::Room,
    _ => unreachable!()
  };
  Declaration::resource(kind, name, &keys)
}


pub fn parse_key_value(pair: Pair) -> KeyValue {
  let mut parts = pair.into_inner();
  let key   = parse_key(parts.next().unwrap());
  let value = parse_expression(parts.next().unwrap());
  KeyValue::new(key, value)
}


pub fn parse_key(pair: Pair) -> Key {
  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();

  match parts.next() {
    None => Key::name(name),

    Some(index) => {
      let mut index_parts = index.into_inner();
      let acc = index_parts.next().unwrap().as_str();
      let args = index_parts.map(parse_expression).collect::<Vec<_>>();

      Key::indexing(name, acc, &args)
    }
  }
}


