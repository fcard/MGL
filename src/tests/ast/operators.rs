use crate::ast::*;

macro assert_operator_name($T: ty, $op: expr) {
  assert_eq!(<$T>::from_str($op).as_str(), $op)
}

#[test]
fn test_ast_operator_names() {
  assert_operator_name!(BinaryOp, ".");
  assert_operator_name!(BinaryOp, "+");
  assert_operator_name!(BinaryOp, "-");
  assert_operator_name!(BinaryOp, "*");
  assert_operator_name!(BinaryOp, "/");
  assert_operator_name!(BinaryOp, "||");
  assert_operator_name!(BinaryOp, "&&");
  assert_operator_name!(BinaryOp, "<");
  assert_operator_name!(BinaryOp, ">");
  assert_operator_name!(BinaryOp, ">=");
  assert_operator_name!(BinaryOp, "<=");
  assert_operator_name!(BinaryOp, "==");
  assert_operator_name!(BinaryOp, "!=");
  assert_operator_name!(UnaryOp,  "-");
  assert_operator_name!(UnaryOp,  "!");
  assert_operator_name!(Accessor, "");
  assert_operator_name!(Accessor, "|");
  assert_operator_name!(Accessor, "?");
  assert_operator_name!(Accessor, "#");
  assert_operator_name!(Accessor, "@");
}
