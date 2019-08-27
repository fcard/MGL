use crate::ast::*;

#[test]
fn test_ast_info_display() {
  assert_eq!(
    format!("{:?}", Expression::name("a")),
    format!("{:?}", IExpr::new(Expression::name("a"))),
  )
}

