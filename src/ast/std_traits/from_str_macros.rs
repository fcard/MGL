use std::str::FromStr;
use crate::error::*;

pub macro implement_from_str {
  (argument = $s: ident; $($T: ty => $func: expr),*) => {
    $(
      impl FromStr for $T {
        type Err = MglError;

        fn from_str($s: &str) -> Result<Self> {
          $func
        }
      }
    )*
  }
}

