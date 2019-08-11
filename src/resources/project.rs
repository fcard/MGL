use crate::resources::sprite::*;

type Items<T> = Vec<Item<T>>;

#[derive(Debug, Clone)]
pub struct Project {
  sprites: Items<Sprite>,
  scripts: Items<Script>,
  objects: Items<Object>,
  sounds:  Items<Sound>,
  fonts:   Items<Font>,
  rooms:   Items<Room>,
}

#[derive(Debug, Clone)]
pub enum Item<F> {
  File(String, F),
  Group(String, Vec<Item<F>>)
}


#[derive(Debug, Clone)]
struct Script;

#[derive(Debug, Clone)]
struct Object;

#[derive(Debug, Clone)]
struct Sound;

#[derive(Debug, Clone)]
struct Font;

#[derive(Debug, Clone)]
struct Room;

