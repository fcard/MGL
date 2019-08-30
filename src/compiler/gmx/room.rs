use std::io::Write;
use std::iter;
use quick_xml::events::Event as XmlEvent;
use crate::error::*;
use crate::resources::room::*;
use crate::compiler::gmx::to_xml_trait::*;
use crate::compiler::gmx::field_value::*;

impl ResourceToXml for Room {
  fn write_xml<W: Write>(&self, w: W) -> Result<W> {
    write_events(w, vec![
      start!("room"),

      start!("width"),
      text(&self.width.to_gmx_value()),
      end("width"),

      start!("height"),
      text(&self.height.to_gmx_value()),
      end("height"),

      start!("speed"),
      text(&self.speed.to_gmx_value()),
      end("speed"),

      start!("isometric"),
      text(&self.isometric.to_gmx_value()),
      end("isometric"),

      start!("hsnap"),
      text(&self.horizontal_snap.to_gmx_value()),
      end("hsnap"),

      start!("vsnap"),
      text(&self.vertical_snap.to_gmx_value()),
      end("vsnap"),

      start!("persistent"),
      text(&self.persistent.to_gmx_value()),
      end("persistent"),

      start!("showcolor"),
      text(&self.show_color.to_gmx_value()),
      end("showcolor"),

      start!("enableViews"),
      text(&self.enable_views.to_gmx_value()),
      end("enableViews"),

      start!("clearViewBackground"),
      text(&self.clear_view_background.to_gmx_value()),
      end("clearViewBackground"),

      //add instances
    ].into_iter().chain(instances_to_xml(&self.instances))

      //finish room
     .chain(iter::once(end("room"))))
  }
}

fn instances_to_xml(instances: &Vec<InstanceItem>) -> Vec<XmlEvent<'static>> {
  let mut result = vec![start!("instances")];

  for instance_item in instances {
    if let InstanceItem::Resolved(name, instance) = instance_item {
      let code = match &instance.creation_code {
        Some(func) => format!("{}();", func.to_scoped_gmx_value("script")),
        None       => String::new()
      };

      result.push(empty!("instance",
        "name"     => name.to_scoped_gmx_value("instance"),
        "objName"  => instance.object.to_scoped_gmx_value("object"),
        "id"       => instance.id.to_gmx_value(),
        "x"        => instance.x.to_gmx_value(),
        "y"        => instance.y.to_gmx_value(),
        "scaleX"   => instance.scale_x.to_gmx_value(),
        "scaleY"   => instance.scale_y.to_gmx_value(),
        "rotation" => instance.rotation.to_gmx_value(),
        "alpha"    => instance.alpha.to_gmx_value(),
        "color"    => instance.color.to_gmx_value(),
        "code"     => code
      ));
    }
  }
  result.push(end("instances"));
  result
}

