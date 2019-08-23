use crate::parser::grammar::Pair;
use crate::parser::context::Ctx;
use std::path::PathBuf;

#[derive(Clone, Eq)]
pub struct Ast<T> {
  pub content: Box<T>,
  pub line_start: usize,
  pub line_end: usize,
  pub column_start: usize,
  pub column_end: usize,
  pub file: PathBuf,
}

impl<T: Clone> Ast<T> {
  pub fn new(content: T) -> Self {
    Ast {
      content: box content,
      line_start: 0,
      line_end: 0,
      column_start: 0,
      column_end: 0,
      file: PathBuf::new()
    }
  }

  pub fn content(self, c: T) -> Self {
    let mut new = self.clone();
    new.content = box c;
    new
  }

  pub fn file(self, c: &Ctx) -> Self {
    let mut new = self.clone();
    new.file = c.file.clone();
    new
  }

  pub fn pos(self, pair: Pair) -> Self {
    let mut new = self.clone();
    let span  = pair.as_span();
    let (sy, sx) = span.start_pos().line_col();
    let (ey, ex) = span.end_pos().line_col();

    new.line_start   = sy;
    new.column_start = sx;
    new.line_end     = ey;
    new.column_end   = ex;
    new
  }
}
