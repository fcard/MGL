use crate::parser::ast::*;

impl Into<Op> for &str {
  fn into(self) -> Op {
    Op::new(self)
  }
}

pub trait FixPrecedence {
  fn fix_precedence(&self) -> Self;
}

impl FixPrecedence for Expression {
  fn fix_precedence(&self) -> Self {
    use Expression::*;

    match &self {
      &Str(_) | &Num(_) | &Bool(_) | &Name(_) => self.clone(),

      &Parentheses(e) => Expression::parentheses(e.fix_precedence()),

      &Negation(box e) => fix_negation_precedence(e.fix_precedence()),

      _ => unreachable!()
    }
  }
}

impl FixPrecedence for Statement {
  fn fix_precedence(&self) -> Self {
    use Statement::*;

    match &self {
      &Return(e) => Statement::return_op(e.fix_precedence()),
      &Call(e)   => Statement::call(e.fix_precedence()),

      &Body(es) => {
        let fixed_es: Vec<_> = es.iter().map(FixPrecedence::fix_precedence).collect();
        Statement::body(&fixed_es)
      }

      &If(cond, box then, or_else) => {
        Statement::if_op(
          cond.fix_precedence(),
          then.fix_precedence(),
          or_else.map(|box s| s.fix_precedence())
        )
      }

      &While(cond, box then) => {
        Statement::while_op(
          cond.fix_precedence(),
          then.fix_precedence(),
        )
      }

      &For(name, range, box then) => {
        Statement::for_op(
          &name,
          range.fix_precedence(),
          then.fix_precedence(),
        )
      }

      &Assignment(left, right) => {
        Statement::assignment(
          left.fix_precedence(),
          right.fix_precedence(),
        )
      }

      &Var(left, right) => {
        Statement::assignment(
          left.fix_precedence(),
          right.fix_precedence(),
        )
      }

      _ => unreachable!()
    }
  }
}

impl FixPrecedence for ForRange {
  fn fix_precedence(&self) -> Self {
    use ForRange::*;

    match &self {
      Array(e) => ForRange::array(e.fix_precedence()),

      Integer(from, to, by) => {
        ForRange::integer(
          from.fix_precedence(),
          to.fix_precedence(),
          by.map(|e| e.fix_precedence()),
        )
      }
    }
  }
}

impl FixPrecedence for Declaration {
  fn fix_precedence(&self) -> Self {
    match &self {
    }
  }
}

pub fn fix_negation_precedence(e: Expression) -> Expression {
  use Expression::*;

  match e {
      Str(_)    | Num(_) | Bool(_) | Name(_) | Parentheses(_) |
      Call(_,_) | Indexing(_,_,_)  | Negation(_) => Expression::negation(e.fix_precedence()),

      BinaryOp(op, box left, box right) => {
        Expression::binary_op(op, Expression::negation(left), right)
      }

      TernaryOp(box condition, box left, box right) => {
        Expression::ternary_op(Expression::negation(condition), left, right)
      }
  }
}
