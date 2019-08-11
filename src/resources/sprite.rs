#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Sprite {
  origin: SpriteOrigin,
  collision_kind: CollisionKind,
  collision_tolerance: u8,
  separate_masks: bool,
  bounding_box: BoundingBox,
  texture: Texture,
  width: u64,
  height: u64,
  frames: Vec<Frame>,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
  mode: BoundingBoxMode,
  left: i64,
  right: i64,
  top: i64,
  bottom: i64
}

#[derive(Debug, Clone, Copy)]
pub enum BoundingBoxMode {
  Automatic = 0,
  FullImage,
  Manual
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteOrigin {
  Coordinates(i64, i64),
  Center
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionKind {
  Precise = 0,
  Rectangle
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
  horizontal: bool,
  vertical: bool,
  used_for_3d: bool,
  texture_group: usize
}

#[derive(Debug, Clone)]
pub struct Frame {
  image_file: String
}

impl Sprite {
  pub fn new() -> Sprite {
    Sprite {
      origin: SpriteOrigin::Coordinates(0, 0),
      collision_kind: CollisionKind::Precise,
      collision_tolerance: 0,
      separate_masks: false,
      bounding_box: BoundingBox::new(),
      texture: Texture::new(),
      width: 0,
      height: 0,
      frames: Vec::new(),
    }
  }
}

impl BoundingBox {
  pub fn new() -> BoundingBox {
    BoundingBox {
      mode: BoundingBoxMode::Automatic,
     left: 0,
     right: 0,
     top: 0,
     bottom: 0
    }
  }
}

impl Texture {
  pub fn new() -> Texture {
    Texture {
      horizontal: false,
      vertical: false,
      used_for_3d: false,
      texture_group: 0
    }
  }
}

