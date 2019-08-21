use crate::ast::*;
use crate::error::*;
use std::convert::TryFrom;

pub macro try_from_common($T: ty, |$expr: ident| $func: expr) {
  impl TryFrom<Expression> for $T {
    type Error = MglError;

    fn try_from($expr: Expression) -> Result<Self> {
      $func
    }
  }
}

pub macro implement_match_try_from {
  ($($T: ty { $($p: pat => $value: expr),* }),*) => {
    $(
      try_from_common!($T, |expr| {
        match expr {
          $($p => Ok($value)),*,
          _ => MglError::convert_expression(expr, stringify!($T))
        }
      });
    )*
  }
}

pub macro implement_try_from_for_numbers($($T: ty),+) {
  $(
    try_from_common!($T, |expr| {
      let value_type = format!("number ({})", stringify!($T));

      match &expr {
        &Expression::Num(ref n) => {
          match n.parse() {
            Ok(n)  => Ok(n),
            Err(_) => MglError::convert_expression(expr.clone(), &value_type)
          }
        }
        _ => MglError::convert_expression(expr.clone(), &value_type)
      }
    });
  )*
}

pub macro implement_try_from_string_options {
  ($($T: ty { $($option: expr => $value: expr),* }),+) => {
    $(
      try_from_common!($T, |expr| {
        let string = String::try_from(expr)?;

        match &*string {
          $($option => Ok($value)),*,
          _ => MglError::unknown_option(&*string, &[$($option),*])
        }
      });
    )*
  }
}

pub macro implement_try_from_wrap_option($($T: ty),*) {
  $(
    try_from_common!(Option<$T>, |expr| {
      Ok(Some(<$T>::try_from(expr)?))
    });
  )*
}

