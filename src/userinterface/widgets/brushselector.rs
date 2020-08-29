extern crate tui;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES,
  RGB_SUM_DIVIDED_BY_2};

use crate::log;
use crate::utils::FullPixel;

const RGB_LABEL_HEIGHT: u16 = 3;
pub const LABEL_WIDTH: u16 = 3;
pub const TITLE_HEIGHT: u16 = 1;
pub const NB_BRUSHES: u16 = 5;
pub const SELECTOR_HEIGHT: u16 = BORDERS + 1;
pub const SELECTOR_WIDTH: u16 = BORDERS + EMPTY_SPACES + LABEL_WIDTH;
pub const RGB_BOX_WIDTH: u16 = BORDERS + EMPTY_SPACES + LABEL_WIDTH;
pub const RGB_BOX_HEIGHT: u16 = BORDERS + RGB_LABEL_HEIGHT;

/// Widget to render a brush selector section. The boxed area is the current
/// brush action. The boxed numbers are the RGB values of the Specific Color
/// brush action.
pub struct BrushSelectorWidget<'a> {
  log: &'a log::Log,
}

impl<'a> BrushSelectorWidget<'a> {
  pub fn new(log: &'a log::Log) -> BrushSelectorWidget<'a> {
    BrushSelectorWidget {
      log: log,
    }
  }

  fn render_brush(&self, buf: &mut Buffer, (left, top): &(u16, u16)) {

    let (red, green, blue) = self.log.brush_getcolor::<u16>();
    let fg_color = if red + green + blue > RGB_SUM_DIVIDED_BY_2 {
      Color::Black
    } else {
      Color::White
    };
    let (red, green, blue) = self.log.brush_getcolor::<u8>();

    buf.set_string(left + BORDERS + 1, top + TITLE_HEIGHT + 1, "D|E",
      Style::default().bg(Color::Green));
    buf.set_string(left + BORDERS + 1,
      top + TITLE_HEIGHT + 1 + SELECTOR_HEIGHT, "D|R",
      Style::default().bg(Color::Blue));
    buf.set_string(left + BORDERS + 1,
      top + TITLE_HEIGHT + 1 + SELECTOR_HEIGHT * 2, " R ",
      Style::default().bg(Color::Red));
    buf.set_string(left + BORDERS + 1,
      top + TITLE_HEIGHT + 1 + SELECTOR_HEIGHT * 3, " D ",
      Style::default().bg(Color::Yellow));
    buf.set_string(left + BORDERS + 1,
      top + TITLE_HEIGHT + 1 + SELECTOR_HEIGHT * 4, " C ",
      Style::default().bg(Color::Rgb(red, green, blue)).fg(fg_color));
  }

  fn render_color(&self, buf: &mut Buffer, (left, top): &(u16, u16)) {
    let (red, green, blue) = self.log.brush_getcolor::<u16>();
    let color = format!("{}{}{}", 1000 + red, 1000 + green, 1000 + blue);
    buf.set_string(left + BORDERS + 1, top + 1, &color[1..4],
      Style::default().fg(Color::Red));
    buf.set_string(left + BORDERS + 1, top + 2, &color[5..8],
      Style::default().fg(Color::Green));
    buf.set_string(left + BORDERS + 1, top + 3, &color[9..12],
      Style::default().fg(Color::Blue));
    let borders = Block::default().title("  C ").borders(Borders::ALL);
    let area = Rect::new(left + 1, *top, RGB_BOX_WIDTH, RGB_BOX_HEIGHT);
    borders.render(area, buf);
    buf.get_mut(left + 2, *top).set_symbol("─");
  }

  fn render_selector(&self, buf: &mut Buffer, (left, top): &(u16, u16)) {

    let borders = Block::default().borders(Borders::ALL);
    let current_brush = match self.log.brush_getcurrentaction() {
      FullPixel::BodyEmpty => 0,
      FullPixel::BodyBorder => 1,
      FullPixel::Border => 2,
      FullPixel::Body => 3,
      FullPixel::SpecificColor(_, _, _) => 4,
    };
    let area = Rect::new(left + 1, top + 1 + SELECTOR_HEIGHT * current_brush,
      SELECTOR_WIDTH, SELECTOR_HEIGHT);
    borders.render(area, buf);
  }
}

impl<'a> Widget for BrushSelectorWidget<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let left = area.left();
    let top = area.top();

    let brush_borders = Block::default().title(" Brush ").borders(Borders::ALL);

    brush_borders.render(area, buf);

    self.render_brush(buf, &(left, top));
    self.render_color(buf, &(left, area.bottom() - RGB_BOX_HEIGHT - 1));
    self.render_selector(buf, &(left, top));
  }
}
