use std::collections::HashMap;

use crate::utility::try_map::TryMap;
use crate::ast::*;
use crate::error::*;
use crate::resources::project::*;
use crate::resources::room::*;
use crate::resources::instance::*;

type InstanceMap = HashMap<ResourceName, Instance>;

pub fn resolve_room_item(instances: &InstanceMap, item: Item<Room>) -> TopResult<Item<Room>> {
  match item {
    Item::File(name,  room)  => Ok(Item::File(name, resolve_room(instances, room)?)),
    Item::Group(name, rooms) => Ok(Item::Group(name, resolve_room_items(instances, rooms)?)),
  }
}

pub fn resolve_room_items(instances: &InstanceMap, items: Items<Room>) -> TopResult<Items<Room>> {
  items.try_map(|item| resolve_room_item(instances, item))
       .map_err(|errors| errors.into_iter().flatten().collect())
}

pub fn resolve_room(instances: &InstanceMap, room: Room) -> TopResult<Room> {
  let mut result = room.clone();
  result.instances = room.instances.clone().try_map(|instance_item| {
    match instance_item {
      InstanceItem::Resolved(_,_)    => Ok(instance_item.clone()),
      InstanceItem::Unresolved(name) => {
        if let Some(instance) = instances.get(&name) {
          Ok(InstanceItem::Resolved(name.clone(), instance.clone()))

        } else {
          Err(MglError::InstanceNotFound(name.clone()))
        }
      }
    }
  })?;
  Ok(result)
}
