use std::io::Write;
use crate::error::*;
use crate::resources::sound::*;
use crate::compiler::gmx::to_xml_trait::*;
use crate::compiler::gmx::field_value::*;

impl ResourceToXml for Sound {
  fn write_xml<W: Write>(&self, w: W) -> Result<W> {
    write_events(w, vec![
      start!("sound"),

      start!("kind"),
      text(&(self.kind as usize).to_gmx_value()),
      end("kind"),

      start!("data"),
      text(&self.data.file_name().to_gmx_value()),
      end("data"),

      start!("volume"),
      start!("volume"),
      text(&self.volume.to_gmx_value()),
      end("volume"),
      end("volume"),

      start!("pan"),
      text(&self.pan.to_gmx_value()),
      end("pan"),

      start!("bitRates"),
      start!("bitRate"),
      text(&self.bit_rate.to_gmx_value()),
      end("bitRate"),
      end("bitRates"),

      start!("sampleRates"),
      start!("sampleRate"),
      text(&self.sample_rate.to_gmx_value()),
      end("sampleRate"),
      end("sampleRates"),

      start!("types"),
      start!("type"),
      text("0"),
      end("type"),
      end("types"),

      start!("bitDepths"),
      start!("bitDepth"),
      text(&self.bit_depth.to_gmx_value()),
      end("bitDepth"),
      end("bitDepths"),

      start!("preload"),
      text(&self.preload.to_gmx_value()),
      end("preload"),

      start!("compress"),
      text(&self.compress.to_gmx_value()),
      end("compress"),

      start!("uncompressOnLoad"),
      text(&self.uncompress_on_load.to_gmx_value()),
      end("uncompressOnLoad"),

      start!("audioGroup"),
      text(&self.audio_group.to_gmx_value()),
      end("audioGroup"),

      end("sound"),

    ].into_iter())
  }
}

