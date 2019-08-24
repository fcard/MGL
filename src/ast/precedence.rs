//! This module fixes unary and binary operations according to the priority of their operators
//! e.g. `a * b + c` is initially parsed as `a * (b + c)`, then fixed to be `(a * b) + c`

use crate::ast::*;

pub trait FixPrecedence {
  fn fix_precedence(self) -> Self;
}

impl FixPrecedence for IExpr {
  fn fix_precedence(self) -> Self {
    use Expression::*;

    match *self.content.clone() {
      UnaryOp(op, e) => {
        fix_unary_precedence(self, op, e)
      }

      BinaryOp(op, left, right) => {
        fix_binary_precedence(self, op, left.clone(), right.clone())
      }

      _ => self
    }
  }
}


pub fn fix_unary_precedence(ast: IExpr, op: UnaryOp, e: IExpr) -> IExpr {
  use Expression::*;

  match *e.content.clone() {
    BinaryOp(bin_op, left, right) => {
      if bin_op.priority() <= op.priority() {
        ast.content(Expression::unary_op(op, e))

      } else {
        let new_left = ast.content(Expression::unary_op(op, left));
        e.content(Expression::binary_op(bin_op, new_left, right))
      }
    }

    TernaryOp(condition, left, right) => {
      let new_cond = ast.content(Expression::unary_op(op, condition));
      e.content(Expression::ternary_op(new_cond, left, right))
    }

    _ => ast.content(Expression::unary_op(op, e))
  }
}


pub fn fix_binary_precedence(ast: IExpr, op: BinaryOp, left: IExpr, right: IExpr) -> IExpr {
  use Expression::*;

  match *right.content.clone() {
    BinaryOp(right_op, right_left, right_right) => {
      if right_op.priority() < op.priority() {
        ast.content(Expression::binary_op(op, left, right))

      } else {
        let new_left = ast.content(Expression::binary_op(op, left, right_left));
        right.content(Expression::binary_op(right_op, new_left, right_right))
      }
    }

    TernaryOp(condition, right_left, right_right) => {
      let new_cond = ast.content(Expression::binary_op(op, left, condition));
      right.content(Expression::ternary_op(new_cond, right_left, right_right))
    }

    _ => ast.content(Expression::binary_op(op, left, right))
  }
}

