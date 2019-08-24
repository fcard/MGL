use crate::ast::*;

use std::convert::AsRef;
use std::ops::Deref;
use std::ops::DerefMut;

impl<T> Deref for AstDebugInfo<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.content
  }
}

impl<T> DerefMut for AstDebugInfo<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.content
  }
}

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



