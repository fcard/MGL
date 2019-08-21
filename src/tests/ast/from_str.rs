use crate::ast::*;
use crate::error::*;
use crate::tests::utility::*;

#[test]
fn test_ast_from_str_resource_declaration() {
  let ok  = "object a {}".parse::<ResourceDeclaration>();
  let err = "function f(x) {}".parse::<ResourceDeclaration>();

  assert_eq!(ok,  Ok(ResourceDeclaration::new(ResourceKind::Object, "a", &[], &[], &[])));
  assert_eq!(err, Err(MglError::NotResource));
}

#[test]
fn test_ast_from_str_declaration() {
  let ok  = "instance b of a {}".parse::<Declaration>();
  let err = "object a {}\n object z {}".parse::<Declaration>();

  assert_eq!(ok,  Ok(Declaration::Instance(InstanceDeclaration::new(expr("a"), "b", &[]))));
  assert_eq!(err, Err(MglError::NotResource));
}

