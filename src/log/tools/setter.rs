extern crate std;
use std::time::Instant;

pub trait Setter {
  fn set_value(&mut self, value: char);
}

/// A numeric cursor which encapsulates shared functions between Setter
/// sub-structs
pub struct Cursor {
  pub pos: usize,
  pub value: Vec<char>,
  pub min_value: u16,
  pub max_value: u16,
  pub instant: Instant,
}

impl Cursor {
  pub fn new(default_value: Vec<char>, min: u16, max: u16) -> Cursor {
    Cursor {
      pos: 0,
      value: default_value,
      min_value: min,
      max_value: max,
      instant: Instant::now(),
    }
  }

  pub fn get_pos(&self) -> usize {
    self.pos
  }

  pub fn cursor_is_blinking(&self) -> bool {
    (self.instant.elapsed().as_millis() % 1000) < 500
  }

  pub fn reset_cursor(&mut self) {
    self.pos = 0;
    self.instant = Instant::now();
  }

  pub fn cursor_left(&mut self) {
    if self.pos > 0 {
      self.pos -= 1;
    }
    self.instant = Instant::now();
  }

  pub fn cursor_right(&mut self) {
    if self.pos < self.value.len() - 1 {
      self.pos += 1;
    }
    self.instant = Instant::now();
  }
}

/// An alphabetic cursor which encapsulates shared functions between Setter
/// sub-structs
pub struct StringCursor {
  pub pos: usize,
  pub value: String,
  pub max_length: usize,
  pub instant: Instant,
}

impl StringCursor {
  pub fn new(max_length: usize) -> StringCursor {
    StringCursor {
      pos: 0,
      value: String::new(),
      max_length: max_length,
      instant: Instant::now(),
    }
  }

  pub fn get_pos(&self) -> usize {
    self.pos
  }

  pub fn cursor_is_blinking(&self) -> bool {
    (self.instant.elapsed().as_millis() % 1000) < 500
  }

  pub fn cursor_left(&mut self) {
    if self.pos > 0 {
      self.pos -= 1;
    }
    self.instant = Instant::now();
  }

  pub fn cursor_right(&mut self) {
    if self.pos < self.value.len() {
      self.pos += 1;
    }
    self.instant = Instant::now();
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_sets_cursor_to_the_right() {
    let mut cursor = Cursor::new(vec!['0', '1', '2', '3', '4'], 0, 10000);
    let init_pos = cursor.get_pos();
    cursor.cursor_right();
    let second_pos = cursor.get_pos();
    let value_len = cursor.value.len() - 1;
    for _i in 2..=value_len {
      cursor.cursor_right();
    }
    let third_pos = cursor.get_pos();
    cursor.cursor_right();
    assert!((init_pos == 0) && (second_pos == 1) &&
      (third_pos == value_len) && (cursor.get_pos() == value_len));
  }

  #[test]
  fn it_sets_cursor_to_the_left() {
    let mut cursor = Cursor::new(vec!['0', '1', '2', '3', '4'], 0, 10000);
    cursor.cursor_right();
    let init_pos = cursor.get_pos();
    cursor.cursor_left();
    let second_pos = cursor.get_pos();
    cursor.cursor_left();
    assert!((init_pos == 1) && (second_pos == 0) &&
      (cursor.get_pos() == 0));
  }

  #[test]
  fn it_resets_cursor() {
    let mut cursor = Cursor::new(vec!['0', '1', '2', '3', '4'], 0, 10000);
    let value_len = cursor.value.len() - 1;
    for _i in 1..=value_len {
      cursor.cursor_right();
    }
    let init_pos = cursor.get_pos();
    cursor.reset_cursor();
    assert!((init_pos == value_len) && (cursor.get_pos() == 0));
  }
}
