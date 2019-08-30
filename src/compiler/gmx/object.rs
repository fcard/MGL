use std::io::Write;
use std::iter;
use quick_xml::events::Event as XmlEvent;
use crate::ast::*;
use crate::event::*;
use crate::error::*;
use crate::resources::object::*;
use crate::compiler::gmx::event::*;
use crate::compiler::gmx::to_xml_trait::*;
use crate::compiler::gmx::field_value::*;

impl ResourceToXml for Object {
  fn write_xml<W: Write>(&self, w: W) -> Result<W> {
    write_events(w, vec![
      start!("object"),

      start!("spriteName"),
      text(&self.sprite.to_scoped_gmx_value("sprite")),
      end("spriteName"),

      start!("depth"),
      text(&self.depth.to_gmx_value()),
      end("depth"),

      start!("solid"),
      text(&self.solid.to_gmx_value()),
      end("solid"),

      start!("visible"),
      text(&self.visible.to_gmx_value()),
      end("visible"),

      start!("persistent"),
      text(&self.persistent.to_gmx_value()),
      end("persistent"),

      start!("parentName"),
      text(&self.parent.to_scoped_gmx_value("object")),
      end("parentName"),

      start!("maskName"),
      text(&self.mask.to_scoped_gmx_value("object")),
      end("maskName"),

      //add events
    ].into_iter().chain(events_to_xml(&self.events))

      //finish object
    .chain(iter::once(end("object"))))
  }
}

fn events_to_xml(events: &Vec<(Event, ResourceName)>) -> Vec<XmlEvent<'static>> {
  let mut result = vec![start!("events")];

  for (event, func) in events {
    let (ty, key, value) = event_info(&event);
    let funcall = format!("{}()", func.to_scoped_gmx_value("script"));

    result.append(&mut vec![
      start!("event",
        "eventtype" => ty.to_gmx_value(),
        key         => value
      ),
      start!("action"),

      start!("libid"),
      text("1"),
      end("libid"),

      start!("id"),
      text("603"),
      end("id"),

      start!("kind"),
      text("7"),
      end("kind"),

      start!("userelative"),
      text("0"),
      end("userelative"),

      start!("useapplyto"),
      text("-1"),
      end("useapplyto"),

      start!("isquestion"),
      text("0"),
      end("isquestion"),

      start!("exetype"),
      text("2"),
      end("exetype"),

      empty!("functionname"),
      empty!("codestring"),

      start!("whoName"),
      text("self"),
      end("whoName"),

      start!("relative"),
      text("0"),
      end("relative"),

      start!("isnot"),
      text("0"),
      end("isnot"),

      start!("arguments"),
      start!("argument"),
      start!("kind"),
      text("1"),
      end("kind"),
      start!("string"),
      text(&funcall),
      end("string"),
      end("argument"),
      end("arguments"),

      end("action"),
      end("event"),
    ]);
  }

  result.push(end("events"));
  result
}


