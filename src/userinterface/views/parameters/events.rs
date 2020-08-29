extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

extern crate std;
use std::time::Duration;

use crate::log;

use crate::events::UserEvent;
use crate::errors::UserError;
use crate::views::View;

pub fn catch_events(log: &mut log::Log) -> (UserEvent, UserError, View) {
  if poll(Duration::from_nanos(1)).unwrap() {
    match read().unwrap() {
      Event::Key(key_event) => {
        if key_event.modifiers.is_empty() {
          match key_event.code {
            KeyCode::Esc => {
              return (UserEvent::Exit, UserError::ResizeCheck,
                View::Parameters)
            },
            KeyCode::Char('g') => {
              return (UserEvent::SetGenerationNumber,
                UserError::ResizeCheck, View::Parameters)
            },
            KeyCode::Char('s') => {
              return (UserEvent::Continue,
                UserError::ResizeCheck, View::Edit)
            },
            KeyCode::Char('b') => {
              log.brush_incrbodyderatio();
            },
            KeyCode::Char('n') => {
              log.brush_incrbodydrratio();
            },
            KeyCode::Char('f') => {
              return (UserEvent::SetTextureFormat, UserError::ResizeCheck,
                View::Parameters)
            },
            KeyCode::Char('p') => {
              log.texturessettings_incrpixelratio();
            },
            _ => (),
          }
        } else if key_event.modifiers == KeyModifiers::SHIFT {
          match key_event.code {
            KeyCode::Char('B') => {
              log.brush_decrbodyderatio();
            },
            KeyCode::Char('N') => {
              log.brush_decrbodydrratio();
            },
            KeyCode::Char('P') => {
              log.texturessettings_decrpixelratio();
            },
            _ => (),
          }
        }
      },
      Event::Resize(_, _) => {
        return (UserEvent::Continue, UserError::ResizeCheck, View::Parameters)
      },
      _ => {},
    }
  }
  (UserEvent::Continue, UserError::ResizeCheck, View::Parameters)
}
