use std::io::Write;
use std::iter;
use quick_xml::events::Event as XmlEvent;
use crate::error::*;
use crate::resources::sprite::*;
use crate::compiler::gmx::to_xml_trait::*;
use crate::compiler::gmx::field_value::*;

impl ResourceToXml for Sprite {
  fn write_xml<W: Write>(&self, w: W) -> Result<W> {
    write_events(w, vec![
      start!("sprite"),

      start!("xorig"),
      text(&self.origin.x.to_gmx_value()),
      end("xorig"),

      start!("yorigin"),
      text(&self.origin.y.to_gmx_value()),
      end("yorigin"),

      start!("colkind"),
      text(&(self.collision_kind as usize).to_gmx_value()),
      end("colkind"),

      start!("sepmasks"),
      text(&self.separate_masks.to_gmx_value()),
      end("sepmasks"),

      start!("bbox_left"),
      text(&self.bounding_box.left.to_gmx_value()),
      end("bbox_left"),

      start!("bbox_right"),
      text(&self.bounding_box.right.to_gmx_value()),
      end("bbox_right"),

      start!("bbox_top"),
      text(&self.bounding_box.top.to_gmx_value()),
      end("bbox_top"),

      start!("bbox_bottom"),
      text(&self.bounding_box.bottom.to_gmx_value()),
      end("bbox_bottom"),

      start!("bboxmode"),
      text(&(self.bounding_box.mode as usize).to_gmx_value()),
      end("bboxmode"),

      start!("coltolerance"),
      text(&self.collision_tolerance.to_gmx_value()),
      end("coltolerance"),

      start!("width"),
      text(&self.width.to_gmx_value()),
      end("width"),

      start!("height"),
      text(&self.height.to_gmx_value()),
      end("height"),

      start!("HTile"),
      text(&self.texture.horizontal.to_gmx_value()),
      end("HTile"),

      start!("VTile"),
      text(&self.texture.vertical.to_gmx_value()),
      end("VTile"),

      start!("For3D"),
      text(&self.texture.used_for_3d.to_gmx_value()),
      end("For3D"),

      //add frames
    ].into_iter().chain(frames_to_xml(&self.frames))

      //finish sprite
    .chain(iter::once(end("sprite"))))
  }
}

fn frames_to_xml(frames: &Vec<Frame>) -> Vec<XmlEvent<'static>> {
  let mut result = vec![start!("frames")];
  let mut index  = 0;

  for frame in frames {
    result.push(start!("frame", "index" => index));
    result.push(text(&frame.data.to_gmx_value()));
    result.push(end("frame"));
    index += 1;
  }
  result.push(end("frames"));
  result
}

