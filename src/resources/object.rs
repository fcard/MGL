use crate::ast::*;
use crate::error::*;
use crate::resources::resource_trait::*;
use crate::event::Event;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Object {
  pub sprite: Option<ResourceName>,
  pub depth: i64,
  pub solid: bool,
  pub visible: bool,
  pub persistent: bool,
  pub parent: Option<ResourceName>,
  pub mask: Option<ResourceName>,

  #[ignore_field]
  pub events: Vec<(Event, ResourceName)>,
}

impl Object {
  pub fn new(declaration: ResourceDeclaration) -> Result<Object> {
    let mut object = Object {
      sprite: None,
      depth: 0,
      solid: false,
      visible: true,
      persistent: false,
      parent: None,
      mask: None,
      events: Vec::new()
    };

    for KeyValue { key, value } in declaration.key_values() {
      match Event::try_from(key.clone()) {
        Ok(event) => {
          object.events.push((event, parse_event_value(value)?));
        }

        Err(EventErrorKind::InvalidName) => {
          object.parse_key_value(&declaration, key, value)?;
        }

        Err(e) => {
          return Err(MglError::Event { kind: e });
        }
      }
    }

    Ok(object)
  }
}

fn parse_event_value(expr: &IExpr) -> Result<ResourceName> {
  match expr.as_ref() {
    &Expression::Name(ref name) => {
      Ok(ResourceName::new(&["script", &*name]))
    }

    &Expression::Resource(ref resource_name) => {
      if resource_name.top_module_is("script") {
        Ok(resource_name.clone())
      } else {
        Ok(ResourceName::InModule(String::from("script"), box resource_name.clone()))
      }
    }
    _ => {
      MglError::convert_expression(expr.clone(), "ResourceName")
    }
  }
}

