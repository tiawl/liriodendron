extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers, MouseEvent,
  MouseButton};

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
              return (UserEvent::Exit, UserError::ResizeCheck, View::Edit)
            },
            KeyCode::Char('a') => {
              log.brush_previous();
            },
            KeyCode::Char('c') => {
              return (UserEvent::SetBrushColor,
                UserError::ResizeCheck, View::Edit)
            },
            KeyCode::Char('g') => {
              return (UserEvent::SetGenerationNumber,
                UserError::ResizeCheck, View::Edit)
            },
            KeyCode::Char('h') => {
              log.increment_height();
            },
            KeyCode::Char('n') => {
              log.grids_next();
            },
            KeyCode::Char('q') => {
              log.brush_next();
            },
            KeyCode::Char('s') => {
              return (UserEvent::Continue,
                UserError::ResizeCheck, View::Parameters)
            },
            KeyCode::Char('r') => {
              return (UserEvent::RenameGrid,
                UserError::ResizeCheck, View::Edit)
            },
            KeyCode::Char('u') => {
              log.undo();
            },
            KeyCode::Char('w') => {
              log.increment_width();
            },
            KeyCode::Char('+') => {
              if !log.grids_isfull() {
                return (UserEvent::GridName,
                  UserError::ResizeCheck, View::Edit)
              } else {
                return (UserEvent::GridNumberError,
                  UserError::ResizeCheck, View::Edit)
              }
            },
            KeyCode::Char('-') => {
              if !log.grids_isalone() {
                log.grids_deletecurrentgrid();
              } else {
                return (UserEvent::GridNumberError,
                  UserError::ResizeCheck, View::Edit)
              }
            },
            KeyCode::Up => {
              log.gridscroll_scrollup();
            },
            KeyCode::Down => {
              log.gridscroll_scrolldown();
            },
            KeyCode::Right => {
              log.gridscroll_scrollright();
            },
            KeyCode::Left => {
              log.gridscroll_scrollleft();
            },
            _ => (),
          }
        } else if key_event.modifiers == KeyModifiers::SHIFT {
          match key_event.code {
            KeyCode::Char('C') => {
              log.clear();
            },
            KeyCode::Char('H') => {
              log.decrement_height();
            },
            KeyCode::Char('N') => {
              log.grids_previous();
            },
            KeyCode::Char('S') => {
              if !log.grids_isalone() {
                return (UserEvent::SwitchGridsOrder,
                  UserError::ResizeCheck, View::Edit)
              } else {
                return (UserEvent::SwitchGridsError,
                  UserError::ResizeCheck, View::Edit)
              }
            },
            KeyCode::Char('U') => {
              log.redo();
            },
            KeyCode::Char('W') => {
              log.decrement_width();
            },
            _ => (),
          }
        }
      }
      Event::Mouse(mouse_event) => match mouse_event {
        MouseEvent::Down(MouseButton::Left, x, y, _) |
          MouseEvent::Drag(MouseButton::Left, x, y, _) => {
            log.brush((x, y));
        },
        MouseEvent::Down(MouseButton::Right, x, y, _) |
          MouseEvent::Drag(MouseButton::Right, x, y, _) => {
            log.erase((x, y));
        },
        _ => {}
      },
      Event::Resize(_, _) => {
        if log.overflow() {
          return (UserEvent::LogOverflow, UserError::ResizeCheck, View::Edit)
        } else {
          return (UserEvent::Continue, UserError::ResizeCheck, View::Edit)
        }
      },
    }
  }
  if log.overflow() {
    (UserEvent::LogOverflow, UserError::ResizeCheck, View::Edit)
  } else {
    (UserEvent::Continue, UserError::NoneError, View::Edit)
  }
}
