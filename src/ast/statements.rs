use crate::ast::info::AstDebugInfo;
use crate::ast::expressions::IExpr;

pub type IStat = AstDebugInfo<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
  Return(IExpr),
  Call(IExpr),
  Body(Vec<IStat>),
  With(IExpr, IStat),
  If(IExpr, IStat, Option<IStat>),
  While(IExpr, IStat),
  For(String, ForRange, IStat),
  Assignment(IExpr, IExpr),
  Var(Vec<VarDeclaration>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForRange {
  Array(IExpr),
  Integer(IExpr, IExpr, Option<IExpr>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarDeclaration {
  Assignment(String, IExpr),
  Name(String)
}


// Implementations

impl Statement {
  pub fn return_op(e: IExpr) -> Self {
    Statement::Return(e)
  }

  pub fn call(e: IExpr) -> Self {
    Statement::Call(e)
  }

  pub fn body(b: &[IStat]) -> Self {
    Statement::Body(Vec::from(b))
  }

  pub fn with(expr: IExpr, body: IStat) -> Self {
    Statement::With(expr, body)
  }

  pub fn if_op(cond: IExpr, then: IStat, or_else: Option<IStat>) -> Self {
    Statement::If(cond, then, or_else)
  }

  pub fn while_op(cond: IExpr, then: IStat) -> Self {
    Statement::While(cond, then)
  }

  pub fn for_op(name: &str, range: ForRange, then: IStat) -> Self {
    Statement::For(String::from(name), range, then)
  }

  #[allow(dead_code)]
  pub fn for_array(name: &str, array: IExpr, then: IStat) -> Self {
    Statement::For(String::from(name), ForRange::Array(array), then)
  }

  #[allow(dead_code)]
  pub fn for_integer(name: &str, s: IExpr, e: IExpr, by: Option<IExpr>, then: IStat) -> Self {
    Statement::For(String::from(name), ForRange::Integer(s, e, by), then)
  }

  pub fn assignment(left: IExpr, right: IExpr) -> Self {
    Statement::Assignment(left, right)
  }

  pub fn var(d: &[VarDeclaration]) -> Self {
    Statement::Var(Vec::from(d))
  }
}

impl ForRange {
  pub fn array(a: IExpr) -> Self {
    ForRange::Array(a)
  }

  pub fn integer(s: IExpr, e: IExpr, by: Option<IExpr>) -> Self {
    ForRange::Integer(s, e, by)
  }
}

impl VarDeclaration {
  pub fn assignment(name: &str, e: IExpr) -> Self {
    VarDeclaration::Assignment(String::from(name), e)
  }

  pub fn name(s: &str) -> Self {
    VarDeclaration::Name(String::from(s))
  }
}

