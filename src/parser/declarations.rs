use crate::ast::*;
use crate::parser::grammar::*;
use crate::parser::statements::*;
use crate::parser::expressions::*;
use ResourceKind::*;

pub fn parse_declaration(pair: Pair) -> Option<Declaration> {
  match pair.as_rule() {
    Rule::function_declaration => {
      Some(Declaration::Function(parse_function(pair)))
    }

    Rule::instance_declaration => {
      Some(Declaration::Instance(parse_instance(pair)))
    }

    Rule::object_declaration => {
      Some(Declaration::Resource(parse_resource(pair, Object)))
    }

    Rule::wrapper_declaration => {
      Some(Declaration::Resource(parse_resource(pair, Wrapper)))
    }

    Rule::room_declaration => {
      Some(Declaration::Resource(parse_resource(pair, Room)))
    }

    Rule::sound_declaration => {
      Some(Declaration::Resource(parse_resource(pair, Sound)))
    }

    Rule::sprite_declaration => {
      Some(Declaration::Resource(parse_resource(pair, Sprite)))
    }

    _ => return None
  }
}

pub fn parse_function(pair: Pair) -> FunctionDeclaration {
  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();
  let args = parts.next().unwrap().into_inner().map(|p| p.as_str()).collect::<Vec<_>>();
  let body = parts.next().unwrap();

  FunctionDeclaration::new(name, &args, parse_statement(body))
}


pub fn parse_instance(pair: Pair) -> InstanceDeclaration {
  let mut parts = pair.into_inner();
  let name    = parts.next().unwrap().as_str();
  let object  = parse_expression(parts.next().unwrap());
  let keyvals = parts.map(parse_key_value).collect::<Vec<_>>();

  InstanceDeclaration::new(object, name, &keyvals)
}


pub fn parse_resource(pair: Pair, kind: ResourceKind) -> ResourceDeclaration {
  let mut methods   = Vec::new();
  let mut keyvalues = Vec::new();
  let mut instances = Vec::new();

  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();

  for item in parts {
    match item.as_rule() {
      Rule::function_declaration => methods.push(parse_function(item)),
      Rule::instance_declaration => instances.push(parse_instance(item)),
      Rule::key_value            => keyvalues.push(parse_key_value(item)),

      _ => unreachable!()
    }
  }

  ResourceDeclaration::new(kind, name, &keyvalues, &methods, &instances)
}

pub fn parse_key_value(pair: Pair) -> KeyValue {
  let mut parts = pair.into_inner();
  let key   = parse_key(parts.next().unwrap().into_inner());
  let value = parse_expression(parts.next().unwrap());
  KeyValue::new(key, value)
}

pub fn parse_key(mut pairs: Pairs) -> Key {
  let name = pairs.next().unwrap().as_str();
  let mut key = Key::name(name);

  while let Some(pair_rule) = pairs.peek().map(|p| p.as_rule()) {
    match pair_rule {
      Rule::name => {
        let right = parse_key(pairs);
        key = Key::dot(key, right);
        break;
      }

      Rule::key_indexing => {
        let index = parse_expression(pairs.next().unwrap());
        key = Key::indexing(name, index);
      }

      _ => unreachable!()
    }
  }
  return key;
}


