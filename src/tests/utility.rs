use crate::ast::*;
use crate::error::*;
use std::str::FromStr;

pub trait CloneAll<T> {
  fn clone_all(&self) -> T;
}

impl<T: Clone> CloneAll<Vec<T>> for Vec<&T> {
  fn clone_all(&self) -> Vec<T> {
    self.iter().map(|x| (*x).clone()).collect()
  }
}

pub fn parse_unwrap<T: FromStr<Err=MglError>>(code: &str) -> T {
  code.parse().unwrap()
}

macro parse_unwrap_aliases($($func: ident -> $T: ty;)*) {
  $(
    pub fn $func(code: &str) -> $T {
      parse_unwrap(code)
    }
  )*
}

parse_unwrap_aliases! {
  expr        -> IExpr;
  statement   -> IStat;
  declaration -> Declaration;
  key         -> Key;
  function    -> FunctionDeclaration;
  instance    -> InstanceDeclaration;
  resource    -> ResourceDeclaration;
}

pub macro resource($($arg: expr),*) {
  Declaration::Resource(ResourceDeclaration::new($($arg),*))
}


pub macro keys {
  () => { Vec::new() },

  ($($k: tt)+) => {
    stringify!($($k)*)
      .split(",")
      .map(|s| s.trim().parse().unwrap())
      .collect::<Vec<KeyValue>>()
  }
}

pub macro rn($name: ident$(::$subnames: ident)*) {
  ResourceName::new(&[stringify!($name)$(, stringify!($subnames))*])
}
