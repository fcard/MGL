use crate::tests::utility::*;
use crate::ast::*;
use crate::error::*;
use crate::compiler::file_reader::*;
use crate::resources::project::*;
use crate::resources::object::*;
use crate::resources::script::*;
use crate::resources::sprite::*;
use crate::resources::sound::*;
use crate::resources::room::*;
use crate::resources::instance::*;
use crate::resources::resource_trait::*;

fn item<T>(name: ResourceName,res: Result<T>) -> Item<T> {
  Item::File(name, res.unwrap())
}

fn project(file1: &str, file2: &str) -> TopResult<Project> {
  let mut top_files = Vec::new();
  let mut sub_files = Vec::new();

  sub_files.push(AstFileTree::Leaf(String::from("file2"), file2.parse().unwrap()));
  let subdirectory = AstFileTree::Node(String::from("subdir"), sub_files);

  top_files.push(AstFileTree::Leaf(String::from("file1"), file1.parse().unwrap()));
  top_files.push(subdirectory);

  let root = AstFileTree::Root(box AstFileTree::Node(String::new(), top_files));
  Project::from_ast_file_tree(root, Module::new())
}

#[test]
fn test_resources_project() {
  let meth1 = "
    function m(a) {
      return a
    }
  ";

  let obj1 = format!("
    object o {{
      {}
    }}
  ", meth1);

  let wrap1 = "
    wrapper w {
      instance wi of o {}
    }
  ";

  let spr1  = "sprite    s      {}";
  let snd1  = "sound     s      {}";
  let ins1  = "instance  i of o {}";
  let func1 = "function  f()    {}";

  let rins1 = "instance ri of o {}";
  let room1 = format!("
    room r {{
      {}
    }}
  ", rins1);


  let file1 =
    format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            obj1, wrap1, spr1, snd1, ins1, func1, room1);


  let file2 = r#"
    object   do  {}
    wrapper  dw  {}
    sprite   ds  {}
    sound    ds  {}
    room     dr  {}
    function f() {}
    instance di of do {}
  "#;

  let project = project(&file1, file2).unwrap();

  let obj  = Object::new(resource(&obj1));
  let wrap = Object::new(resource(wrap1));
  let meth = Script::method(function(meth1), rn!(object::o));
  let func = Script::global(function(func1));
  let spr  = Sprite::new(resource(spr1));
  let snd  = Sound::new(resource(snd1));
  let ins  = Instance::new(instance(ins1));
  let rins = Instance::new(instance(rins1));
  let room = Room::new((resource(&room1),
    vec![InstanceItem::Resolved(rn!(instance::r::ri), rins.clone().unwrap())]
  ));

  assert_eq!(project.objects[0], item(rn!(object::o),    obj));
  assert_eq!(project.objects[1], item(rn!(wrapper::w),   wrap));
  assert_eq!(project.scripts[0], item(rn!(script::o::m), Ok(meth)));
  assert_eq!(project.scripts[1], item(rn!(script::f),    Ok(func)));
  assert_eq!(project.sprites[0], item(rn!(sprite::s),    spr));
  assert_eq!(project.sounds[0],  item(rn!(sound::s),     snd));
  assert_eq!(project.rooms[0],   item(rn!(room::r),      room));
  assert_eq!(project.instances[&rn!(instance::i)],       ins.unwrap());
  assert_eq!(project.instances[&rn!(instance::r::ri)],   rins.unwrap());
}


#[test]
fn test_resources_project_error() {
  let file1   = "object o  {unknown: p\n}";
  let file2   = "object do {unknown: p\n}";
  let project = project(file1, file2);

  let err = MglError::InvalidField {
    kind: InvalidFieldKind::NotFound,
    field: String::from("unknown")
  };

  assert_eq!(project, Err(vec![err.clone(), err]));
}

