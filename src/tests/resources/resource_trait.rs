use crate::tests::utility::*;
use crate::ast::*;
use crate::error::*;
use crate::resources::resource_trait::*;

#[derive(PartialEq, Default, Resource)]
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

#[derive(PartialEq, Default, Resource)]
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

