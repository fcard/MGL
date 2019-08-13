use crate::ast::operators::Accessor;
use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
  Function(FunctionDeclaration),
  Object(ObjectDeclaration),
  Resource(ResourceDeclaration)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
  name: String,
  args: Vec<String>,
  body: Box<Statement>
}


#[derive(Debug, Clone, PartialEq)]
pub struct ObjectDeclaration {
  name: String,
  key_value_pairs: Vec<KeyValue>,
  methods: Vec<FunctionDeclaration>,
  wrapper: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ResourceDeclaration {
  kind: ResourceKind,
  name: String,
  key_value_pairs: Vec<KeyValue>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceKind {
  Sprite,
  Sound,
  Room
}


#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
  key: Key,
  value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
  Name(String),
  Indexing(String, Accessor, Vec<Expression>)
}


// Implementations

impl Declaration {
  pub fn function(name: &str, args: &[&str], body: Statement) -> Declaration {
    Declaration::Function(
      FunctionDeclaration {
        name: String::from(name),
        args: args.iter().map(|x| String::from(*x)).collect(),
        body: box body
      },
    )
  }

  pub fn object(name: &str,
                keyvals: &[KeyValue],
                methods: &[FunctionDeclaration],
                wrapper: bool) -> Declaration {

    return Declaration::Object(
      ObjectDeclaration {
        name:            String::from(name),
        key_value_pairs: Vec::from(keyvals),
        methods:         Vec::from(methods),
        wrapper:         wrapper
      },
    )
  }

  pub fn resource(kind: ResourceKind, name: &str, keyvals: &[KeyValue]) -> Declaration {
    Declaration::Resource(
      ResourceDeclaration {
        kind: kind,
        name: String::from(name),
        key_value_pairs: Vec::from(keyvals)
      },
    )
  }
}


impl Key {
  pub fn name(name: &str) -> Key {
    Key::Name(String::from(name))
  }

  pub fn indexing<T: Into<Accessor>>(name: &str, accessor: T, args: &[Expression]) -> Key {
    Key::Indexing(String::from(name), accessor.into(), Vec::from(args))
  }
}

impl KeyValue {
  pub fn new(key: Key, value: Expression) -> KeyValue {
    KeyValue {
      key,
      value
    }
  }
}

