use crate::ast::*;
use crate::error::*;
use crate::resources::resource_trait::*;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Instance {
  pub object: ResourceName,
  pub id: u32,
  pub x: i64,
  pub y: i64,
  pub scale_x: f64,
  pub scale_y: f64,
  pub rotation: f64,
  pub alpha: f64,
  pub color: u32,
  pub creation_code: Option<ResourceName>,
}

impl ResourceDefault<InstanceDeclaration> for Instance {
  fn default(declaration: &InstanceDeclaration) -> Result<Self> {
    Ok(
      Instance {
        object: declaration.object.clone(),
        id: 0,
        x: 0,
        y: 0,
        scale_x: 1.0,
        scale_y: 1.0,
        rotation: 0.0,
        alpha: 1.0,
        color: 0,
        creation_code: None,
      }
    )
  }
}
