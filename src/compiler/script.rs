use crate::ast::*;
use crate::resources::script::*;

struct StatementBuilder {
  root: bool,
  result: String,
  indentation: usize,
}

pub fn build_script(s: Script) -> String {
  let source = &s.source;

  let mut builder = StatementBuilder::new(true, 0);
  builder.argument_vars(&source.args);
  builder.build_statement(source.body.as_ref());

  format!("{}", builder.result)
}

fn build_expression<T: AsRef<Expression>>(e: &T) -> String {
  use crate::ast::BinaryOp::Dot;
  use Expression::*;
  let ex = build_expression;

  match &e.as_ref() {
    &Str(string)         => format!("\"{}\"", string),
    &Num(number)         => number.clone(),
    &Bool(true)          => String::from("true"),
    &Bool(false)         => String::from("false"),
    &Name(name)          => name.clone(),
    &Resource(name)      => build_resource_name(&name),
    &Parentheses(e)      => format!("({})", build_expression(&e)),
    &UnaryOp(op, e)      => format!("{}{}", op.as_str(), build_expression(&e)),
    &BinaryOp(Dot, a, b) => format!("{}.{}", ex(&a), ex(&b)),
    &BinaryOp(op, a, b)  => format!("{} {} {}", ex(&a), op.as_str(), ex(&b)),
    &Call(f, ref args)   => build_call(&f, args),
    &Indexing(v, a, k)   => build_indexing(&v, *a, &k),
    &TernaryOp(_,_,_)    => unreachable!()
  }
}

fn build_resource_name(name: &ResourceName) -> String {
  match &name {
    &ResourceName::Name(name) => name.clone(),
    &ResourceName::InModule(m, n) => {
      format!("{}__{}", m, build_resource_name(&*n))
    }
  }
}


fn join_arguments<T: AsRef<Expression>>(v: &Vec<T>) -> String {
  v.iter().map(|e| build_expression(e.as_ref())).collect::<Vec<_>>().join(", ")
}

fn build_call<T,U>(caller: &T, args: &Vec<U>) -> String
  where T: AsRef<Expression>,
        U: AsRef<Expression> {

  let mut result = build_expression(caller);

  result.push('(');
  result.push_str(&join_arguments(args));
  result.push(')');

  result
}

fn build_indexing<T,U>(v: &T, op: Accessor, keys: &Vec<U>) -> String
  where T: AsRef<Expression>,
        U: AsRef<Expression> {

  let mut result = build_expression(v);

  result.push('[');
  result.push_str(op.as_str());
  result.push_str(&join_arguments(keys));
  result.push(']');

  result
}


fn build_for_range(var: &str, f: &ForRange) -> String {
  match f {
    ForRange::Integer(from, to, Some(by)) => {
      let from = build_expression(from);
      let to   = build_expression(to);
      let by   = build_expression(by);
      format!("var {v} = {}; {v} != {}; {v} += {}", from, to, by, v=var)
    }
    _ => unreachable!()
  }
}


impl StatementBuilder {
  fn new(root: bool, indentation: usize) -> Self {
    StatementBuilder {
      root,
      result: String::new(),
      indentation
    }
  }

  fn add(&mut self, line: &str) {
    self.result.push_str(&format!("{}{}", " ".repeat(self.indentation), line));
  }

  fn argument_vars(&mut self, args: &Vec<String>) {
    let mut i = 0;
    for argument in args {
      self.add(&format!("var {} = argument{};\n", argument, i));
      i += 1;
    }
  }

  fn build_statement<T: AsRef<Statement>>(&mut self, statement: T) {
    match &statement.as_ref() {
      &Statement::Return(expr) => {
        self.add(&format!("return {};\n", build_expression(&expr)));
      }

      &Statement::Call(expr) => {
        self.add(&format!("{};\n", build_expression(&expr)));
      }

      &Statement::Body(statements) => {
        let indentation = if self.root {0} else {self.indentation + 4};
        let mut builder = StatementBuilder::new(false, indentation);
        for statement in statements {
          builder.build_statement(statement);
        }
        self.add(&builder.result);
      }

      &Statement::With(with, body) => {
        self.add(&format!("with {} {{\n", build_expression(&with)));
        self.build_statement(body);
        self.add("}\n");
      }

      &Statement::If(cond, then, or_else) => {
        self.add(&format!("if {} {{\n", build_expression(&cond)));
        self.build_statement(then);

        if let Some(or_else) = or_else {
          self.add("} else {\n");
          self.build_statement(or_else);
        }
        self.add("}\n");
      }

      &Statement::While(cond, body) => {
        self.add(&format!("while {} {{\n", build_expression(&cond)));
        self.build_statement(body);
        self.add("}\n");
      }

      &Statement::For(var, range, body) => {
        self.add(&format!("for ({}) {{\n", build_for_range(&var, &range)));
        self.build_statement(body);
        self.add("}\n");
      }

      Statement::Assignment(left, right) => {
        self.add(&format!("{} = {};\n", build_expression(left), build_expression(right)));
      }

      Statement::Var(vars) => {
        let mut parts = Vec::new();

        for var in vars {
          match var {
            VarDeclaration::Assignment(var, expr) => {
              parts.push(format!("{} = {}", var, build_expression(expr)));
            }
            VarDeclaration::Name(var) => {
              parts.push(var.clone())
            }
          }
        }
        self.add(&format!("var {};\n", parts.join(", ")));
      }
    }
  }
}

