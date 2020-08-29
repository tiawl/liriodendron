extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode};

extern crate std;
use std::time::Duration;

use crate::log;

use crate::events::UserEvent;
use crate::errors::UserError;

pub fn catch_events(log: &mut log::Log) -> (UserEvent, UserError) {
  if poll(Duration::from_nanos(1)).unwrap() {
    match read().unwrap() {
      Event::Key(key_event) => {
        if key_event.modifiers.is_empty() {
          match key_event.code {
            KeyCode::Enter => {
              log.grids_switchorder();
              return (UserEvent::Continue, UserError::ResizeCheck)
            },
            KeyCode::Up => {
              log.grids_previousswitchcursor();
            },
            KeyCode::Down => {
              log.grids_nextswitchcursor();
            },
            _ => (),
          }
        }
      },
      Event::Resize(_, _) => {
        return (UserEvent::SwitchGridsOrder, UserError::ResizeCheck)
      },
      _ => {},
    }
  }
  (UserEvent::SwitchGridsOrder, UserError::NoneError)
}
