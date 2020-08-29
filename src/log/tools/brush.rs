use crate::utils::FullPixel;

/// Module to set and check <i>color</i> member of the brush
pub mod brushcolor;

/// Represent brush tool
pub struct Brush {

  /// Possible Brush actions
  pixels: [FullPixel; 4],

  /// Specific pixel color
  color: (u8, u8, u8),

  /// Cycled counter between 0 and 4
  current_action: usize,

  /// Body percent chance of apparition when a pixel is BodyEmpty
  body_de_ratio: u8,

  /// Body percent chance of apparition when a pixel is BodyBorder
  body_dr_ratio: u8,
}

impl Brush {
  pub fn new() -> Brush {
    Brush {
      pixels:
      [
        FullPixel::BodyEmpty,
        FullPixel::BodyBorder,
        FullPixel::Border,
        FullPixel::Body,
      ],
      color: (255, 255, 255),
      current_action: 0,
      body_de_ratio: 50,
      body_dr_ratio: 50,
    }
  }

  /// If <i>current_action</i> is 4, the current action is
  /// <i>FullPixel::SpecificColor</i>
  pub fn get_current_action(&self) -> FullPixel {
    if self.current_action < 4 {
      self.pixels[self.current_action]
    } else {
      let (red, green, blue) = self.color;
      FullPixel::SpecificColor(red, green, blue)
    }
  }

  pub fn next(&mut self) {
    if self.current_action == 4 {
      self.current_action = 0;
    } else {
      self.current_action += 1;
    }
  }

  pub fn previous(&mut self) {
    if self.current_action == 0 {
      self.current_action = 4;
    } else {
      self.current_action -= 1;
    }
  }

  pub fn get_color(&self) -> (u8, u8, u8) {
    self.color
  }

  pub fn set_color(&mut self, color: (u8, u8, u8)) {
    self.color = color;
  }

  pub fn get_body_de_ratio(&self) -> u8 {
    self.body_de_ratio
  }

  pub fn incr_body_de_ratio(&mut self) {
    if self.body_de_ratio > 1 {
      self.body_de_ratio -= 1;
    }
  }

  pub fn decr_body_de_ratio(&mut self) {
    if self.body_de_ratio < 99 {
      self.body_de_ratio += 1;
    }
  }

  pub fn get_body_dr_ratio(&self) -> u8 {
    self.body_dr_ratio
  }

  pub fn incr_body_dr_ratio(&mut self) {
    if self.body_dr_ratio > 1 {
      self.body_dr_ratio -= 1;
    }
  }

  pub fn drcr_body_dr_ratio(&mut self) {
    if self.body_dr_ratio < 99 {
      self.body_dr_ratio += 1;
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_passes_to_previous_brush() {
    let mut brush = Brush::new();
    let init_brush = brush.current_action;
    brush.previous();
    assert!((init_brush == 0) && (brush.current_action == 4));
  }

  #[test]
  fn it_passes_to_next_brush() {
    let mut brush = Brush::new();
    brush.previous();
    let init_brush = brush.current_action;
    brush.next();
    assert!((init_brush == 4) && (brush.current_action == 0));
  }

  #[test]
  fn it_sets_brush_color() {
    let mut brush = Brush::new();
    let init_color = brush.get_color();
    let new_color = (200, 150, 8);
    brush.set_color(new_color);
    assert!((init_color == (255, 255, 255)) &&
      (brush.get_color() == new_color));
  }
}
