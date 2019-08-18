use std::fmt::*;
use crate::ast::*;

impl Display for ResourceName {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut resource = self;

    while let ResourceName::InModule(module, box next) = resource {
      write!(f, "{}::", module)?;
      resource = next;
    }

    if let ResourceName::Name(name) = resource {
      write!(f, "{}", name)
    } else {
      unreachable!()
    }
  }
}

