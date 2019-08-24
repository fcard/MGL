use crate::ast::info::AstDebugInfo;
use crate::ast::operators::*;

pub type IExpr = AstDebugInfo<Expression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
  Str(String),
  Num(String),
  Bool(bool),
  Name(String),
  Resource(ResourceName),
  Parentheses(IExpr),
  UnaryOp(UnaryOp, IExpr),
  BinaryOp(BinaryOp, IExpr, IExpr),
  TernaryOp(IExpr, IExpr, IExpr),
  Call(IExpr, Vec<IExpr>),
  Indexing(IExpr, Accessor, Vec<IExpr>),
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

  pub fn parentheses(e: IExpr) -> Self {
    Expression::Parentheses(e)
  }

  pub fn resource(names: &[&str]) -> Self {
    Expression::Resource(ResourceName::new(names))
  }

  pub fn unary_op<T: Into<UnaryOp>>(op: T, e: IExpr) -> Self {
    Expression::UnaryOp(op.into(), e)
  }

  pub fn binary_op<T: Into<BinaryOp>>(op: T, a: IExpr, b: IExpr) -> Self {
    Expression::BinaryOp(op.into(), a, b)
  }

  pub fn ternary_op(condition: IExpr, a: IExpr, b: IExpr) -> Self {
    Expression::TernaryOp(condition, a, b)
  }

  pub fn call(caller: IExpr, args: &[IExpr]) -> Self {
    Expression::Call(caller, Vec::from(args))
  }

  pub fn indexing<T: Into<Accessor>>(value: IExpr, op: T, keys: &[IExpr]) -> Self {
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

