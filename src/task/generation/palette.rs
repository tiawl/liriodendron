extern crate image;
use image::Rgba;

extern crate rand;
use rand::{thread_rng, Rng};

/// An RGBA array
const PREDEFINED_COLORS: [Rgba<u8>; 14] = [
  Rgba([252, 41, 34, 255]),           // RED
  Rgba([6, 124, 77, 255]),            // DARK GREEN
  Rgba([83, 101, 255, 255]),          // BLUE
  Rgba([247, 244, 139, 255]),         // YELLOW
  Rgba([147, 112, 246, 255]),         // PURPLE
  Rgba([158, 194, 255, 255]),         // LIGHT BLUE
  Rgba([255, 255, 255, 255]),         // WHITE
  Rgba([125, 125, 125, 255]),         // GREY
  Rgba([244, 164, 96, 255]),          // ORANGE
  Rgba([252, 145, 212, 255]),         // PINK
  Rgba([181, 101, 29, 255]),          // BROWN
  Rgba([180, 215, 162, 255]),         // LIGHT GREEN
  Rgba([142, 32, 21, 255]),           // BORDEAUX
  Rgba([33, 42, 165, 255]),           // NAVY
];

/// A set of colors
#[derive(Clone, Copy)]
pub struct Palette {
  body_color: Rgba<u8>,
  border_color: Rgba<u8>,
}

impl Palette {

  pub fn new(color: Rgba<u8>, border_ratio: u8) -> Palette {
    let Rgba{ 0: rgb } = color;
    Palette {
      body_color: color,
      border_color: Rgba([rgb[0] / border_ratio, rgb[1] / border_ratio,
        rgb[2] / border_ratio, 255]),
    }
  }

  pub fn get_body_color(&self) -> Rgba<u8> {
    self.body_color
  }

  pub fn get_border_color(&self) -> Rgba<u8> {
    self.border_color
  }
}

/// Generate a set of colors
#[derive(Clone, Copy)]
pub enum PaletteGeneration {
  RandomPredefinedColors,
}

impl PaletteGeneration {

  pub fn run(&self, border_ratio: u8) -> Palette {
    match self {
      PaletteGeneration::RandomPredefinedColors => {
        let mut rng = thread_rng();
        Palette::new(PREDEFINED_COLORS[rng.gen_range(0, 13)], border_ratio)
      },
    }
  }
}
