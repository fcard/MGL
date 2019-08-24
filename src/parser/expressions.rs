use crate::ast::*;
use crate::ast::precedence::*;
use crate::parser::grammar::*;
use crate::parser::tokens::*;

pub fn parse_expression(top_tokens: Tokens) -> IExpr {
  let mut expression = None;

  for tk in top_tokens.clone().into_inner() {
    let tk_clone = tk.clone();

    expression = Some(IExpr::new(
      match tk.as_rule() {
        Rule::name    => Expression::name(tk.as_str()),
        Rule::string  => Expression::string(tk.as_str()),
        Rule::number  => Expression::num(tk.as_str()),
        Rule::boolean => Expression::boolean(tk.as_str().parse().unwrap()),

        Rule::variable_name => parse_expression(tk).content(),
        Rule::parentheses   => Expression::parentheses(parse_expression(tk)),

        Rule::resource => {
          let names: Vec<_> = tk.into_inner().map(|p| p.as_str()).collect();
          Expression::resource(&names)
        }

        Rule::unary_op => {
          let mut parts = tk.into_inner();
          let operator = parts.next().unwrap().as_str();
          let operand  = parse_expression(parts.next().unwrap());
          Expression::unary_op(operator, operand)
        }

        Rule::binary_op => {
          let mut parts = tk.into_inner();
          let op = parts.next().unwrap().as_str();
          let right = parse_expression(parts.next().unwrap());
          Expression::binary_op(op, expression.unwrap(), right)
        }

        Rule::ternary_op => {
          let mut parts = tk.into_inner();
          let left  = parse_expression(parts.next().unwrap());
          let right = parse_expression(parts.next().unwrap());
          Expression::ternary_op(expression.unwrap(), left, right)
        }

        Rule::call => {
          let parts = tk.into_inner();
          let args = parts.map(parse_expression).collect::<Vec<_>>();
          Expression::call(expression.unwrap(), &args)
        }

        Rule::indexing => {
          let mut parts = tk.into_inner();
          let acc  = parts.next().unwrap().as_str();
          let args = parts.map(parse_expression).collect::<Vec<_>>();
          Expression::indexing(expression.unwrap(), acc, &args)
        }

        _ => unreachable!()
      }
    ).with_info(tk_clone));
  }
  expression.unwrap().fix_precedence()
}

