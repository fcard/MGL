use crate::ast::operators::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Str(String),
  Num(String),
  Bool(bool),
  Name(String),
  Resource(ResourceName),
  Parentheses(Box<Expression>),
  UnaryOp(UnaryOp, Box<Expression>),
  BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
  TernaryOp(Box<Expression>, Box<Expression>, Box<Expression>),
  Call(Box<Expression>, Vec<Expression>),
  Indexing(Box<Expression>, Accessor, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceName {
  Name(String),
  InModule(String, Box<ResourceName>)
}

// Implementations

impl Expression {
  pub fn string(s: &str) -> Expression {
    Expression::Str(String::from(s))
  }

  pub fn num(s: &str) -> Expression {
    Expression::Num(String::from(s))
  }

  pub fn boolean(b: bool) -> Expression {
    Expression::Bool(b)
  }

  pub fn name(s: &str) -> Expression {
    Expression::Name(String::from(s))
  }

  pub fn parentheses(e: Expression) -> Expression {
    Expression::Parentheses(box e)
  }

  pub fn resource(names: &[&str]) -> Expression {
    let mut resource = None;

    for name in names.iter().rev() {
      resource = Some(match resource {
        None      => ResourceName::Name(String::from(*name)),
        Some(res) => ResourceName::InModule(String::from(*name), box res)
      });
    }
    Expression::Resource(resource.unwrap())
  }

  pub fn unary_op<T: Into<UnaryOp>>(op: T, e: Expression) -> Expression {
    Expression::UnaryOp(op.into(), box e)
  }

  pub fn binary_op<T: Into<BinaryOp>>(op: T, a: Expression, b: Expression) -> Expression {
    Expression::BinaryOp(op.into(), box a, box b)
  }

  pub fn ternary_op(condition: Expression, a: Expression, b: Expression) -> Expression {
    Expression::TernaryOp(box condition, box a, box b)
  }

  pub fn call(caller: Expression, args: &[Expression]) -> Expression {
    Expression::Call(box caller, Vec::from(args))
  }

  pub fn indexing<T: Into<Accessor>>(value: Expression, op: T, keys: &[Expression]) -> Expression {
    Expression::Indexing(box value, op.into(), Vec::from(keys))
  }
}

