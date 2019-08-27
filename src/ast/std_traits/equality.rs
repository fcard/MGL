use crate::ast::*;

impl<T: PartialEq> PartialEq for AstDebugInfo<T> {
  fn eq(&self, other: &AstDebugInfo<T>) -> bool {
    self.content == other.content
  }
}
