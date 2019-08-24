use crate::ast::*;
use crate::ast::precedence::*;
use crate::parser::grammar::*;

pub fn parse_expression(top_pair: Pair) -> IExpr {
  let mut expression = None;

  for pair in top_pair.clone().into_inner() {
    let pair_clone = pair.clone();

    expression = Some(IExpr::new(
      match pair.as_rule() {
        Rule::name    => Expression::name(pair.as_str()),
        Rule::string  => Expression::string(pair.as_str()),
        Rule::number  => Expression::num(pair.as_str()),
        Rule::boolean => Expression::boolean(pair.as_str().parse().unwrap()),

        Rule::variable_name => parse_expression(pair).content(),
        Rule::parentheses   => Expression::parentheses(parse_expression(pair)),

        Rule::resource => {
          let names: Vec<_> = pair.into_inner().map(|p| p.as_str()).collect();
          Expression::resource(&names)
        }

        Rule::unary_op => {
          let mut parts = pair.into_inner();
          let operator = parts.next().unwrap().as_str();
          let operand  = parse_expression(parts.next().unwrap());
          Expression::unary_op(operator, operand)
        }

        Rule::binary_op => {
          let mut parts = pair.into_inner();
          let op = parts.next().unwrap().as_str();
          let right = parse_expression(parts.next().unwrap());
          Expression::binary_op(op, expression.unwrap(), right)
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
    ).with_position(pair_clone));
  }
  expression.unwrap().fix_precedence()
}

