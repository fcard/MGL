use crate::tests::utility::*;
use crate::ast::*;
use crate::error::*;
use crate::resources::resource_trait::*;

#[derive(Debug, PartialEq, Default, Resource)]
struct ResourceTest {
  normal_field_1: i64,
  normal_field_2: String,

  #[array_field]
  array_field: Vec<u64>,

  #[sub_resource]
  sub_resource: SubResourceTest,

  #[array_field]
  #[sub_resource]
  sub_array_field: Vec<SubResourceTest>,

  #[ignore_field]
  other: i32,
}

#[derive(Debug, PartialEq, Default, Resource)]
struct SubResourceTest {
  sub_field: u32,
}

struct ResourceKeyValues(Vec<KeyValue>);

impl ResourceAst for ResourceKeyValues {
  fn key_values(&self) -> &[KeyValue] {
    &self.0
  }
}

impl ResourceDefault<ResourceKeyValues> for ResourceTest {
  fn default(_: &ResourceKeyValues) -> Result<Self> {
    Ok(Default::default())
  }
}

#[test]
fn test_resources_trait() {
  let ast = ResourceKeyValues(keys! {
    normal_field_1: 10,
    normal_field_2: "abc",
    array_field[1]: 1,
    array_field[2]: 2,
    sub_resource.sub_field: 3,
    sub_array_field[0].sub_field: 4,
    sub_array_field[5].sub_field: 5
  });
  let resource = ResourceTest::new(ast).unwrap();

  assert_eq!(resource.normal_field_1, 10);
  assert_eq!(resource.normal_field_2, String::from("abc"));
  assert_eq!(resource.array_field, [0,1,2]);
  assert_eq!(resource.sub_resource.sub_field, 3);
  assert_eq!(resource.sub_array_field.get(0).unwrap().sub_field, 4);
  assert_eq!(resource.sub_array_field.get(5).unwrap().sub_field, 5);
}

#[test]
fn test_resources_trait_fail() {
  use InvalidFieldKind::*;

  let resource = |keys| ResourceTest::new(ResourceKeyValues(keys));
  let err1 = resource(keys![unknown_field: 404]);
  let err2 = resource(keys![normal_field_1[0]: 1]);
  let err3 = resource(keys![array_field: 2]);
  let err4 = resource(keys![sub_resource: 3]);
  let err5 = resource(keys![normal_field_1: "a"]);

  assert_eq!(err1, MglError::invalid_field("unknown_field",  NotFound));
  assert_eq!(err2, MglError::invalid_field("normal_field_1", NotSimple(key("normal_field_1[0]"))));
  assert_eq!(err3, MglError::invalid_field("array_field",    NotArray(key("array_field"))));
  assert_eq!(err4, MglError::invalid_field("sub_resource",   NotSubResource(key("sub_resource"))));
  assert_eq!(err5, MglError::wrong_field_type(expr("\"a\""), "number (i64)", "normal_field_1"));
}


