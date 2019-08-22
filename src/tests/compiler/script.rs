use crate::tests::utility::*;
use crate::compiler::script::*;
use crate::resources::script::*;

fn script(source: &str) -> String {
  build_script(Script::global(function(source)))
}

fn func(body: &str) -> String {
  format!("function f() {{{}\n}}", body)
}

fn assert_script_eq(source: &str, expected: &str) {
  assert_eq!(script(source).trim(), expected.trim());
}

const RESULT_ARGS: &str = r#"
var a = argument0;
var b = argument1;
"#;

#[test]
fn test_compiler_script_arguments() {
  assert_script_eq("function f(a,b) {}", RESULT_ARGS);
}

#[test]
fn test_compiler_script_return() {
  assert_script_eq("function f() {return 0\n}", "return 0;");
}

#[test]
fn test_compiler_script_expressions() {
  let ex = |e: &str| format!("function f() {{return {}\n}}", e);
  assert_script_eq(&ex("\"abc\""), "return \"abc\";");
  assert_script_eq(&ex("0"),       "return 0;");
  assert_script_eq(&ex("true"),    "return true;");
  assert_script_eq(&ex("false"),   "return false;");
  assert_script_eq(&ex("abc"),     "return abc;");
  assert_script_eq(&ex("m::n"),    "return m__n;");
  assert_script_eq(&ex("(x)"),     "return (x);");
  assert_script_eq(&ex("-x"),      "return -x;");
  assert_script_eq(&ex("x + y"),   "return x + y;");
  assert_script_eq(&ex("x.y"),     "return x.y;");
  assert_script_eq(&ex("f(x,y)"),  "return f(x, y);");
  assert_script_eq(&ex("a[@x]"),   "return a[@x];");
}

#[test]
fn test_compiler_script_call() {
  assert_script_eq(&func("print(x)"), "print(x);");
}

#[test]
fn test_compiler_script_assign() {
  assert_script_eq(&func("a = b"), "a = b;");
}

#[test]
fn test_compiler_script_var() {
  assert_script_eq(&func("var a"),        "var a;");
  assert_script_eq(&func("var a, b"),     "var a, b;");
  assert_script_eq(&func("var a = 1, b"), "var a = 1, b;");
}

const RESULT_WITH: &str = r#"
with obj {
    print(obj.x);
}
"#;

#[test]
fn test_compiler_script_with() {
  assert_script_eq(&func("with obj {print(obj.x)\n}"), RESULT_WITH);
}


const RESULT_IF: &str = r#"
if a > b {
    return a - b;
}
"#;

const RESULT_IF_ELSE: &str = r#"
if k {
    f(x);
} else {
    f(y);
}
"#;

#[test]
fn test_compiler_script_if() {
  assert_script_eq(&func("if a > b {return a - b\n}"), RESULT_IF);
  assert_script_eq(&func("if k {f(x)\n} else {f(y)\n}"), RESULT_IF_ELSE);
}

const RESULT_WHILE: &str = r#"
while i <= len {
    i = i + 1;
}
"#;

#[test]
fn test_compiler_script_while() {
  assert_script_eq(&func("while i <= len {i = i + 1\n}"), RESULT_WHILE);
}

const RESULT_FOR: &str = r#"
for (var i = 0; i != 10; i += 1) {
    print(i);
}
"#;

#[test]
fn test_compiler_script_for() {
  assert_script_eq(&func("for i in 0..10 by 1 {print(i)\n}"), RESULT_FOR);
}

