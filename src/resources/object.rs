use crate::ast::*;
use crate::error::*;
use crate::resources::resource_trait::*;
use crate::event::{Event, EventParserError};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Object {
  sprite: Option<ResourceName>,

  #[ignore_field]
  events: Vec<(Event, ResourceName)>,
}

impl Object {
  pub fn new(declaration: ResourceDeclaration) -> Result<Object> {
    let mut object = Object {
      sprite: None,
      events: Vec::new()
    };

    for KeyValue { key, value } in declaration.key_values() {
      match Event::try_from(key.clone()) {
        Ok(event) => {
          object.events.push((event, parse_event_value(key, value)?));
        }

        Err(EventParserError::InvalidIndex(message)) => {
          return Err(MglError::EventInvalidIndex { message });
        }

        Err(EventParserError::InvalidName) => {
          object.parse_key_value(&declaration, key.clone(), value.clone())?;
        }
      }
    }

    Ok(object)
  }
}

fn parse_event_value(key: &Key, expr: &Expression) -> Result<ResourceName> {
  match expr {
    Expression::Resource(resource_name) => {
      if resource_name.top_module_is("script") {
        Ok(resource_name.clone())
      } else {
        Ok(ResourceName::InModule(String::from("script"), box resource_name.clone()))
      }
    }
    _ => {
      MglError::wrong_field_type(expr.clone(), &key.name_of(), "script name")
    }
  }
}

