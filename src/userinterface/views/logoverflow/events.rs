extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode};

extern crate std;
use std::time::Duration;

use crate::events::UserEvent;
use crate::errors::UserError;

pub fn catch_events() -> (UserEvent, UserError) {
  if poll(Duration::from_nanos(1)).unwrap() {
    match read().unwrap() {
      Event::Key(key_event) => {
        if key_event.modifiers.is_empty() {
          match key_event.code {
            KeyCode::Enter => {
              return (UserEvent::Continue, UserError::ResizeCheck)
            },
            _ => (),
          }
        }
      },
      Event::Resize(_, _) => {
        return (UserEvent::LogOverflow, UserError::ResizeCheck)
      },
      _ => {},
    }
  }
  (UserEvent::LogOverflow, UserError::NoneError)
}
