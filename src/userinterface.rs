extern crate better_panic;

extern crate crossterm;
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{self, size};

extern crate std;
use std::cmp::max;
use std::io::Write;

pub mod errors;
pub mod events;

/// Module for rendering and events catching of the different views
pub mod views;

/// Module for tui-rs Widget sub-structs
mod widgets;

pub struct UserInterface {
  event: events::UserEvent,
  error: errors::UserError,
  view: views::View,
}

impl UserInterface {

  pub fn new() -> UserInterface {
    UserInterface {
      view: views::View::Edit,
      event: events::UserEvent::Continue,
      error: UserInterface::check_terminal_size(
        events::UserEvent::Continue, views::View::Edit),
    }
  }

  fn cmp((terminal_width, terminal_height): (u16, u16),
    (min_width, min_height): (u16, u16)) -> errors::UserError {
      if (terminal_height < min_height) ||
        (terminal_width < min_width) {
          errors::UserError::ResizeError
      } else {
        errors::UserError::NoneError
      }
  }

  pub fn check_terminal_size(event: events::UserEvent, view: views::View)
    -> errors::UserError {
      let terminal_size = size().unwrap();
      match event {
        events::UserEvent::Continue => {
          match view {
            views::View::Edit => {
              let min_width = max(views::MIN_WIDTH_EDIT, views::MIN_WIDTH);
              let min_height = max(views::MIN_BRUSH_AREA_HEIGHT,
                max(views::MIN_SHORTCUTS_AREA_HEIGHT, max(views::MIN_HEIGHT,
                views::MIN_GRID_AREA_HEIGHT)));
              UserInterface::cmp(terminal_size, (min_width, min_height))
            },
            views::View::Parameters => {
              let min_width = max(views::MIN_SHORTCUTS_AREA_WIDTH +
                max(max(max(max(views::MIN_WIDTH_TEXTURE, views::MIN_WIDTH_COLOR),
                  views::MIN_WIDTH_BRUSH_PARAM), views::MIN_WIDTH_GRID_PARAM),
                    views::MIN_WIDTH_PIXEL) * 2,views::MIN_WIDTH);
              let min_height = max(views::MIN_HEIGHT, views::MIN_HEIGHT_PARAM);
              UserInterface::cmp(terminal_size, (min_width, min_height))
            },

          }
        },
        events::UserEvent::Exit => {
          let min_width = max(views::MIN_WIDTH_EXIT, views::MIN_WIDTH);
          let min_height = max(views::MIN_HEIGHT_EXIT, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::SetBrushColor => {
          let min_width =
            max(views::MIN_WIDTH_SETBRUSHCOLOR, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SETBRUSHCOLOR, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::SetGenerationNumber => {
          let min_width =
            max(views::MIN_WIDTH_SETGENERATIONNUMBER, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SETGENERATIONNUMBER, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::SetTextureFormat => {
          let min_width =
            max(views::MIN_WIDTH_SETTEXTUREFORMAT, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SETTEXTUREFORMAT, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::GridName | events::UserEvent::RenameGrid => {
          let min_width = max(views::MIN_WIDTH_GRIDNAME, views::MIN_WIDTH);
          let min_height = max(views::MIN_HEIGHT_GRIDNAME, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::GridNumberError => {
          let min_width =
            max(views::MIN_WIDTH_GRIDNUMBERERROR, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_GRIDNUMBERERROR, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::GenerationError => {
          let min_width =
            max(views::MIN_WIDTH_GENERATIONERROR, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_GENERATIONERROR, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::LogOverflow => {
          let min_width = max(views::MIN_WIDTH_LOGOVERFLOW, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_LOGOVERFLOW, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::UnavailableThread => {
          let min_width =
            max(views::MIN_WIDTH_UNAVAILABLETHREAD, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_UNAVAILABLETHREAD, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::Shutdown => {
          let min_width =
            max(views::MIN_WIDTH_SHUTDOWN, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SHUTDOWN, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::SwitchGridsOrder => {
          let min_width =
            max(views::MIN_WIDTH_SWITCHGRIDSORDER, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SWITCHGRIDSORDER, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::SwitchGridsError => {
          let min_width =
            max(views::MIN_WIDTH_SWITCHGRIDSERROR, views::MIN_WIDTH);
          let min_height =
            max(views::MIN_HEIGHT_SWITCHGRIDSERROR, views::MIN_HEIGHT);
          UserInterface::cmp(terminal_size, (min_width, min_height))
        },
        events::UserEvent::Generate => errors::UserError::NoneError,
      }
  }

  pub fn get_event(&self) -> events::UserEvent {
    self.event
  }

  pub fn get_view(&self) -> views::View {
    self.view
  }

  pub fn get_error(&self) -> errors::UserError {
    self.error
  }

  pub fn set_event(&mut self, event: events::UserEvent) {
    self.event = event;
  }

  pub fn set_error(&mut self, error: errors::UserError) {
    self.error = error;
  }

  fn set_view(&mut self, view: views::View) {
    self.view = view;
  }

  pub fn set_popupview(&mut self, (event, error):
    (events::UserEvent, errors::UserError)) {
      self.set_event(event);
      self.set_error(error);
  }

  pub fn set(&mut self, (event, error, view):
    (events::UserEvent, errors::UserError, views::View)) {
      self.set_event(event);
      self.set_error(error);
      self.set_view(view);
  }
}

pub fn init() {
  let mut stdout = std::io::stdout();
  terminal::enable_raw_mode().unwrap();
  execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
  execute!(stdout, terminal::EnterAlternateScreen).unwrap();
  execute!(stdout, cursor::Hide).unwrap();
  events::enable_mouse_capture(&mut stdout);
}

pub fn finish() {
  let mut stdout = std::io::stdout();
  events::disable_mouse_capture(&mut stdout);
  execute!(stdout, cursor::Show).unwrap();
  execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
  execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
  execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
  terminal::disable_raw_mode().unwrap();
}
