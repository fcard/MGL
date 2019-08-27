use crate::ast::*;

use std::convert::AsRef;

impl<T> AsRef<T> for AstDebugInfo<T> {
  fn as_ref(&self) -> &T {
    &self.content
  }
}

impl AsRef<Expression> for Expression {
  fn as_ref(&self) -> &Expression {
    &self
  }
}

impl AsRef<Statement> for Statement {
  fn as_ref(&self) -> &Statement {
    &self
  }
}






