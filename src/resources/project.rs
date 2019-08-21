use std::collections::HashMap;

use crate::ast::*;
use crate::error::*;
use crate::compiler::file_reader::*;
use crate::resources::instance::Instance;
use crate::resources::sprite::Sprite;
use crate::resources::script::Script;
use crate::resources::object::Object;
use crate::resources::sound::Sound;
use crate::resources::room::{Room, InstanceItem, InstanceItems};
use crate::resources::resource_trait::*;

pub type Items<T> = Vec<Item<T>>;
pub type Module = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
  sprites: Items<Sprite>,
  scripts: Items<Script>,
  objects: Items<Object>,
  sounds:  Items<Sound>,
  fonts:   Items<Font>,
  rooms:   Items<Room>,
  module:  Module,

  instances: HashMap<ResourceName, Instance>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item<F> {
  File(ResourceName, F),
  Group(String, Vec<Item<F>>)
}


#[derive(Debug, Clone, PartialEq)]
struct Font;

impl Project {
  pub fn new(m: Module) -> Project {
    Project {
      sprites:   Items::new(),
      scripts:   Items::new(),
      objects:   Items::new(),
      sounds:    Items::new(),
      fonts:     Items::new(),
      rooms:     Items::new(),
      instances: HashMap::new(),
      module:    m,
    }
  }

  pub fn from_ast_file_tree(tree: AstFileTree, m: Module) -> TopResult<Project> {
    let mut errors  = Vec::new();
    let mut project = Project::new(m.clone());
    let is_root = tree.is_root();

    match tree {
      AstFileTree::Leaf(_, file) => {
        for declaration in file.declarations {
          if let Err(e) = project.parse_declaration(declaration) {
            errors.push(e);
          }
        }
      }
      AstFileTree::Node(directory, files) |
      AstFileTree::Root(box AstFileTree::Node(directory, files)) => {
        let mut sprites = Vec::new();
        let mut scripts = Vec::new();
        let mut objects = Vec::new();
        let mut sounds  = Vec::new();
        let mut fonts   = Vec::new();
        let mut rooms   = Vec::new();

        let from_subtree = |t: &AstFileTree| {
          Project::from_ast_file_tree(t.clone(), m.clone())
        };

        for subproject in files.iter().map(from_subtree) {
          match subproject {
            Ok(mut subproject) => {
              sprites.append(&mut subproject.sprites);
              scripts.append(&mut subproject.scripts);
              objects.append(&mut subproject.objects);
              sounds.append(&mut subproject.sounds);
              fonts.append(&mut subproject.fonts);
              rooms.append(&mut subproject.rooms);
            }
            Err(mut e) => {
              errors.append(&mut e);
            }
          }
        }

        if is_root {
          project.sprites = sprites;
          project.scripts = scripts;
          project.objects = objects;
          project.sounds  = sounds;
          project.fonts   = fonts;
          project.rooms   = rooms;

        } else {
          if !sprites.is_empty() { project.sprites.push(Item::Group(directory.clone(), sprites)); }
          if !scripts.is_empty() { project.scripts.push(Item::Group(directory.clone(), scripts)); }
          if !objects.is_empty() { project.objects.push(Item::Group(directory.clone(), objects)); }
          if !sounds.is_empty()  { project.sounds.push(Item::Group(directory.clone(), sounds)); }
          if !fonts.is_empty()   { project.fonts.push(Item::Group(directory.clone(), fonts)); }
          if !rooms.is_empty()   { project.rooms.push(Item::Group(directory.clone(), rooms)); }
        }
      }
      _ => unreachable!()
    }
    if errors.is_empty() {
      Ok(project)
    } else {
      Err(errors)
    }
  }

  pub fn parse_declaration(&mut self, declaration: Declaration) -> Result<()> {
    match declaration {
      Declaration::Function(function) => {
        self.parse_function(function)?;
      }

      Declaration::Resource(resource) => {
        self.parse_resource(resource)?;
      }

      Declaration::Instance(instance) => {
        self.parse_instance(instance)?;
      }
    }
    Ok(())
  }

  pub fn parse_resource(&mut self, resource: ResourceDeclaration) -> Result<()> {
    let mut instances = InstanceItems::new();
    let resource_name = full_name(&resource, &self.module);
    let sub_module = module_add(&self.module, resource.name.clone());

    for method_ast in &resource.methods {
      let method      = Script::method(method_ast.clone(), resource_name.clone());
      let method_name = full_name_for("script", &method_ast.name, &sub_module);
      self.scripts.push(Item::File(method_name, method));
    }

    for instance_ast in &resource.instances {
      let instance      = Instance::new(instance_ast.clone())?;
      let instance_name = full_name_for("instance", &instance_ast.name, &sub_module);
      self.instances.insert(instance_name, instance.clone());

      if resource.kind == ResourceKind::Room {
        instances.push(InstanceItem::Resolved(instance));
      }
    }

    macro add_item {
      ($field: ident, $file: expr) => {
        self.$field.push(Item::File(resource_name, $file))
      }
    };

    match resource.kind {
      ResourceKind::Object   => add_item!(objects, Object::new(resource)?),
      ResourceKind::Wrapper  => add_item!(objects, Object::new(resource)?),
      ResourceKind::Sprite   => add_item!(sprites, Sprite::new(resource)?),
      ResourceKind::Sound    => add_item!(sounds,  Sound::new(resource)?),
      ResourceKind::Room     => add_item!(rooms,   Room::new((resource, instances))?),
    }
    Ok(())
  }

  pub fn parse_function(&mut self, function: FunctionDeclaration) -> Result<()> {
    let resource_name = full_name_for("script", &function.name, &self.module);
    self.scripts.push(Item::File(resource_name, Script::global(function)));
    Ok(())
  }

  pub fn parse_instance(&mut self, declaration: InstanceDeclaration) -> Result<()>  {
    let instance_name = full_name_for("instance", &declaration.name, &self.module);
    self.instances.insert(instance_name, Instance::new(declaration)?);
    Ok(())
  }
}

pub fn module_add(module: &Module, addition: String) -> Module {
  let mut new_module = module.clone();
  new_module.push(addition);
  new_module
}

pub fn full_name_for(kind_module: &str, name: &str, module: &Module) -> ResourceName {
  let mut names: Vec<_> = module.iter().map(String::as_ref).collect();
  names.push(name);
  names.insert(0, kind_module);
  ResourceName::new(&names)
}

pub fn full_name(resource: &ResourceDeclaration, m: &Module) -> ResourceName {
  full_name_for(&resource.kind.module(), &resource.name, m)
}
