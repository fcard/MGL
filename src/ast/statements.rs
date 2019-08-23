use crate::ast::wrapper::*;
use crate::ast::expressions::Expression;

type Expr = Ast<Expression>;
type Stat = Ast<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
  Return(Expr),
  Call(Expr),
  Body(Vec<Stat>),
  With(Expr, Stat),
  If(Expr, Stat, Option<Stat>),
  While(Expr, Stat),
  For(String, ForRange, Stat),
  Assignment(Expr, Expr),
  Var(Vec<VarDeclaration>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForRange {
  Array(Expr),
  Integer(Expr, Expr, Option<Expr>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarDeclaration {
  Assignment(String, Expr),
  Name(String)
}


// Implementations

impl Statement {
  pub fn return_op(e: Expr) -> Self {
    Statement::Return(e)
  }

  pub fn call(e: Expr) -> Self {
    Statement::Call(e)
  }

  pub fn body(b: &[Stat]) -> Self {
    Statement::Body(Vec::from(b))
  }

  pub fn with(expr: Expr, body: Stat) -> Self {
    Statement::With(expr, body)
  }

  pub fn if_op(cond: Expr, then: Stat, or_else: Option<Stat>) -> Self {
    Statement::If(cond, then, or_else)
  }

  pub fn while_op(cond: Expr, then: Stat) -> Self {
    Statement::While(cond, then)
  }

  pub fn for_op(name: &str, range: ForRange, then: Stat) -> Self {
    Statement::For(String::from(name), range, then)
  }

  #[allow(dead_code)]
  pub fn for_array(name: &str, array: Expr, then: Stat) -> Self {
    Statement::For(String::from(name), ForRange::Array(array), then)
  }

  #[allow(dead_code)]
  pub fn for_integer(name: &str, s: Expr, e: Expr, by: Option<Expr>, then: Stat) -> Self {
    Statement::For(String::from(name), ForRange::Integer(s, e, by), then)
  }

  pub fn assignment(left: Expr, right: Expr) -> Self {
    Statement::Assignment(left, right)
  }

  pub fn var(d: &[VarDeclaration]) -> Self {
    Statement::Var(Vec::from(d))
  }
}

impl ForRange {
  pub fn array(a: Expr) -> Self {
    ForRange::Array(a)
  }

  pub fn integer(s: Expr, e: Expr, by: Option<Expr>) -> Self {
    ForRange::Integer(s, e, by)
  }
}

impl VarDeclaration {
  pub fn assignment(name: &str, e: Expr) -> Self {
    VarDeclaration::Assignment(String::from(name), e)
  }

  pub fn name(s: &str) -> Self {
    VarDeclaration::Name(String::from(s))
  }
}

