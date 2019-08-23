use crate::ast::*;

impl<T: PartialEq> PartialEq for Ast<T> {
  fn eq(&self, other: &Ast<T>) -> bool {
    self.content == other.content
  }
}
