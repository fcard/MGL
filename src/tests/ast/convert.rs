use crate::ast::*;
use crate::error::*;
use crate::resources::sprite::*;
use crate::resources::sound::*;
use crate::resources::room::*;
use crate::tests::utility::*;
use std::path::PathBuf;
use std::convert::TryFrom;

#[test]
fn test_ast_convert_string() {
  let ok  = String::try_from(expr("\"abc\""));
  let err = String::try_from(expr("0"));

  assert_eq!(ok,  Ok(String::from("abc")));
  assert_eq!(err, MglError::convert_expression(expr("0"), "String"));
}

#[test]
fn test_ast_convert_pathbaf() {
  let ok  = PathBuf::try_from(expr("\"abc\""));
  let err = PathBuf::try_from(expr("0"));

  assert_eq!(ok,  Ok(PathBuf::from("abc")));
  assert_eq!(err, MglError::convert_expression(expr("0"), "PathBuf"));
}

#[test]
fn test_ast_convert_bool() {
  let ok  = bool::try_from(expr("true"));
  let err = bool::try_from(expr("0"));

  assert_eq!(ok,  Ok(true));
  assert_eq!(err, MglError::convert_expression(expr("0"), "bool"));
}

#[test]
fn test_ast_convert_resource_name() {
  let ok1 = ResourceName::try_from(expr("name"));
  let ok2 = ResourceName::try_from(expr("name::sub"));
  let err = ResourceName::try_from(expr("0"));

  assert_eq!(ok1, Ok(ResourceName::new(&["name"])));
  assert_eq!(ok2, Ok(ResourceName::new(&["name", "sub"])));
  assert_eq!(err, MglError::convert_expression(expr("0"), "ResourceName"));
}

#[test]
fn test_ast_convert_instance_item() {
  let ok  = InstanceItem::try_from(expr("name"));
  let err = InstanceItem::try_from(expr("0"));

  assert_eq!(ok,  Ok(InstanceItem::Unresolved(ResourceName::new(&["name"]))));
  assert_eq!(err, MglError::convert_expression(expr("0"), "ResourceName"));
}

#[test]
fn test_ast_convert_collision_kind() {
  let ok1  = CollisionKind::try_from(expr("\"precise\""));
  let ok2  = CollisionKind::try_from(expr("\"rectangle\""));
  let err1 = CollisionKind::try_from(expr("\"unknown\""));
  let err2 = CollisionKind::try_from(expr("0"));

  assert_eq!(ok1,  Ok(CollisionKind::Precise));
  assert_eq!(ok2,  Ok(CollisionKind::Rectangle));
  assert_eq!(err1, MglError::unknown_option("unknown", &["precise","rectangle"]));
  assert_eq!(err2, MglError::convert_expression(expr("0"), "String"));
}

#[test]
fn test_ast_convert_bounding_box_mode() {
  let ok1  = BoundingBoxMode::try_from(expr("\"automatic\""));
  let ok2  = BoundingBoxMode::try_from(expr("\"full_image\""));
  let ok3  = BoundingBoxMode::try_from(expr("\"manual\""));
  let err1 = BoundingBoxMode::try_from(expr("\"unknown\""));
  let err2 = BoundingBoxMode::try_from(expr("0"));

  assert_eq!(ok1,  Ok(BoundingBoxMode::Automatic));
  assert_eq!(ok2,  Ok(BoundingBoxMode::FullImage));
  assert_eq!(ok3,  Ok(BoundingBoxMode::Manual));
  assert_eq!(err1, MglError::unknown_option("unknown", &["automatic","full_image","manual"]));
  assert_eq!(err2, MglError::convert_expression(expr("0"), "String"));
}

#[test]
fn test_ast_convert_sound_kind() {
  let options = ["normal", "background", "3d", "external"];

  let ok1  = SoundKind::try_from(expr("\"normal\""));
  let ok2  = SoundKind::try_from(expr("\"background\""));
  let ok3  = SoundKind::try_from(expr("\"3d\""));
  let ok4  = SoundKind::try_from(expr("\"external\""));
  let err1 = SoundKind::try_from(expr("\"unknown\""));
  let err2 = SoundKind::try_from(expr("0"));

  assert_eq!(ok1,  Ok(SoundKind::Normal));
  assert_eq!(ok2,  Ok(SoundKind::Background));
  assert_eq!(ok3,  Ok(SoundKind::ThreeDimensional));
  assert_eq!(ok4,  Ok(SoundKind::ExternalPlayer));
  assert_eq!(err1, MglError::unknown_option("unknown", &options));
  assert_eq!(err2, MglError::convert_expression(expr("0"), "String"));
}

macro test_ast_convert_number($T: ty, $ok: expr) {{
  let t   = format!("number ({})", stringify!($T));
  let ok  = <$T>::try_from(expr(stringify!($ok)));
  let err = <$T>::try_from(expr("\"0\""));

  assert_eq!(ok,  Ok($ok));
  assert_eq!(err, MglError::convert_expression(expr("\"0\""), &t));
}}

#[test]
fn test_ast_convert_number() {
  test_ast_convert_number!(u8,  0);
  test_ast_convert_number!(u32, 0);
  test_ast_convert_number!(u64, 0);
  test_ast_convert_number!(i32, 0);
  test_ast_convert_number!(i64, 0);
  test_ast_convert_number!(f32, 0.0);
  test_ast_convert_number!(f64, 0.0);
  test_ast_convert_number!(usize, 0);
}

#[test]
fn test_ast_convert_option_pathbuf() {
  let ok  = <Option<PathBuf>>::try_from(expr("\"abc\""));
  let err = <Option<PathBuf>>::try_from(expr("0"));

  assert_eq!(ok,  Ok(Some(PathBuf::from("abc"))));
  assert_eq!(err, MglError::convert_expression(expr("0"), "PathBuf"));
}

#[test]
fn test_ast_convert_option_resource_name() {
  let ok  = <Option<ResourceName>>::try_from(expr("name"));
  let err = <Option<ResourceName>>::try_from(expr("0"));

  assert_eq!(ok,  Ok(Some(ResourceName::new(&["name"]))));
  assert_eq!(err, MglError::convert_expression(expr("0"), "ResourceName"));
}

