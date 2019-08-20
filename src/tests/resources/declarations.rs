use std::path::PathBuf;
use crate::ast::*;
use crate::error::*;
use crate::tests::utility::*;
use crate::resources::resource_trait::*;
use crate::resources::sprite::*;
use crate::resources::sound::*;
use crate::resources::room::*;
use crate::resources::script::*;

#[test]
fn test_resources_sprite() {
  let s = Sprite::new(resource(r#"
    sprite s {
      collision_kind: "rectangle"
      collision_tolerance: 10
      separate_masks: true
      width: 100
      height: 100

      origin.x: 1
      origin.y: 1
      origin.center: true

      frames[0].data: "images/s.png"

      bounding_box.mode: "manual"
      bounding_box.left:   1
      bounding_box.right:  1
      bounding_box.top:    1
      bounding_box.bottom: 1

      texture.horizontal:    true
      texture.vertical:      true
      texture.used_for_3d:   true
      texture.texture_group: 1
    }
  "#)).unwrap();

  assert_eq!(s.collision_kind, CollisionKind::Rectangle);
  assert_eq!(s.collision_tolerance, 10);
  assert_eq!(s.separate_masks, true);
  assert_eq!(s.width, 100);
  assert_eq!(s.height, 100);
  assert_eq!(s.origin.x, 1);
  assert_eq!(s.origin.y, 1);
  assert_eq!(s.origin.center, true);
  assert_eq!(s.frames[0].data, Some(PathBuf::from("images/s.png")));
  assert_eq!(s.bounding_box.mode, BoundingBoxMode::Manual);
  assert_eq!(s.bounding_box.left, 1);
  assert_eq!(s.bounding_box.right, 1);
  assert_eq!(s.bounding_box.top, 1);
  assert_eq!(s.bounding_box.bottom, 1);
  assert_eq!(s.texture.horizontal, true);
  assert_eq!(s.texture.vertical, true);
  assert_eq!(s.texture.used_for_3d, true);
  assert_eq!(s.texture.texture_group, 1);

  let e = Sprite::new(resource("sprite e { k: 1\n }"));
  assert_eq!(e, MglError::invalid_field("k",  InvalidFieldKind::NotFound));
}


#[test]
fn test_resources_sound() {
  let mut s = Sound::new(resource(r#"
    sound s {
      kind: "background"
      data: "sounds/s.wav"
      volume: 0.5
      pan: 0.5
      bit_rate: 1000
      sample_rate: 88000
      bit_depth: 32
      preload: true
      compress: true
      uncompress_on_load: true
      audio_group: 1
    }
  "#)).unwrap();

  assert_eq!(s.kind, SoundKind::Background);
  assert_eq!(s.data, PathBuf::from("sounds/s.wav"));
  assert_eq!(s.volume, 0.5);
  assert_eq!(s.pan, 0.5);
  assert_eq!(s.bit_rate, 1000);
  assert_eq!(s.sample_rate, 88000);
  assert_eq!(s.bit_depth, 32);
  assert_eq!(s.preload, true);
  assert_eq!(s.compress, true);
  assert_eq!(s.uncompress_on_load, true);
  assert_eq!(s.audio_group, 1);

  let e = Sound::new(resource("sound e { k: 1\n }"));
  assert_eq!(e, MglError::invalid_field("k",  InvalidFieldKind::NotFound));
}

#[test]
fn test_resources_room() {
  let r = Room::new((resource(r#"
    room r {
      width: 100
      height: 100
      speed: 60
      persistent: true
      enable_views: true
      clear_view_background: true
      clear_display_buffer: true
      color: 0
      tiled: "rooms/r.tmx"
      instances[0]: obj::inst
    }
  "#), Vec::new())).unwrap();

  assert_eq!(r.width, 100);
  assert_eq!(r.height, 100);
  assert_eq!(r.speed, 60);
  assert_eq!(r.persistent, true);
  assert_eq!(r.enable_views, true);
  assert_eq!(r.clear_view_background, true);
  assert_eq!(r.clear_display_buffer, true);
  assert_eq!(r.color, 0);
  assert_eq!(r.tiled, Some(PathBuf::from("rooms/r.tmx")));
  assert_eq!(r.instances[0], InstanceItem::Unresolved(ResourceName::new(&["obj", "inst"])));

  let e = Room::new((resource("room e { k: 1\n }"), Vec::new()));
  assert_eq!(e, MglError::invalid_field("k",  InvalidFieldKind::NotFound));
}

#[test]
fn test_resource_script() {
  let f = Script::global(function(r#"
    function f(x) {
      return x + 1
    }
  "#));

  let m = Script::method(function(r#"
    function m(x,y) {
      return x + y
    }
  "#), ResourceName::new(&["o"]));

  assert_eq!(f.kind, ScriptKind::Global);
  assert_eq!(f.source, function("function f(x) { return x + 1\n }"));

  assert_eq!(m.kind, ScriptKind::Method(ResourceName::new(&["o"])));
  assert_eq!(m.source, function("function m(x,y) { return x + y\n }"));
}
