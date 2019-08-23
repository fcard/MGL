use crate::ast::*;
use crate::tests::utility::*;

macro assert_parse_statement {
  ($code: expr, $ast: expr) => {{
    assert_eq!(*statement($code).content, $ast)
  }}
}

#[test]
fn test_statement_return() {
  assert_parse_statement!("return a + b", Statement::return_op(expr("a + b")))
}

#[test]
fn test_statement_call() {
  assert_parse_statement!("f()",    Statement::call(expr("f()")));
  assert_parse_statement!("f(a)",   Statement::call(expr("f(a)")));
  assert_parse_statement!("g(x,y)", Statement::call(expr("g(x,y)")));
}

#[test]
fn test_statement_body() {
  assert_parse_statement!("{}", Statement::body(&[]));

  assert_parse_statement!("{
      print(\"Hello World\")
      return 0
    }",
    Statement::body(&[
      statement("print(\"Hello World\")"),
      statement("return 0"),
    ])
  );
}

#[test]
fn test_statement_with() {
  assert_parse_statement!(
    "with sprite::hello {
       do_thing()
     }",

    Statement::with(
      expr("sprite::hello"),
      statement("{
        do_thing()
      }")
    )
  )
}


#[test]
fn test_statement_if() {
  assert_parse_statement!(
    "if a == b {
       then()
     }",

    Statement::if_op(
      expr("a == b"),
      statement("{
        then()
      }"),
      None
    )
  )
}

#[test]
fn test_statement_if_else() {
  assert_parse_statement!(
    "if a != b {
       a_not_b()

     } else {
       a_is_b()
     }",

    Statement::if_op(
      expr("a != b"),
      statement("{
        a_not_b()
      }"),
      Some(statement("{
        a_is_b()
      }"))
    )
  )
}


#[test]
fn test_statement_while() {
  assert_parse_statement!(
    "while true {
       forever()
     }",

    Statement::while_op(
      expr("true"),
      statement("{
        forever()
      }")
    )
  )
}

#[test]
fn test_statement_for() {
  assert_parse_statement!(
    "for element in array {
       print(element)
     }",

    Statement::for_op(
      "element",
      ForRange::array(expr("array")),
      statement("{
        print(element)
      }")
    )
  );

  assert_parse_statement!(
    "for i in 0..10 {
       print(i)
     }",

    Statement::for_op(
      "i",
      ForRange::integer(expr("0"), expr("10"), None),
      statement("{
        print(i)
      }")
    )
  );

  assert_parse_statement!(
    "for j in 0..20 by 2 {
       print(j/2)
     }",

    Statement::for_op(
      "j",
      ForRange::integer(expr("0"), expr("20"), Some(expr("2"))),
      statement("{
        print(j/2)
      }")
    )
  );
}

#[test]
fn test_statement_assignment() {
  assert_parse_statement!("a = 2", Statement::assignment(expr("a"), expr("2")));
  assert_parse_statement!("g[0] = a + b", Statement::assignment(expr("g[0]"), expr("a + b")));
}

#[test]
fn test_statement_var() {
  assert_parse_statement!(
    "var a",
    Statement::var(&[VarDeclaration::name("a")])
  );

  assert_parse_statement!(
    "var a, b",
    Statement::var(&[VarDeclaration::name("a"), VarDeclaration::name("b")])
  );

  assert_parse_statement!(
    "var x = 1",
    Statement::var(&[VarDeclaration::assignment("x", expr("1"))])
  );

  assert_parse_statement!(
    "var x = 1, y",
    Statement::var(&[
      VarDeclaration::assignment("x", expr("1")),
      VarDeclaration::name("y"),
    ])
  );

  assert_parse_statement!(
    "var x = a + b, y",
    Statement::var(&[
      VarDeclaration::assignment("x", expr("a + b")),
      VarDeclaration::name("y"),
    ])
  );

  assert_parse_statement!(
    "var x, y = a + b",
    Statement::var(&[
      VarDeclaration::name("x"),
      VarDeclaration::assignment("y", expr("a + b")),
    ])
  );

  assert_parse_statement!(
    "var x = 1, y = 2, z = 3",
    Statement::var(&[
      VarDeclaration::assignment("x", expr("1")),
      VarDeclaration::assignment("y", expr("2")),
      VarDeclaration::assignment("z", expr("3")),
    ])
  );

  assert_parse_statement!(
    "var x = a + b, y = b + c, z = c + d",
    Statement::var(&[
      VarDeclaration::assignment("x", expr("a + b")),
      VarDeclaration::assignment("y", expr("b + c")),
      VarDeclaration::assignment("z", expr("c + d")),
    ])
  );
}

