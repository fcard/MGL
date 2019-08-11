#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Top {
  declarations: Vec<Declaration>
}

impl Top {
  pub fn new(d: &[Declaration]) -> Self {
    Top {
      declarations: Vec::from(d)
    }
  }
}

#[derive(Debug, Clone)]
pub enum Expression {
  Str(String),
  Num(String),
  Bool(bool),
  Name(String),
  Parentheses(Box<Expression>),
  UnaryOp(UnaryOp, Box<Expression>),
  BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
  TernaryOp(Box<Expression>, Box<Expression>, Box<Expression>),
  Call(Box<Expression>, Vec<Expression>),
  Indexing(Box<Expression>, String, Vec<Expression>),
}

pub trait Operator {
  fn priority(self) -> i64;
  fn from_str(s: &str) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
  Dot, Add, Sub, Mul, Div, Or, And, Lt, Gt, Geq, Leq, Eq, Diff
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
  Neg, Not
}

#[derive(Debug, Clone)]
pub enum Statement {
  Return(Expression),
  Call(Expression),
  Body(Vec<Statement>),
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

#[derive(Debug, Clone)]
pub enum Declaration {
  Function(String, Vec<String>, Box<Statement>),
}


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

  pub fn indexing(value: Expression, op: &str, keys: &[Expression]) -> Expression {
    Expression::Indexing(box value, String::from(op), Vec::from(keys))
  }
}

impl Operator for UnaryOp {
  fn from_str(s: &str) -> Self {
    use UnaryOp::*;

    match s {
      "-" => Neg,
      "!" => Not,
      _ => unreachable!()
    }
  }

  fn priority(self) -> i64 {
    use UnaryOp::*;

    match self {
      Neg => 1,
      Not => 1,
    }
  }
}

impl Into<UnaryOp> for &str {
  fn into(self) -> UnaryOp {
    UnaryOp::from_str(self)
  }
}

impl Operator for BinaryOp {
  fn from_str(s: &str) -> Self {
    use BinaryOp::*;

    match s {
      "."  => Dot,
      "+"  => Add,
      "-"  => Sub,
      "*"  => Mul,
      "/"  => Div,
      "||" => Or,
      "&&" => And,
      "<"  => Lt,
      ">"  => Gt,
      ">=" => Geq,
      "<=" => Leq,
      "==" => Eq,
      "!=" => Diff,
      _ => unreachable!()
    }
  }

  fn priority(self) -> i64 {
    use BinaryOp::*;

    match self {
      Dot   => 1,
      Mul   => 2,
      Div   => 2,
      Add   => 3,
      Sub   => 3,
      And   => 4,
      Or    => 5,
      Lt    => 6,
      Gt    => 6,
      Geq   => 6,
      Leq   => 6,
      Eq    => 6,
      Diff  => 6,
    }
  }
}



impl Into<BinaryOp> for &str {
  fn into(self) -> BinaryOp {
    BinaryOp::from_str(self)
  }
}

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

  pub fn if_op(cond: Expression, then: Statement, or_else: Option<Statement>) -> Statement {
    Statement::If(cond, box then, or_else.map(Box::new))
  }

  pub fn while_op(cond: Expression, then: Statement) -> Statement {
    Statement::While(cond, box then)
  }

  pub fn for_op(name: &str, range: ForRange, then: Statement) -> Statement {
    Statement::For(String::from(name), range, box then)
  }

  pub fn for_array(name: &str, array: Expression, then: Statement) -> Statement {
    Statement::For(String::from(name), ForRange::Array(array), box then)
  }

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

impl Declaration {
  pub fn function(name: &str, args: &[&str], body: Statement) -> Declaration {
    Declaration::Function(
      String::from(name), args.iter().map(|x| String::from(*x)).collect(), box body)
  }
}
