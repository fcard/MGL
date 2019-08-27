use crate::parser::tokens::Tokens;
use crate::source_files::*;

#[derive(Clone, Eq)]
pub struct AstDebugInfo<T> {
  pub content: Box<T>,
  pub line_start: usize,
  pub line_end: usize,
  pub column_start: usize,
  pub column_end: usize,
  pub file: SourceFile,
}

impl<T: Clone> AstDebugInfo<T> {
  pub fn new(content: T) -> Self {
    AstDebugInfo {
      content: box content,
      line_start: 0,
      line_end: 0,
      column_start: 0,
      column_end: 0,
      file: SourceFile::None
    }
  }

  pub fn content(self) -> T {
    *self.content
  }

  pub fn content_clone(&self) -> T {
    *self.content.clone()
  }

  pub fn with_content(self, c: T) -> Self {
    let mut new = self.clone();
    new.content = box c;
    new
  }

  pub fn with_info(self, tk: Tokens) -> Self {
    let mut new = self.clone();
    let span  = tk.as_span();
    let (sy, sx) = span.start_pos().line_col();
    let (ey, ex) = span.end_pos().line_col();

    new.file = tk.file();
    new.line_start   = sy;
    new.column_start = sx;
    new.line_end     = ey;
    new.column_end   = ex;
    new
  }
}
