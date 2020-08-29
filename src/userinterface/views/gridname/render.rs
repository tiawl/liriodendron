extern crate tui;
use tui::Terminal;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

use crate::log;

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, querierpopup};

pub const MAX_BOX_HEIGHT: u16 = BORDERS + 3;
pub const TEXT_HEIGHT: u16 = 6;

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 29;

pub const WIN_WIDTH: u16 = LARGER_ROW + (EXTRA_SPACES + EMPTY_SPACES) * 2 +
  BORDERS;

pub const WIN_HEIGHT: u16 = TEXT_HEIGHT + EMPTY_SPACES + BORDERS +
  MAX_BOX_HEIGHT + 1;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, log: &mut log::Log) {
  terminal.draw(|frame| {
    let frame_area = frame.size();

    let frame_left = frame_area.left();
    let frame_top = frame_area.top();
    let frame_width = frame_area.right() - frame_left;
    let frame_height = frame_area.bottom() - frame_top;

    let window_area = Rect::new(frame_left + frame_width/2 - WIN_WIDTH/2,
      frame_top + frame_height/2 - WIN_HEIGHT/2, WIN_WIDTH, WIN_HEIGHT);

    let keyboard_instructions = Text::from("Choose the name of your grid
Max size: 16
Authorized characters:
< a-z >, < A-Z > and < 0-9 >
< Enter > when name is chosen
< Esc > to exit");

    let gridname;
    if !log.grids_erroroccured() {
      gridname = vec![Spans::from(vec![Span::raw(log.gridname_getvalue())])];
    } else {
      gridname = vec![
        Spans::from(vec![Span::raw(log.gridname_getvalue())]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(format!("{}", log.grids_geterror()),
          Style::default().fg(Color::Red))]),
      ];
    }

    frame.render_widget(querierpopup::QuerierPopupWidget::new(
      keyboard_instructions, gridname, log.gridname_getcursor()),
      window_area);
  }).unwrap();
}
