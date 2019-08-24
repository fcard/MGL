use std::str::FromStr;

use crate::ast::*;
use crate::error::*;
use crate::parser::*;
use crate::parser::grammar::*;
use crate::parser::expressions::*;
use crate::parser::statements::*;
use crate::parser::declarations::*;
use crate::ast::std_traits::from_str_macros::*;

implement_from_str! {
  argument = s;

  Expression => {
    Ok(*parse_expression(parse_mgl(Rule::expression, s)?).content)
  },

  Statement => {
    let pair = parse_mgl(Rule::statement_non_silent, s)?;
    let stat = pair.into_inner().next().unwrap();
    Ok(*parse_statement(stat).content)
  },

  KeyValue => {
    Ok(parse_key_value(parse_mgl(Rule::key_value, s)?))
  },

  Key => {
    Ok(parse_key(parse_mgl(Rule::key, s)?.into_inner()))
  },

  FunctionDeclaration => {
    Ok(parse_function(parse_mgl(Rule::function_declaration, s)?))
  },

  InstanceDeclaration => {
    Ok(parse_instance(parse_mgl(Rule::instance_declaration, s)?))
  },

  ResourceDeclaration => {
    if let Declaration::Resource(res) = s.parse()? {
      Ok(res)

    } else {
      Err(MglError::NotResource)
    }
  },

  Declaration => {
    let Top { declarations } = s.parse()?;
    if let [ declaration ] = &*declarations {
      Ok(declaration.clone())

    } else {
      Err(MglError::NotResource)
    }
  },

  Top => {
    parse_code(s)
  },

  {T: FromStr<Err=MglError> + Clone} AstDebugInfo<T> => {
    Ok(AstDebugInfo::new(T::from_str(s)?))
  }
}

