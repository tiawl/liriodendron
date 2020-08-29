extern crate std;
use std::ops::{Deref, DerefMut};

use crate::log::tools::setter;

/// Represents a customizable and usable color with brush tool
pub struct BrushColor {
  cursor: setter::Cursor,
}

/// Tips to use shared functions between Setter sub-structs
impl Deref for BrushColor {
  type Target = setter::Cursor;

  fn deref(&self) -> &Self::Target {
    &self.cursor
  }
}

/// Tips to use shared functions between Setter sub-structs
impl DerefMut for BrushColor {

  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.cursor
  }
}

impl BrushColor {
  pub fn new() -> BrushColor {
    BrushColor {
      cursor: setter::Cursor::new(
        vec!['2', '5', '5', '2', '5', '5', '2', '5', '5'], 0, 255),
    }
  }

  pub fn get_value(&self) -> (String, String, String) {
    let mut red = self.value[0].to_string();
    red.push(self.value[1]);
    red.push(self.value[2]);
    let mut green = self.value[3].to_string();
    green.push(self.value[4]);
    green.push(self.value[5]);
    let mut blue = self.value[6].to_string();
    blue.push(self.value[7]);
    blue.push(self.value[8]);
    (red, green, blue)
  }
}

impl setter::Setter for BrushColor {

  /// Checks value and set it
  fn set_value(&mut self, number: char) {
    let mut value: String;
    let pos = self.pos;
    if pos <= 2 {
      value = self.value[0].to_string();
      value.push(self.value[1]);
      value.push(self.value[2]);
    } else if pos >= 3 && pos <= 5 {
      value = self.value[3].to_string();
      value.push(self.value[4]);
      value.push(self.value[5]);
    } else {
      value = self.value[6].to_string();
      value.push(self.value[7]);
      value.push(self.value[8]);
    }
    let mut relative_pos = pos;
    while relative_pos > 2 {
      relative_pos -= 3;
    }
    value.remove(relative_pos);
    value.insert(relative_pos, number);
    if value.parse::<u16>().unwrap() <= self.max_value {
      self.value[pos] = number;
      self.cursor_right();
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::log::tools::setter::Setter;

  #[test]
  fn it_sets_color_with_the_char_sequence_123_34567892_67895_67895_000() {
    let mut brushcolor = BrushColor::new();
    let color = "123345678926789567895000";
    let res = (String::from("123"), String::from("255"), String::from("000"));
    for number in color.chars() {
      brushcolor.set_value(number);
    }
    assert_eq!(brushcolor.get_value(), res);
  }
}
