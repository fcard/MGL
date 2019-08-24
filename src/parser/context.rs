use crate::source_files::*;

pub struct ParserContext<'a> {
  pub code: &'a str,
  pub file: SourceFile,
  pub verbose_errors: bool,
}

pub fn code<'a>(c: &'a str) -> ParserContext<'a> {
  ParserContext {
    code: c,
    file: SourceFile::None,
    verbose_errors: true,
  }
}

impl<'a> ParserContext<'a> {
  pub fn new(c: &'a str) -> Self {
    ParserContext {
      code: c,
      file: SourceFile::None,
      verbose_errors: true,
    }
  }

  pub fn with_file(self, file: SourceFile) -> Self {
    ParserContext {
      code: self.code,
      file: file,
      verbose_errors: self.verbose_errors
    }
  }

  #[allow(dead_code)]
  pub fn with_verbose_errors(self, v: bool) -> Self {
    ParserContext {
      code: self.code,
      file: self.file,
      verbose_errors: v
    }
  }
}

impl<'a> From<&'a str> for ParserContext<'a> {
  fn from(s: &'a str) -> ParserContext<'a> {
    code(s)
  }
}

