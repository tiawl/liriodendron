extern crate tui;
use tui::Terminal;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

use crate::log;

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, querierpopup};

pub const RGB_BOX_HEIGHT: u16 = BORDERS + 1;
pub const TEXT_HEIGHT: u16 = 3;

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 31;

pub const WIN_WIDTH: u16 = LARGER_ROW + (EXTRA_SPACES + EMPTY_SPACES) * 2 +
  BORDERS;

pub const WIN_HEIGHT: u16 = TEXT_HEIGHT + EMPTY_SPACES + BORDERS +
  RGB_BOX_HEIGHT + 1;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, log: &mut log::Log) {
  terminal.draw(|frame| {
    let frame_area = frame.size();

    let frame_left = frame_area.left();
    let frame_top = frame_area.top();
    let frame_width = frame_area.right() - frame_left;
    let frame_height = frame_area.bottom() - frame_top;

    let window_area = Rect::new(frame_left + frame_width/2 - WIN_WIDTH/2,
      frame_top + frame_height/2 - WIN_HEIGHT/2, WIN_WIDTH, WIN_HEIGHT);

    let keyboard_instructions = Text::from("< 0-9 > to modify RGB values,
< ←  | →  > to move,
< Enter > to save modifications");

    let (red, green, blue) = log.brushcolor_getvalue();
    let rgb_value = vec![Spans::from(vec![
        Span::styled(red, Style::default().fg(Color::Red)),
        Span::styled(green, Style::default().fg(Color::Green)),
        Span::styled(blue, Style::default().fg(Color::Blue)),
      ])];

    frame.render_widget(querierpopup::QuerierPopupWidget::new(
      keyboard_instructions, rgb_value, log.brushcolor_getcursor()),
      window_area);
  }).unwrap();
}
