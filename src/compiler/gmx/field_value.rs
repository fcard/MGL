use std::path::PathBuf;
use std::ffi::OsStr;
use crate::ast::*;
use crate::compiler::script::*;

pub trait ToGmxValue {
  fn to_gmx_value(&self) -> String;
}

macro impl_gmx_trait {
  (|&$s: ident| {
      $($({$($tps: tt)*})?$T: ty => $body: expr),*
   }) => {

    $(
      impl$(<$($tps)*>)* ToGmxValue for $T {
        fn to_gmx_value(&$s) -> String {
          $body
        }
      }
    )*
  }
}

impl_gmx_trait! {
  |&self| {
    u8      => self.to_string(),
    u32     => self.to_string(),
    u64     => self.to_string(),
    usize   => self.to_string(),
    i64     => self.to_string(),
    i32     => self.to_string(),
    f64     => self.to_string(),
    &str    => self.to_string(),
    &OsStr  => self.to_string_lossy().to_string(),
    bool    => (if *self { -1 } else { 0 }).to_string(),

    PathBuf => self.iter().map(|p| p.to_str().unwrap_or("").to_string())
                          .collect::<Vec<_>>().join("\\"),

    {T: ToGmxValue} Option<T> => {
      if let Some(value) = self {
        value.to_gmx_value()
      } else {
        undefined()
      }
    }
  }
}

pub trait ToScopedGmxValue {
  fn to_scoped_gmx_value(&self, scope: &str) -> String;
}

impl ToScopedGmxValue for ResourceName {
  fn to_scoped_gmx_value(&self, scope: &str) -> String {
    build_resource_name(&self.with_top_module(scope))
  }
}

impl ToScopedGmxValue for Option<ResourceName> {
  fn to_scoped_gmx_value(&self, scope: &str) -> String {
    self.as_ref()
        .map(|s| s.with_top_module(scope))
        .map(|s| build_resource_name(&s))
        .unwrap_or(undefined())
  }
}

fn undefined() -> String {
  String::from("<undefined>")
}


