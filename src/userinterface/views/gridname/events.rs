extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode};

extern crate std;
use std::time::Duration;

use crate::log;

use crate::events::UserEvent;
use crate::errors::UserError;

pub fn catch_events(log: &mut log::Log, event: UserEvent) -> (UserEvent, UserError) {
  if poll(Duration::from_nanos(1)).unwrap() {
    match read().unwrap() {
      Event::Key(key_event) => {
        match key_event.code {
          KeyCode::Enter => {
            if let UserEvent::GridName = event {
              log.grids_add();
            } else if let UserEvent::RenameGrid = event {
              log.grids_renamecurrentgrid();
            }
            if !log.grids_erroroccured() {
              log.gridname_reset();
              return (UserEvent::Continue, UserError::ResizeCheck)
            }
          },
          KeyCode::Esc => {
            log.gridname_reset();
            return (UserEvent::Continue, UserError::ResizeCheck)
          },
          KeyCode::Left => {
            log.gridname_cursorleft();
          },
          KeyCode::Right => {
            log.gridname_cursorright();
          },
          KeyCode::Char(n) if n.is_ascii_digit() || n.is_ascii_alphabetic() => {
            log.gridname_setvalue(n);
          },
          KeyCode::Backspace => {
            log.gridname_backspace();
          },
          _ => (),
        }
      },
      Event::Resize(_, _) => {
        return (event, UserError::ResizeCheck)
      },
      _ => {},
    }
  }
  (event, UserError::NoneError)
}
