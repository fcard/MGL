use pest::Parser;
use pest::iterators;

type Pair<'a>  = iterators::Pair<'a, Rule>;
type Pairs<'a> = iterators::Pairs<'a, Rule>;

use crate::parser::ast::*;
use crate::parser::precedence::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct MglParser;

pub fn parse_code(code: &str) -> Top  {
  let top_expressions = top_to_pairs(code);
  let mut declarations = Vec::new();

  for top_expression in top_expressions {
    if let Some(declaration) = parse_declaration(top_expression) {
      declarations.push(declaration);
    }
  }

  Top::new(&declarations)
}

pub fn parse_declaration(pair: Pair) -> Option<Declaration> {
  match pair.as_rule() {
    Rule::function_declaration => {
      Some(parse_function(pair))
    }

    Rule::object_declaration => {
      unimplemented!()
    }

    Rule::wrapper_declaration => {
      unimplemented!()
    }

    _ => return None
  }
}

pub fn parse_function(pair: Pair) -> Declaration {
  let mut parts = pair.into_inner();
  let name = parts.next().unwrap().as_str();
  let args = parts.next().unwrap().into_inner().map(|p| p.as_str()).collect::<Vec<_>>();
  let body = parts.next().unwrap();

  Declaration::function(name, &args, parse_body(body))
}

pub fn parse_statement(pair: Pair) -> Statement {
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
}

pub fn parse_with(if_pair: Pair) -> Statement {
  let mut parts = if_pair.into_inner();
  let expr = parse_expression(parts.next().unwrap());
  let body = parse_body(parts.next().unwrap());
  Statement::with(expr, body)
}

pub fn parse_if(if_pair: Pair) -> Statement {
  let mut parts = if_pair.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_body(parts.next().unwrap());
  let or_else = parts.next().map(parse_body);
  Statement::if_op(cond, body, or_else)
}


pub fn parse_while(while_pair: Pair) -> Statement {
  let mut parts = while_pair.into_inner();
  let cond = parse_expression(parts.next().unwrap());
  let body = parse_body(parts.next().unwrap());
  Statement::while_op(cond, body)
}


pub fn parse_for(for_pair: Pair) -> Statement {
  let mut parts = for_pair.into_inner();
  let name  = parts.next().unwrap().as_str();
  let range = parse_for_range(parts.next().unwrap());
  let body  = parse_body(parts.next().unwrap());
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


pub fn parse_expression(top_pair: Pair) -> Expression {
  let mut expression = None;

  for pair in top_pair.into_inner() {
    expression = Some(
      match pair.as_rule() {
        Rule::name    => Expression::name(pair.as_str()),
        Rule::string  => Expression::string(pair.as_str()),
        Rule::number  => Expression::num(pair.as_str()),
        Rule::boolean => Expression::boolean(pair.as_str().parse().unwrap()),

        Rule::variable_name => parse_expression(pair),
        Rule::parentheses   => Expression::parentheses(parse_expression(pair)),

        Rule::resource => {
          let names: Vec<_> = pair.into_inner().map(|p| p.as_str()).collect();
          Expression::resource(&names)
        }

        Rule::unary_op => {
          let mut parts = pair.into_inner();
          let operator = parts.next().unwrap().as_str();
          let operand  = parse_expression(parts.next().unwrap());
          Expression::unary_op(operator, operand).fix_precedence()
        }

        Rule::binary_op => {
          let mut parts = pair.into_inner();
          let op = parts.next().unwrap().as_str();
          let right = parse_expression(parts.next().unwrap());
          Expression::binary_op(op, expression.unwrap(), right).fix_precedence()
        }

        Rule::ternary_op => {
          let mut parts = pair.into_inner();
          let left  = parse_expression(parts.next().unwrap());
          let right = parse_expression(parts.next().unwrap());
          Expression::ternary_op(expression.unwrap(), left, right)
        }

        Rule::call => {
          let parts = pair.into_inner();
          let args = parts.map(parse_expression).collect::<Vec<_>>();
          Expression::call(expression.unwrap(), &args)
        }

        Rule::indexing => {
          let mut parts = pair.into_inner();
          let acc  = parts.next().unwrap().as_str();
          let args = parts.map(parse_expression).collect::<Vec<_>>();
          Expression::indexing(expression.unwrap(), acc, &args)
        }

        _ => unreachable!()
      }
    );
  }
  expression.unwrap()
}

fn top_to_pairs<'a>(code: &'a str) -> Pairs<'a> {
  match MglParser::parse(Rule::top, code) {
    Ok(top_pairs) => {
      for top_pair in top_pairs {
        match top_pair.as_rule() {
          Rule::top => {
            return top_pair.into_inner()
          }
          _ => ()
        }
      }
      panic!("top rule not found");
    }
    Err(err) => panic!("{}", err)
  }
}
