//! This module fixes unary and binary operations according to the priority of their operators
//! e.g. `a * b + c` is initially parsed as `a * (b + c)`, then fixed to be `(a * b) + c`

use crate::parser::ast::*;


pub trait FixPrecedence {
  fn fix_precedence(&self) -> Self;
}

impl FixPrecedence for Expression {
  fn fix_precedence(&self) -> Self {
    use Expression::*;

    match &self {
      &UnaryOp(op, box e) => {
        fix_unary_precedence(*op, e.clone())
      }

      &BinaryOp(op, box left, box right) => {
        fix_binary_precedence(*op, left.clone(), right.clone())
      }

      _ => unreachable!()
    }
  }
}


pub fn fix_unary_precedence(op: UnaryOp, e: Expression) -> Expression {
  use Expression::*;

  match e.clone() {
    BinaryOp(right_op, box left, box right) => {
      if right_op.priority() <= op.priority() {
        Expression::unary_op(op, e)

      } else {
        Expression::binary_op(right_op, Expression::unary_op(op, left), right)
      }
    }

    TernaryOp(box condition, box left, box right) => {
      Expression::ternary_op(Expression::unary_op(op, condition), left, right)
    }

    _ => Expression::unary_op(op, e)
  }
}


pub fn fix_binary_precedence(op: BinaryOp, left: Expression, right: Expression) -> Expression {
  use Expression::*;

  match right.clone() {
    BinaryOp(right_op, box right_left, box right_right) => {
      if right_op.priority() < op.priority() {
        Expression::binary_op(op, left, right)

      } else {
        Expression::binary_op(
          right_op, Expression::binary_op(op, left, right_left), right_right
        )
      }
    }

    TernaryOp(box condition, box right_left, box right_right) => {
      Expression::ternary_op(
        Expression::binary_op(op, left, condition), right_left, right_right
      )
    }

    _ => Expression::binary_op(op, left, right)
  }
}

