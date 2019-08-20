use crate::resources::resource_trait::*;
use crate::ast::*;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Sound {
  pub kind: SoundKind,
  pub data: PathBuf,
  pub volume: f64,
  pub pan: f64,
  pub bit_rate: u64,
  pub sample_rate: u64,
  pub bit_depth: u64,
  pub preload: bool,
  pub compress: bool,
  pub uncompress_on_load: bool,
  pub audio_group: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SoundKind {
  Normal = 0,
  Background,
  ThreeDimensional,
  ExternalPlayer
}

impl ResourceDefault<ResourceDeclaration> for Sound {
  fn default(_resource: &ResourceDeclaration) -> Result<Self> {
    Ok(
      Sound {
        kind: SoundKind::Normal,
        data: PathBuf::from(""),
        volume: 1.0,
        pan: 0.0,
        bit_rate: 192,
        sample_rate: 44100,
        bit_depth: 16,
        preload: false,
        compress: false,
        uncompress_on_load: false,
        audio_group: 0,
      },
    )
  }
}

