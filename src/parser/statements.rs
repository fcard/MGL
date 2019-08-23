use crate::ast::*;
use crate::parser::grammar::*;
use crate::parser::expressions::*;

pub fn parse_statement(pair: Pair) -> Ast<Statement> {
  let pair_clone = pair.clone();

  Ast::new(
    match pair.as_rule() {
      Rule::return_statement => {
        Statement::return_op(parse_expression(pair))
      }

      Rule::statement_call => {
        Statement::call(parse_expression(pair))
      }

      Rule::body => {
        parse_body(pair)
      }

      Rule::with_statement => {
        parse_with(pair)
      }

      Rule::if_statement => {
        parse_if(pair)
      }

      Rule::while_statement => {
        parse_while(pair)
      }

      Rule::for_statement => {
        parse_for(pair)
      }

      Rule::assignment => {
        parse_assignment(pair)
      }

      Rule::var => {
        parse_var(pair)
      }

      _ => unreachable!()
    }
  ).pos(pair_clone)
}


pub fn parse_with(with_pair: Pair) -> Statement {
  let mut parts = with_pair.into_inner();
  let expr = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  Statement::with(expr, body)
}


pub fn parse_if(if_pair: Pair) -> Statement {
  let mut parts = if_pair.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  let or_else = parts.next().map(parse_statement);
  Statement::if_op(cond, body, or_else)
}


pub fn parse_while(while_pair: Pair) -> Statement {
  let mut parts = while_pair.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_statement(parts.next().unwrap());
  Statement::while_op(cond, body)
}


pub fn parse_for(for_pair: Pair) -> Statement {
  let mut parts = for_pair.into_inner();
  let name  = parts.next().unwrap().as_str();
  let range = parse_for_range(parts.next().unwrap());
  let body  = parse_statement(parts.next().unwrap());
  Statement::for_op(name, range, body)
}


pub fn parse_for_range(pair: Pair) -> ForRange {
  let mut parts = pair.into_inner();
  let start = parse_expression(parts.next().unwrap());

  if let Some(end) = parts.next().map(parse_expression) {
    let by = parts.next().map(parse_expression);
    ForRange::integer(start, end, by)

  } else {
    ForRange::array(start)
  }
}


pub fn parse_assignment(pair: Pair) -> Statement {
  let mut parts = pair.into_inner();
  let left  = parse_expression(parts.next().unwrap());
  let right = parse_expression(parts.next().unwrap());
  Statement::assignment(left, right)
}


pub fn parse_var_assignment(pair: Pair) -> VarDeclaration {
  let mut parts = pair.into_inner();
  let left  = parts.next().unwrap().as_str();
  let right = parse_expression(parts.next().unwrap());
  VarDeclaration::assignment(left, right)
}


pub fn parse_var(pair: Pair) -> Statement {
  let mut vars = Vec::new();

  for var in pair.into_inner() {
    match var.as_rule() {
      Rule::name => vars.push(VarDeclaration::name(var.as_str())),
      Rule::var_assignment => vars.push(parse_var_assignment(var)),
      _ => unreachable!()
    }
  }

  Statement::var(&vars)
}


pub fn parse_body(body_pair: Pair) -> Statement {
  let mut body_statements = Vec::new();

  for statement_pair in body_pair.into_inner() {
    body_statements.push(parse_statement(statement_pair));
  }

  Statement::body(&body_statements)
}


