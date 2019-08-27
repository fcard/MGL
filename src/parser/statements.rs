use crate::ast::*;
use crate::parser::tokens::*;
use crate::parser::grammar::*;
use crate::parser::expressions::*;

pub fn parse_statement(tk: Tokens) -> IStat {
  let tk_clone = tk.clone();

  IStat::new(
    match tk.as_rule() {
      Rule::return_statement => {
        Statement::return_op(parse_expression(tk))
      }

      Rule::statement_call => {
        Statement::call(parse_expression(tk))
      }

      Rule::body => {
        parse_body(tk)
      }

      Rule::with_statement => {
        parse_with(tk)
      }

      Rule::if_statement => {
        parse_if(tk)
      }

      Rule::while_statement => {
        parse_while(tk)
      }

      Rule::for_statement => {
        parse_for(tk)
      }

      Rule::assignment => {
        parse_assignment(tk)
      }

      Rule::var => {
        parse_var(tk)
      }

      _ => unreachable!()
    }
  ).with_info(tk_clone)
}


pub fn parse_with(tk: Tokens) -> Statement {
  let mut parts = tk.into_inner();
  let expr = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  Statement::with(expr, body)
}


pub fn parse_if(tk: Tokens) -> Statement {
  let mut parts = tk.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  let or_else = parts.next().map(parse_statement);
  Statement::if_op(cond, body, or_else)
}


pub fn parse_while(tk: Tokens) -> Statement {
  let mut parts = tk.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  Statement::while_op(cond, body)
}


pub fn parse_for(tk: Tokens) -> Statement {
  let mut parts = tk.into_inner();
  let name  = parts.next().unwrap().as_str();
  let range = parse_for_range(parts.next().unwrap());
  let body  = parse_statement(parts.next().unwrap());
  Statement::for_op(name, range, body)
}


pub fn parse_for_range(tk: Tokens) -> ForRange {
  let mut parts = tk.into_inner();
  let start = parse_expression(parts.next().unwrap());

  if let Some(end) = parts.next().map(parse_expression) {
    let by = parts.next().map(parse_expression);
    ForRange::integer(start, end, by)

  } else {
    ForRange::array(start)
  }
}


pub fn parse_assignment(tk: Tokens) -> Statement {
  let mut parts = tk.into_inner();
  let left  = parse_expression(parts.next().unwrap());
  let right = parse_expression(parts.next().unwrap());
  Statement::assignment(left, right)
}


pub fn parse_var_assignment(tk: Tokens) -> VarDeclaration {
  let mut parts = tk.into_inner();
  let left  = parts.next().unwrap().as_str();
  let right = parse_expression(parts.next().unwrap());
  VarDeclaration::assignment(left, right)
}


pub fn parse_var(tk: Tokens) -> Statement {
  let mut vars = Vec::new();

  for var in tk.into_inner() {
    match var.as_rule() {
      Rule::name => vars.push(VarDeclaration::name(var.as_str())),
      Rule::var_assignment => vars.push(parse_var_assignment(var)),
      _ => unreachable!()
    }
  }

  Statement::var(&vars)
}


pub fn parse_body(tk: Tokens) -> Statement {
  let mut body_statements = Vec::new();

  for statement_tk in tk.into_inner() {
    body_statements.push(parse_statement(statement_tk));
  }

  Statement::body(&body_statements)
}


