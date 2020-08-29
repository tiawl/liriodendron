extern crate std;
use std::ops::{Deref, DerefMut};

use crate::log::tools::setter;

/// Represents a customizable number of generated textures
pub struct GenerationNumber {
  cursor: setter::Cursor,
}

/// Tips to use shared functions between Setter sub-structs
impl Deref for GenerationNumber {
  type Target = setter::Cursor;

  fn deref(&self) -> &Self::Target {
    &self.cursor
  }
}

/// Tips to use shared functions between Setter sub-structs
impl DerefMut for GenerationNumber {

  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.cursor
  }
}

impl GenerationNumber {
  pub fn new() -> GenerationNumber {
    GenerationNumber {
      cursor: setter::Cursor::new(vec!['0', '0', '1'], 1, 500),
    }
  }

  pub fn get_value(&self) -> String {
    self.value.iter().collect()
  }
}

impl setter::Setter for GenerationNumber {

  /// Checks value and set it
  fn set_value(&mut self, number: char) {
    let mut value = self.get_value();
    let pos = self.pos;
    value.remove(pos);
    value.insert(pos, number);
    let value = value.parse::<u16>().unwrap();
    if (value <= self.max_value) && (value >= self.min_value) {
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
  fn it_sets_number_with_the_char_sequences_56789400_0512345678901234567890_00_0() {
    let mut generationnumber = GenerationNumber::new();
    let first_number = "56789400";
    let first_res = String::from("400");
    for number in first_number.chars() {
      generationnumber.set_value(number);
    }
    let first_test = first_res == generationnumber.get_value();
    generationnumber.cursor_left();
    generationnumber.cursor_left();
    let second_number = "0512345678901234567890";
    let second_res = String::from("500");
    for number in second_number.chars() {
      generationnumber.set_value(number);
    }
    let second_test = second_res == generationnumber.get_value();
    generationnumber.cursor_left();
    generationnumber.cursor_left();
    generationnumber.set_value('1');
    generationnumber.set_value('1');
    generationnumber.cursor_left();
    generationnumber.cursor_left();
    generationnumber.set_value('0');
    generationnumber.set_value('0');
    let third_res = String::from("010");
    let third_test = third_res == generationnumber.get_value();
    generationnumber.cursor_right();
    generationnumber.set_value('1');
    generationnumber.cursor_left();
    generationnumber.set_value('0');
    generationnumber.set_value('0');
    let forth_res = String::from("001");
    let forth_test = forth_res == generationnumber.get_value();
    assert!(first_test && second_test && third_test && forth_test);
  }
}
