use crate::ast::*;
use crate::resources::instance::*;
use crate::resources::resource_trait::*;
use std::path::PathBuf;

pub type InstanceItems = Vec<InstanceItem>;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Room {
  pub width: u64,
  pub height: u64,
  pub speed: u64,
  pub isometric: bool,
  pub horizontal_snap: usize,
  pub vertical_snap: usize,
  pub persistent: bool,
  pub show_color: bool,
  pub enable_views: bool,
  pub clear_view_background: bool,
  pub clear_display_buffer: bool,
  pub color: u32,
  pub tiled: Option<PathBuf>,

  #[array_field]
  pub instances: InstanceItems,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceItem {
  Resolved(ResourceName, Instance),
  Unresolved(ResourceName),
}

impl ResourceDefault<(ResourceDeclaration, InstanceItems)> for Room {
  fn default((_resource, instances): &(ResourceDeclaration, InstanceItems)) -> Result<Room> {
    Ok(
      Room {
        width: 1024,
        height: 768,
        speed: 30,
        isometric: false,
        horizontal_snap: 16,
        vertical_snap: 16,
        persistent: false,
        show_color: true,
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

