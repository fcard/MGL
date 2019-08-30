use crate::tests::utility::*;
use crate::resources::resolve::*;
use crate::resources::project::*;
use crate::resources::room::*;
use crate::resources::instance::*;
use crate::resources::resource_trait::*;

#[test]
fn test_resources_resolve_room() {
  let room_source = r#"
    room r {
      instances[0]: i
    }
  "#;

  let instance_source = r#"
    instance i of o {}
  "#;

  let room = Room::new((resource(room_source), vec![])).unwrap();
  let inst = Instance::new(instance(instance_source)).unwrap();

  let tree = Item::Group(String::from("g"), vec![
    Item::File(rn!(r), room.clone()),
  ]);

  let mut map = InstanceMap::new();
  map.insert(rn!(i), inst.clone());

  let result = resolve_room_item(&map, tree);

  if let Ok(Item::Group(_, items)) = result {
    if let Item::File(_, room) = &items[0] {
      assert_eq!(
        room.instances[0],
        InstanceItem::Resolved(rn!(i), inst)
      );
      return;
    }
  }
  unreachable!()
}

#[test]
fn test_resources_resolve_room_error() {
  let room_source = r#"
    room r {
      instances[0]: i
      instances[1]: j
    }
  "#;
  let room = Room::new((resource(room_source), vec![])).unwrap();

  let tree = Item::Group(String::from("g"), vec![
    Item::File(rn!(r), room.clone()),
  ]);

  let map = InstanceMap::new();

  let result = resolve_room_item(&map, tree);

  assert_eq!(
    result,
    Err(vec![
        MglError::InstanceNotFound(rn!(i)),
        MglError::InstanceNotFound(rn!(j)),
    ])
  )
}



