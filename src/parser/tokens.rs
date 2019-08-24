use crate::source_files::*;
use crate::parser::grammar::*;

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
  pair: Pair<'a>,
  file: SourceFile,
}

#[derive(Debug, Clone)]
pub struct InnerTokens<'a> {
  pairs: Pairs<'a>,
  file: SourceFile
}


impl<'a> Tokens<'a> {
  pub fn new(pair: Pair<'a>, file: SourceFile) -> Self {
    Tokens {
      pair,
      file,
    }
  }

  pub fn file(&'a self) -> SourceFile {
    self.file
  }

  pub fn as_str(self) -> &'a str {
    self.pair.as_str()
  }

  pub fn as_span(&self) -> Span {
    self.pair.as_span()
  }

  pub fn as_rule(&self) -> Rule {
    self.pair.as_rule()
  }

  pub fn into_inner(self) -> InnerTokens<'a> {
    InnerTokens::new(self.pair.into_inner(), self.file)
  }
}


impl<'a> InnerTokens<'a> {
  pub fn new(pairs: Pairs<'a>, file: SourceFile) -> Self {
    InnerTokens {
      pairs,
      file,
    }
  }

  pub fn peek(&self) -> Option<Tokens<'a>> {
    self.pairs.peek().map(|p| Tokens::new(p, self.file))
  }
}

impl<'a> Iterator for InnerTokens<'a> {
  type Item = Tokens<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.pairs.next().map(|p| Tokens::new(p, self.file))
  }
}

