extern crate tui;
use tui::Terminal;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::text::Text;

use crate::log;

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, selectorpopup};

pub const BOX_HEIGHT: u16 = 3;
pub const TEXT_HEIGHT: u16 = 2;

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 31;

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

    let keyboard_instructions = Text::from("< ↑  | ↓  > to move,
< Enter > to save modifications");

    let (format, cursor) = log.texturessettings_getformatinfo();
    let format = format.iter()
      .map(|key| format!("{:?}", key).to_uppercase()).collect::<Vec<String>>();

    frame.render_widget(selectorpopup::SelectorPopupWidget::new(
      keyboard_instructions, format, cursor), window_area);
  }).unwrap();
}
