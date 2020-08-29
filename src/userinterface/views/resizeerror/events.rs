extern crate crossterm;
use crossterm::event::{poll, read, Event};

extern crate std;
use std::time::Duration;

use crate::errors::UserError;

pub fn catch_events() -> UserError {
  if poll(Duration::from_nanos(1)).unwrap() {
    match read().unwrap() {
      Event::Resize(_, _) => {
        return UserError::ResizeCheck
      },
      _ => {},
    }
  }
  UserError::ResizeError
}
