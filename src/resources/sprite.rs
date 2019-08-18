#![allow(dead_code)]

use crate::ast::*;
use crate::resources::resource_trait::*;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Sprite {
  collision_kind: CollisionKind,
  collision_tolerance: u8,
  separate_masks: bool,
  width: u64,
  height: u64,

  #[sub_resource]
  origin: SpriteOrigin,

  #[array_field]
  #[sub_resource]
  frames: Vec<Frame>,

  #[sub_resource]
  bounding_box: BoundingBox,

  #[sub_resource]
  texture: Texture,
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct BoundingBox {
  mode: BoundingBoxMode,
  left: i64,
  right: i64,
  top: i64,
  bottom: i64
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct SpriteOrigin {
  x: i64,
  y: i64,
  center: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub struct Texture {
  horizontal: bool,
  vertical: bool,
  used_for_3d: bool,
  texture_group: usize
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct Frame {
  data: Option<PathBuf>
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

