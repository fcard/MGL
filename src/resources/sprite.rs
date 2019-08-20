#![allow(dead_code)]

use crate::ast::*;
use crate::resources::resource_trait::*;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Sprite {
  pub collision_kind: CollisionKind,
  pub collision_tolerance: u8,
  pub separate_masks: bool,
  pub width: u64,
  pub height: u64,

  #[sub_resource]
  pub origin: SpriteOrigin,

  #[array_field]
  #[sub_resource]
  pub frames: Vec<Frame>,

  #[sub_resource]
  pub bounding_box: BoundingBox,

  #[sub_resource]
  pub texture: Texture,
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct BoundingBox {
  pub mode: BoundingBoxMode,
  pub left: i64,
  pub right: i64,
  pub top: i64,
  pub bottom: i64
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct SpriteOrigin {
  pub x: i64,
  pub y: i64,
  pub center: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct Texture {
  pub horizontal: bool,
  pub vertical: bool,
  pub used_for_3d: bool,
  pub texture_group: usize
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct Frame {
  pub data: Option<PathBuf>
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundingBoxMode {
  Automatic = 0,
  FullImage,
  Manual
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionKind {
  Precise = 0,
  Rectangle
}

impl ResourceDefault<ResourceDeclaration> for Sprite {
  fn default(_resource: &ResourceDeclaration) -> Result<Sprite> {
    Ok(
      Sprite {
        origin: SpriteOrigin::default(),
        collision_kind: CollisionKind::Precise,
        collision_tolerance: 0,
        separate_masks: false,
        bounding_box: BoundingBox::default(),
        texture: Texture::default(),
        width: 0,
        height: 0,
        frames: Vec::new(),
      },
    )
  }
}

impl Default for BoundingBox {
  fn default() -> BoundingBox {
    BoundingBox {
      mode: BoundingBoxMode::Automatic,
      left: 0,
      right: 0,
      top: 0,
      bottom: 0
    }
  }
}

impl Default for Texture {
  fn default() -> Texture {
    Texture {
      horizontal: false,
      vertical: false,
      used_for_3d: false,
      texture_group: 0
    }
  }
}

impl Default for SpriteOrigin {
  fn default() -> SpriteOrigin {
    SpriteOrigin {
      x: 0,
      y: 0,
      center: false,
    }
  }
}

