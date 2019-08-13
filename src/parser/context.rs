pub struct ParserContext<'a> {
  pub code: &'a str,
  pub verbose_errors: bool,
}

pub fn code<'a>(c: &'a str) -> ParserContext<'a> {
  ParserContext {
    code: c,
    verbose_errors: true,
  }
}

impl<'a> ParserContext<'a> {
  #[allow(dead_code)]
  pub fn verbose_errors(self, v: bool) -> ParserContext<'a> {
    ParserContext {
      code: self.code,
      verbose_errors: v
    }
  }
}

impl<'a> From<&'a str> for ParserContext<'a> {
  fn from(s: &'a str) -> ParserContext<'a> {
    code(s)
  }
}

