use std::path::PathBuf;

pub type Ctx<'a> = ParserContext<'a>;

pub struct ParserContext<'a> {
  pub code: &'a str,
  pub file: PathBuf,
  pub verbose_errors: bool,
}

pub fn code<'a>(c: &'a str) -> ParserContext<'a> {
  ParserContext {
    code: c,
    file: PathBuf::new(),
    verbose_errors: true,
  }
}

impl<'a> ParserContext<'a> {
  pub fn file(self, file: PathBuf) -> Self {
    ParserContext {
      code: self.code,
      file: file,
      verbose_errors: self.verbose_errors
    }
  }

  #[allow(dead_code)]
  pub fn verbose_errors(self, v: bool) -> Self {
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

