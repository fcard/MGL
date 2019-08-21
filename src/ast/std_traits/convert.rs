use crate::ast::*;
use crate::ast::std_traits::convert_macros::*;
use crate::resources::sprite::*;
use crate::resources::sound::*;
use crate::resources::room::*;
use std::path::PathBuf;

implement_match_try_from! {
  String  { Expression::Str(string)   => string },
  PathBuf { Expression::Str(string)   => PathBuf::from(string) },
  bool    { Expression::Bool(boolean) => boolean },

  ResourceName {
    Expression::Name(name)  => ResourceName::Name(name),
    Expression::Resource(r) => r
  },

  InstanceItem {
    name => InstanceItem::Unresolved(ResourceName::try_from(name)?)
  }
}


implement_try_from_string_options! {
  CollisionKind {
    "precise"   => CollisionKind::Precise,
    "rectangle" => CollisionKind::Rectangle
  },

  BoundingBoxMode {
    "automatic"  => BoundingBoxMode::Automatic,
    "full_image" => BoundingBoxMode::FullImage,
    "manual"     => BoundingBoxMode::Manual
  },

  SoundKind {
    "normal"     => SoundKind::Normal,
    "background" => SoundKind::Background,
    "3d"         => SoundKind::ThreeDimensional,
    "external"   => SoundKind::ExternalPlayer
  }
}

implement_try_from_for_numbers!(u8, u32, u64, i32, i64, f32, f64, usize);
implement_try_from_wrap_option!(PathBuf, ResourceName);

