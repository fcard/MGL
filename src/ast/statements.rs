use crate::ast::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
  Return(Expression),
  Call(Expression),
  Body(Vec<Statement>),
  With(Expression, Box<Statement>),
  If(Expression, Box<Statement>, Option<Box<Statement>>),
  While(Expression, Box<Statement>),
  For(String, ForRange, Box<Statement>),
  Assignment(Expression, Expression),
  Var(Vec<VarDeclaration>),
}

#[derive(Debug, Clone)]
pub enum ForRange {
  Array(Expression),
  Integer(Expression, Expression, Option<Expression>)
}

#[derive(Debug, Clone)]
pub enum VarDeclaration {
  Assignment(String, Expression),
  Name(String)
}


// Implementations

impl Statement {
  pub fn return_op(e: Expression) -> Statement {
    Statement::Return(e)
  }

  pub fn call(e: Expression) -> Statement {
    Statement::Call(e)
  }

  pub fn body(b: &[Statement]) -> Statement {
    Statement::Body(Vec::from(b))
  }

  pub fn with(expr: Expression, body: Statement) -> Statement {
    Statement::With(expr, box body)
  }

  pub fn if_op(cond: Expression, then: Statement, or_else: Option<Statement>) -> Statement {
    Statement::If(cond, box then, or_else.map(Box::new))
  }

  pub fn while_op(cond: Expression, then: Statement) -> Statement {
    Statement::While(cond, box then)
  }

  pub fn for_op(name: &str, range: ForRange, then: Statement) -> Statement {
    Statement::For(String::from(name), range, box then)
  }

  #[allow(dead_code)]
  pub fn for_array(name: &str, array: Expression, then: Statement) -> Statement {
    Statement::For(String::from(name), ForRange::Array(array), box then)
  }

  #[allow(dead_code)]
  pub fn for_integer(name: &str, s: Expression, e: Expression, by: Option<Expression>,
                     then: Statement) -> Statement {
    Statement::For(String::from(name), ForRange::Integer(s, e, by), box then)
  }

  pub fn assignment(left: Expression, right: Expression) -> Statement {
    Statement::Assignment(left, right)
  }

  pub fn var(d: &[VarDeclaration]) -> Statement {
    Statement::Var(Vec::from(d))
  }
}

impl ForRange {
  pub fn array(a: Expression) -> ForRange {
    ForRange::Array(a)
  }

  pub fn integer(s: Expression, e: Expression, by: Option<Expression>) -> ForRange {
    ForRange::Integer(s, e, by)
  }
}

impl VarDeclaration {
  pub fn assignment(name: &str, e: Expression) -> VarDeclaration {
    VarDeclaration::Assignment(String::from(name), e)
  }

  pub fn name(s: &str) -> VarDeclaration {
    VarDeclaration::Name(String::from(s))
  }
}

