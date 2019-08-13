use crate::ast::*;
use crate::tests::utility::*;

macro assert_parse_expr {
  ($code: expr, $ast: expr) => {
    assert_eq!(expr($code), $ast)
  }
}


#[test]
fn test_expression_string() {
  assert_parse_expr!("\"hello world\"", Expression::string("hello world"))
}

#[test]
fn test_expression_num_integer() {
  assert_parse_expr!("12", Expression::num("12"))
}

#[test]
fn test_expression_num_float() {
  assert_parse_expr!("32.180", Expression::num("32.180"))
}

#[test]
fn test_expression_bool_true() {
  assert_parse_expr!("true", Expression::boolean(true))
}

#[test]
fn test_expression_bool_false() {
  assert_parse_expr!("false", Expression::boolean(false))
}

#[test]
fn test_expression_name_1() {
  assert_parse_expr!("x", Expression::name("x"))
}

#[test]
fn test_expression_name_2() {
  assert_parse_expr!("foo_bar123", Expression::name("foo_bar123"))
}

#[test]
fn test_expression_resource_1() {
  assert_parse_expr!("a::b", Expression::resource(&["a", "b"]))
}

#[test]
fn test_expression_resource_2() {
  assert_parse_expr!("a::b::c", Expression::resource(&["a", "b", "c"]))
}

#[test]
fn test_expression_parentheses() {
  assert_parse_expr!("(a)", Expression::parentheses(Expression::name("a")))
}

#[test]
fn test_expression_unary_not() {
  assert_parse_expr!("!a", Expression::unary_op(UnaryOp::Not, Expression::name("a")))
}

#[test]
fn test_expression_unary_neg() {
  assert_parse_expr!("-a", Expression::unary_op(UnaryOp::Neg, Expression::name("a")))
}

#[test]
fn test_expression_binary() {
  use BinaryOp::*;
  let a = Expression::name("a");
  let b = Expression::name("b");

  for (code, op) in
    [("a.b"   , Dot), ("a .  b", Dot),
     ("a +  b", Add), ("a -  b", Sub), ("a *  b", Mul), ("a /  b", Div ),
     ("a || b", Or ), ("a && b", And), ("a <  b", Lt ), ("a >  b", Gt  ),
     ("a >= b", Geq), ("a <= b", Leq), ("a == b", Eq ), ("a != b", Diff)].iter() {

    assert_parse_expr!(code, Expression::binary_op(*op, a.clone(), b.clone()))
  }
}

#[test]
fn test_expression_ternary() {
  assert_parse_expr!(
    "a ? b : c",
    Expression::ternary_op(
      Expression::name("a"),
      Expression::name("b"),
      Expression::name("c"),
    )
  )
}

#[test]
fn test_expression_call() {
  let f = Expression::name("f");
  let a = Expression::name("a");
  let b = Expression::name("b");

  let call = |args: Vec<&Expression>| {
    Expression::call(f.clone(), &args.clone_all())
  };

  assert_parse_expr!("f()",    call(vec![]));
  assert_parse_expr!("f(a)",   call(vec![&a]));
  assert_parse_expr!("f(a,b)", call(vec![&a, &b]))
}

#[test]
fn test_expression_indexing() {
  use Accessor::*;

  let a = Expression::name("a");
  let i = Expression::name("i");
  let j = Expression::name("j");

  let indexing = |acc, args: Vec<&Expression>| {
    Expression::indexing(a.clone(), acc, &args.clone_all())
  };

  assert_parse_expr!("a[i]",    indexing(None,  vec![&i]));
  assert_parse_expr!("a[i,j]",  indexing(None,  vec![&i, &j]));
  assert_parse_expr!("a[|i]",   indexing(List,  vec![&i]));
  assert_parse_expr!("a[|i,j]", indexing(List,  vec![&i, &j]));
  assert_parse_expr!("a[?i]",   indexing(Map,   vec![&i]));
  assert_parse_expr!("a[?i,j]", indexing(Map,   vec![&i, &j]));
  assert_parse_expr!("a[#i]",   indexing(Grid,  vec![&i]));
  assert_parse_expr!("a[#i,j]", indexing(Grid,  vec![&i, &j]));
  assert_parse_expr!("a[@i]",   indexing(Array, vec![&i]));
  assert_parse_expr!("a[@i,j]", indexing(Array, vec![&i, &j]));
}

#[test]
fn test_expression_multiple_ops() {
  let a = expr("a");
  let b = expr("b");
  let c = expr("c");
  let d = expr("d");
  let fx = expr("f(x)");

  let uny = |op, x: &Expression| {
    Expression::unary_op(op, (*x).clone())
  };

  let bin = |op, x: &Expression, y: &Expression| {
    Expression::binary_op(op, (*x).clone(), (*y).clone())
  };

  let ter = |x: &Expression, y: &Expression, z: &Expression| {
    Expression::ternary_op((*x).clone(), (*y).clone(), (*z).clone())
  };

  let par = |e: &Expression| {
    Expression::parentheses((*e).clone())
  };

  assert_parse_expr!("a + b + c", bin("+", &bin("+", &a, &b), &c));
  assert_parse_expr!("a - b - c", bin("-", &bin("-", &a, &b), &c));
  assert_parse_expr!("a * b + c", bin("+", &bin("*", &a, &b), &c));
  assert_parse_expr!("a + b * c", bin("+", &a, &bin("*", &b, &c)));
  assert_parse_expr!("a + b / c", bin("+", &a, &bin("/", &b, &c)));

  assert_parse_expr!("a > b  || c < d",  bin("||", &bin(">", &a, &b), &bin("<", &c, &d)));
  assert_parse_expr!("a != b && c == d", bin("&&", &bin("!=", &a, &b), &bin("==", &c, &d)));
  assert_parse_expr!("a >= b && c <= d", bin("&&", &bin(">=", &a, &b), &bin("<=", &c, &d)));

  assert_parse_expr!("a + f(x)", bin("+", &a, &fx));
  assert_parse_expr!("f(x) + a", bin("+", &fx, &a));

  assert_parse_expr!("!a.b",   uny("!", &bin(".", &a, &b)));
  assert_parse_expr!("!a + b", bin("+", &uny("!", &a), &b));
  assert_parse_expr!("a + !b", bin("+", &a, &uny("!", &b)));
  assert_parse_expr!("-a + b", bin("+", &uny("-", &a), &b));
  assert_parse_expr!("a + -b", bin("+", &a, &uny("-", &b)));

  assert_parse_expr!("(a + b) * c", bin("*", &par(&bin("+", &a, &b)), &c));
  assert_parse_expr!("a + (b * c)", bin("+", &a, &par(&bin("*", &b, &c))));

  assert_parse_expr!(
    "a + b * c + d",
    bin("+", &bin("+", &a, &bin("*", &b, &c)), &d)
  );

  assert_parse_expr!(
    "a * b + c * d",
    bin("+", &bin("*", &a, &b), &bin("*", &c, &d))
  );

  assert_parse_expr!("!a ? b : c", ter(&uny("!", &a), &b, &c));
  assert_parse_expr!("!(a ? b : c)", uny("!", &par(&ter(&a, &b, &c))));

  assert_parse_expr!("a + b ? c : d", ter(&bin("+", &a, &b), &c, &d));
  assert_parse_expr!("a + (b ? c : d)", bin("+", &a, &par(&ter(&b, &c, &d))));
}
