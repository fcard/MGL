use std::str::FromStr;
use crate::error::*;

pub macro implement_from_str {
  (argument = $s: ident; $($({$($tps: tt)*})?$T: ty => $func: expr),*) => {
    $(
      impl$(<$($tps)*>)* FromStr for $T {
        type Err = MglError;

        fn from_str($s: &str) -> Result<Self> {
          let $s = $s.trim();
          $func
        }
      }
    )*
  }
}

