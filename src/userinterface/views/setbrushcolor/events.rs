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
              log.brushcolor_resetcursor();
              log.brush_setcolor();
              return (UserEvent::Continue, UserError::ResizeCheck)
            },
            KeyCode::Left => {
              log.brushcolor_cursorleft();
            },
            KeyCode::Right => {
              log.brushcolor_cursorright();
            },
            KeyCode::Char(n) if n.is_ascii_digit() => {
              log.brushcolor_setvalue(n);
            },
            _ => (),
          }
        }
      },
      Event::Resize(_, _) => {
        return (UserEvent::SetBrushColor, UserError::ResizeCheck)
      },
      _ => {},
    }
  }
  (UserEvent::SetBrushColor, UserError::NoneError)
}
