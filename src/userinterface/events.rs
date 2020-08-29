extern crate crossterm;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;

extern crate std;
use std::io::Write;

#[derive(Clone, Copy)]
pub enum UserEvent {
  Continue,
  Exit,
  Generate,
  GenerationError,
  GridName,
  GridNumberError,
  LogOverflow,
  RenameGrid,
  SetBrushColor,
  SetGenerationNumber,
  SetTextureFormat,
  Shutdown,
  SwitchGridsError,
  SwitchGridsOrder,
  UnavailableThread,
}

pub fn disable_mouse_capture(stdout: &mut std::io::Stdout) {
  execute!(stdout, DisableMouseCapture).unwrap();
}

pub fn enable_mouse_capture(stdout: &mut std::io::Stdout) {
  execute!(stdout, EnableMouseCapture).unwrap();
}
