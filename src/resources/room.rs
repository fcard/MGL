use crate::ast::*;
use crate::resources::instance::*;
use crate::resources::resource_trait::*;
use std::path::PathBuf;

pub type InstanceItems = Vec<InstanceItem>;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Room {
  width: u64,
  height: u64,
  speed: u64,
  persistent: bool,
  enable_views: bool,
  clear_view_background: bool,
  clear_display_buffer: bool,
  color: u32,
  tiled: Option<PathBuf>,

  #[array_field]
  instances: InstanceItems,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceItem {
  Resolved(Instance),
  Unresolved(ResourceName),
}

impl ResourceDefault<(ResourceDeclaration, InstanceItems)> for Room {
  fn default((_resource, instances): &(ResourceDeclaration, InstanceItems)) -> Result<Room> {
    Ok(
      Room {
        width: 1024,
        height: 768,
        speed: 30,
        persistent: false,
        enable_views: false,
        clear_view_background: false,
        clear_display_buffer: false,
        color: 0xc0c0c0,
        tiled: None,
        instances: instances.clone(),
      },
    )
  }
}

impl Default for InstanceItem {
  fn default() -> Self {
    InstanceItem::Unresolved(ResourceName::Name(String::from("")))
  }
}
