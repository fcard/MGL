use crate::ast::expressions::{Expression, IExpr, ResourceName};
use crate::ast::statements::IStat;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
  Function(FunctionDeclaration),
  Resource(ResourceDeclaration),
  Instance(InstanceDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
  pub name: String,
  pub args: Vec<String>,
  pub body: IStat
}


#[derive(Debug, Clone, PartialEq)]
pub struct ResourceDeclaration {
  pub kind: ResourceKind,
  pub name: String,
  pub methods: Vec<FunctionDeclaration>,
  pub instances: Vec<InstanceDeclaration>,
  pub key_value_pairs: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceDeclaration {
  pub object: ResourceName,
  pub name: String,
  pub key_value_pairs: Vec<KeyValue>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceKind {
  Room,
  Sound,
  Sprite,
  Object,
  Wrapper,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
  pub key:   Key,
  pub value: IExpr
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
  Name(String),
  Indexing(String, IExpr),
  Dot(Box<Key>, Box<Key>)
}

// Implementations

impl FunctionDeclaration {
  pub fn new(name: &str, args: &[&str], body: IStat) -> Self {
    FunctionDeclaration {
      name: String::from(name),
      args: args.iter().map(|x| String::from(*x)).collect(),
      body: body
    }
  }
}

impl InstanceDeclaration {
  pub fn new(object_expression: IExpr, name: &str, keyvals: &[KeyValue]) -> Self {
    let object_name;
    match *object_expression.content {
      Expression::Resource(name) => { object_name = name },
      Expression::Name(name)     => { object_name = ResourceName::Name(name) }
      _ => unreachable!()
    }
    InstanceDeclaration {
      name:            String::from(name),
      object:          object_name,
      key_value_pairs: Vec::from(keyvals)
    }
  }
}

impl ResourceDeclaration {
  pub fn new(kind:      ResourceKind,
             name:      &str,
             keyvals:   &[KeyValue],
             methods:   &[FunctionDeclaration],
             instances: &[InstanceDeclaration]) -> Self {

    ResourceDeclaration {
      kind:            kind,
      name:            String::from(name),
      methods:         Vec::from(methods),
      instances:       Vec::from(instances),
      key_value_pairs: Vec::from(keyvals),
    }
  }
}

impl ResourceKind {
  pub fn module(&self) -> String {
    use ResourceKind::*;

    String::from(
      match self {
        Object   => "object",
        Wrapper  => "wrapper",
        Sprite   => "sprite",
        Sound    => "sound",
        Room     => "room",
      },
    )
  }
}



impl Key {
  pub fn name(name: &str) -> Key {
    Key::Name(String::from(name))
  }

  pub fn indexing(left: &str, arg: IExpr) -> Key {
    Key::Indexing(String::from(left), arg)
  }

  pub fn dot(left: Key, right: Key) -> Key {
    Key::Dot(box left, box right)
  }

  #[allow(dead_code)]
  pub fn is_name(&self) -> bool {
    if let &Key::Name(_) = &self { true } else { false }
  }

  #[allow(dead_code)]
  pub fn is_indexing(&self) -> bool {
    if let &Key::Indexing(_, _) = &self { true } else { false }
  }

  pub fn is_dot(&self) -> bool {
    if let &Key::Dot(_, _) = &self { true } else { false }
  }

  pub fn name_of(&self) -> String {
    match &self {
      Key::Name(name) | Key::Indexing(name, _) => name.clone(),
      Key::Dot(left, _) => left.name_of()
    }
  }

  pub fn index_of<'a>(&'a self) -> Option<&'a IExpr> {
    match &self {
      Key::Indexing(_, index) => Some(&index),
      _ => None
    }
  }

  pub fn leftmost_index_of<'a>(&'a self) -> Option<&'a IExpr> {
    if let Some(left) = self.left_of() {
      left.leftmost_index_of()

    } else {
      self.index_of()
    }
  }

  pub fn left_of<'a>(&'a self) -> Option<&'a Key> {
    match &self {
      Key::Dot(box left, _) => Some(left),
      _ => None
    }
  }
}

impl KeyValue {
  pub fn new(key: Key, value: IExpr) -> KeyValue {
    KeyValue {
      key,
      value
    }
  }
}

