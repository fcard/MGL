pub trait Operator {
  fn priority(self) -> i64;
  fn from_str(s: &str) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
  Dot,
  Add,
  Sub,
  Mul,
  Div,
  Or,
  And,
  Lt,
  Gt,
  Geq,
  Leq,
  Eq,
  Diff
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
  Neg,
  Not
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessor {
  None,
  List,
  Map,
  Grid,
  Array
}

// Implementations

macro implement_into_operator_for_str {
  ($T: ty) => {
    impl Into<$T> for &str {
      fn into(self) -> $T {
        Operator::from_str(self)
      }
    }
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

implement_into_operator_for_str!(UnaryOp);

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
      Lt    => 4,
      Gt    => 4,
      Geq   => 4,
      Leq   => 4,
      Eq    => 4,
      Diff  => 4,
      And   => 5,
      Or    => 6,
    }
  }
}


implement_into_operator_for_str!(BinaryOp);

impl Operator for Accessor {
  fn from_str(s: &str) -> Self {
    use Accessor::*;

    match s {
      ""  => None,
      "|" => List,
      "?" => Map,
      "#" => Grid,
      "@" => Array,
      _ => unreachable!()
    }
  }

  fn priority(self) -> i64 {
    return 1;
  }
}

implement_into_operator_for_str!(Accessor);
