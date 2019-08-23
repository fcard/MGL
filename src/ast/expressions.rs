use crate::ast::wrapper::Ast;
use crate::ast::operators::*;

type Expr = Ast<Expression>;
pub trait ExprRef = AsRef<Expression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
  Str(String),
  Num(String),
  Bool(bool),
  Name(String),
  Resource(ResourceName),
  Parentheses(Expr),
  UnaryOp(UnaryOp, Expr),
  BinaryOp(BinaryOp, Expr, Expr),
  TernaryOp(Expr, Expr, Expr),
  Call(Expr, Vec<Expr>),
  Indexing(Expr, Accessor, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceName {
  Name(String),
  InModule(String, Box<ResourceName>)
}

// Implementations

impl Expression {
  pub fn string(s: &str) -> Self {
    Expression::Str(String::from(s))
  }

  pub fn num(s: &str) -> Self {
    Expression::Num(String::from(s))
  }

  pub fn boolean(b: bool) -> Self {
    Expression::Bool(b)
  }

  pub fn name(s: &str) -> Self {
    Expression::Name(String::from(s))
  }

  pub fn parentheses(e: Expr) -> Self {
    Expression::Parentheses(e)
  }

  pub fn resource(names: &[&str]) -> Self {
    Expression::Resource(ResourceName::new(names))
  }

  pub fn unary_op<T: Into<UnaryOp>>(op: T, e: Expr) -> Self {
    Expression::UnaryOp(op.into(), e)
  }

  pub fn binary_op<T: Into<BinaryOp>>(op: T, a: Expr, b: Expr) -> Self {
    Expression::BinaryOp(op.into(), a, b)
  }

  pub fn ternary_op(condition: Expr, a: Expr, b: Expr) -> Self {
    Expression::TernaryOp(condition, a, b)
  }

  pub fn call(caller: Expr, args: &[Expr]) -> Self {
    Expression::Call(caller, Vec::from(args))
  }

  pub fn indexing<T: Into<Accessor>>(value: Expr, op: T, keys: &[Expr]) -> Self {
    Expression::Indexing(value, op.into(), Vec::from(keys))
  }
}

impl ResourceName {
  pub fn new(names: &[&str]) -> ResourceName {
    let mut resource = None;

    for name in names.iter().rev() {
      resource = Some(match resource {
        None      => ResourceName::Name(String::from(*name)),
        Some(res) => ResourceName::InModule(String::from(*name), box res)
      });
    }
    resource.unwrap()
  }

  pub fn top_module_is(&self, name: &str) -> bool {
    match &self {
      ResourceName::Name(_) => false,
      ResourceName::InModule(module, _) => &*module == name
    }
  }
}

