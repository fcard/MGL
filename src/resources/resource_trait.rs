use std::convert::TryFrom;
use crate::ast::*;
pub use crate::resources::room::InstanceItems;
pub use mgl_resource_derive::*;
pub use crate::error::{Result, MglError, InvalidFieldKind};

// Traits to implement

pub trait Resource<T: ResourceAst>: Sized {
  fn parse_key_value(&mut self, source: &T, key: &Key, value: &Expression) -> Result<()>;
}

pub trait ResourceDefault<T: ResourceAst>: Sized {
  fn default(source: &T) -> Result<Self>;
}

// Helper functions

pub trait FromExpression = TryFrom<Expression, Error=MglError>;

pub fn parse_field_default<T: FromExpression>(key: &Key, expr: &Expression) -> Result<T> {
  let field = &key.name_of();
  match T::try_from(expr.clone()) {
    Ok(value) => Ok(value),

    Err(MglError::ConvertExpression { value, into_type }) => {
      MglError::wrong_field_type(value, &into_type, field)
    }

    error => error
  }
}


// Helper traits and types

pub trait ResourceAst {
  fn key_values(&self) -> &[KeyValue];
}

impl ResourceAst for InstanceDeclaration {
  fn key_values(&self) -> &[KeyValue] {
    &self.key_value_pairs
  }
}

impl ResourceAst for ResourceDeclaration {
  fn key_values(&self) -> &[KeyValue] {
    &self.key_value_pairs
  }
}

impl ResourceAst for (ResourceDeclaration, InstanceItems) {
  fn key_values(&self) -> &[KeyValue] {
    &self.0.key_value_pairs
  }
}


pub trait ResourceCreate<T: ResourceAst>: Sized + ResourceDefault<T> {
  fn new(source: T) -> Result<Self>;
}

impl<S: ResourceDefault<T> + Resource<T>, T: ResourceAst> ResourceCreate<T> for S {
  fn new(source: T) -> Result<Self> {
    let mut resource = Self::default(&source)?;
    for KeyValue { key, value } in source.key_values() {
      resource.parse_key_value(&source, key, value)?;
    }
    Ok(resource)
  }
}



pub struct KeyInspector;

impl KeyInspector {
  pub fn assert_field_has_no_index(field: &str, key: &Key) -> Result<()> {
    match key {
      &Key::Name(_) => Ok(()),
      _ => MglError::invalid_field(field, InvalidFieldKind::NotSimple(key.clone()))
    }
  }

  pub fn get_array_index(field: &str, key: &Key) -> Result<usize> {
    match key.leftmost_index_of() {
      Some(a) => Ok(parse_field_default(key, &a)?),
      _ => MglError::invalid_field(field, InvalidFieldKind::NotArray(key.clone()))
    }
  }

  pub fn get_sub_field_key(field: &str, key: &Key) -> Result<Key> {
    match key {
      Key::Dot(_, box sf) => Ok(sf.clone()),
      _ => MglError::invalid_field(field, InvalidFieldKind::NotSubResource(key.clone()))
    }
  }
}

