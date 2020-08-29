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
              log.generationnumber_resetcursor();
              log.texturessettings_setgenerationnumber();
              return (UserEvent::Generate, UserError::ResizeCheck)
            },
            KeyCode::Esc => {
              log.generationnumber_resetcursor();
              return (UserEvent::Continue, UserError::ResizeCheck)
            }
            KeyCode::Left => {
              log.generationnumber_cursorleft();
            },
            KeyCode::Right => {
              log.generationnumber_cursorright();
            },
            KeyCode::Char(n) if n.is_ascii_digit() => {
              log.generationnumber_setvalue(n);
            },
            _ => (),
          }
        }
      },
      Event::Resize(_, _) => {
        return (UserEvent::SetGenerationNumber, UserError::ResizeCheck)
      },
      _ => {},
    }
  }
  (UserEvent::SetGenerationNumber, UserError::NoneError)
}
