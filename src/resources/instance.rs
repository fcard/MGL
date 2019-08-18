use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
  object: ResourceName,
  id: u32,
  x: i64,
  y: i64,
  scale: f64,
  rotation: f64,
  alpha: f64,
  color: u32,
  creation_code: Option<ResourceName>,
}

impl Instance {
  pub fn new(declaration: InstanceDeclaration) -> Self {
    Instance {
      object: declaration.object,
      id: 0,
      x: 0,
      y: 0,
      scale: 1.0,
      rotation: 0.0,
      alpha: 1.0,
      color: 0,
      creation_code: None,
    }
  }
}
