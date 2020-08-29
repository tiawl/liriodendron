extern crate tui;
use tui::Terminal;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::text::Text;

use crate::log;

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, selectorpopup};

pub const BOX_HEIGHT: u16 = 11;
pub const TEXT_HEIGHT: u16 = 7;

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 34;

pub const WIN_WIDTH: u16 = LARGER_ROW + (EXTRA_SPACES + EMPTY_SPACES) * 2 +
  BORDERS;

pub const WIN_HEIGHT: u16 = TEXT_HEIGHT + EMPTY_SPACES + BORDERS +
  BOX_HEIGHT + 1;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, log: &mut log::Log) {
  terminal.draw(|frame| {
    let frame_area = frame.size();

    let frame_left = frame_area.left();
    let frame_top = frame_area.top();
    let frame_width = frame_area.right() - frame_left;
    let frame_height = frame_area.bottom() - frame_top;

    let window_area = Rect::new(frame_left + frame_width/2 - WIN_WIDTH/2,
      frame_top + frame_height/2 - WIN_HEIGHT/2, WIN_WIDTH, WIN_HEIGHT);

    let keyboard_instructions = Text::from("More a grid is near to the top
of the stack more the content of
this grid is important than other
With which grid do you want switch
the order of the current grid ?
< ↑  | ↓  > to move,
< Enter > to make your choice");

    let current_grid = log.grids_getcurrentgridid();
    let mut i = 0;
    let mut names = log.grids_getnames();
    names.retain(|_| (i != current_grid, i += 1).0);

    let cursor = log.grids_getswitchcursor();

    frame.render_widget(selectorpopup::SelectorPopupWidget::new(
      keyboard_instructions, names, cursor), window_area);
  }).unwrap();
}
