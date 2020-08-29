extern crate std;
use std::ops::{Deref, DerefMut};

use crate::log::tools::{MAX_SIZE, setter};

/// Represents a customizable name for a grid
pub struct GridName {
  cursor: setter::StringCursor,
}

/// Tips to use shared functions between Setter sub-structs
impl Deref for GridName {
  type Target = setter::StringCursor;

  fn deref(&self) -> &Self::Target {
    &self.cursor
  }
}

/// Tips to use shared functions between Setter sub-structs
impl DerefMut for GridName {

  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.cursor
  }
}

impl GridName {
  pub fn new() -> GridName {
    GridName {
      cursor: setter::StringCursor::new(MAX_SIZE),
    }
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }

  pub fn backspace(&mut self) {
    if self.pos > 0 {
      self.cursor_left();
      let index = self.pos;
      self.value.remove(index);
    }
  }

  pub fn reset(&mut self) {
    self.value.clear();
    self.pos = 0;
  }
}

impl setter::Setter for GridName {

  /// Checks value and set it
  fn set_value(&mut self, letter: char) {
    let index = self.pos;
    if index < self.max_length {
      self.value.insert(index, letter);
      self.cursor_right();
    }
  }
}
