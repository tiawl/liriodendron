use image;

/// Module to set and check the number of generated textures
pub mod generationnumber;

use crate::task::generation::palette::PaletteGeneration;

const DEFAULT_PIXEL_RATIO: usize = 3;
const DEFAULT_BORDER_RATIO: u8 = 3;
const DEFAULT_PALETTE: PaletteGeneration =
  PaletteGeneration::RandomPredefinedColors;
const DEFAULT_NUMBER_GENERATION: u16 = 1;

const NB_PIXEL_RATIO_SETTINGS: usize = 15;
const NB_FORMAT: usize = 2;

/// Represents textures' parameters
pub struct TexturesSettings {

  /// Ratio between grid's cell and texture's pixels
  pixel_ratio: [u16; NB_PIXEL_RATIO_SETTINGS],

  current_pixel_ratio: usize,

  /// Ratio between <i>FullPixel::BorderPixel</i> color and
  /// <i>FullPixel::BodyPixel</i> color
  border_ratio: u8,

  palette: PaletteGeneration,
  number_generation: u16,

  format: [image::ImageFormat; NB_FORMAT],
  current_format: usize,
}

impl TexturesSettings {

  pub fn new() -> TexturesSettings {
    TexturesSettings {
      pixel_ratio: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 20, 25, 30],
      current_pixel_ratio: DEFAULT_PIXEL_RATIO,
      border_ratio: DEFAULT_BORDER_RATIO,
      palette: DEFAULT_PALETTE,
      number_generation: DEFAULT_NUMBER_GENERATION,
      format: [image::ImageFormat::Png, image::ImageFormat::Jpeg],
      current_format: 0,
    }
  }

  pub fn get_pixel_ratio(&self) -> u16 {
    self.pixel_ratio[self.current_pixel_ratio]
  }

  pub fn incr_pixel_ratio(&mut self) {
    if self.current_pixel_ratio < NB_PIXEL_RATIO_SETTINGS - 1 {
      self.current_pixel_ratio += 1;
    }
  }

  pub fn decr_pixel_ratio(&mut self) {
    if self.current_pixel_ratio > 0 {
      self.current_pixel_ratio -= 1;
    }
  }

  pub fn get_border_ratio(&self) -> u8 {
    self.border_ratio
  }

  pub fn get_palette(&self) -> PaletteGeneration {
    self.palette
  }

  pub fn get_number(&self) -> u16 {
    self.number_generation
  }

  pub fn set_number(&mut self, number: u16) {
    self.number_generation = number;
  }

  pub fn get_format(&self) -> image::ImageFormat {
    self.format[self.current_format]
  }

  pub fn next_format(&mut self) {
    if self.current_format > 0 {
      self.current_format -= 1;
    }
  }

  pub fn previous_format(&mut self) {
    if self.current_format < self.format.len() - 1 {
      self.current_format += 1;
    }
  }

  pub fn get_format_info(&self) -> ([image::ImageFormat; NB_FORMAT], usize) {
    (self.format, self.current_format)
  }
}
