extern crate tui;
use tui::buffer::Buffer;
use tui::layout::{Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::userinterface::widgets::BORDERS;

pub const SCROLLER: u16 = 1;
pub const ARROWS: u16 = 2;

const GREY: Color = Color::Rgb(150, 150, 150);

/// Widget to render a vertical of horizotal scroller
pub struct ScrollerWidget {
  direction: tui::layout::Direction,
  scroll: u16,
  section_size: u16,
}

impl ScrollerWidget {

  pub fn new(direction: tui::layout::Direction, scroll: u16, section_size: u16)
    -> ScrollerWidget {
      ScrollerWidget {
        direction: direction,
        scroll: scroll,
        section_size: section_size,
      }
  }

  fn compute_size(&self, scroller_part: u16,  scroller_size: u16) -> u16 {
    ((scroller_part * scroller_size) as f64 / self.section_size as f64)
      .round() as u16
  }

  fn render_vertical_scroller(&self, scroller: Rect, buf: &mut Buffer) {
    let (scroller_left, scroller_top, scroller_bottom) =
      (scroller.left(), scroller.top(), scroller.bottom());
    let scroller_height = scroller_bottom - scroller_top;
    let borders_scroller_bottom = scroller_bottom + BORDERS;

    buf.get_mut(scroller_left, scroller_top).set_symbol("▴")
      .set_style(Style::default().bg(Color::Reset).fg(GREY));
    buf.get_mut(scroller_left, borders_scroller_bottom - 1).set_symbol("▾")
      .set_style(Style::default().bg(Color::Reset).fg(GREY));

    let first_height = self.compute_size(self.scroll, scroller_height);
    let second_height = self.compute_size(scroller_height, scroller_height);

    let first_area =
      Rect::new(scroller_left, scroller_top + 1, SCROLLER, first_height);
    buf.set_style(first_area, Style::default().bg(Color::Black));

    let second_area =
      Rect::new(scroller_left, scroller_top + 1 + first_height,
        SCROLLER, second_height);
    buf.set_style(second_area, Style::default().bg(GREY));

    let third_area =
      Rect::new(scroller_left, scroller_top + 1 + first_height + second_height,
        SCROLLER, scroller_height - first_height - second_height);
    buf.set_style(third_area, Style::default().bg(Color::Black));
  }

  fn render_horizontal_scroller(&self, scroller: Rect, buf: &mut Buffer) {
    let (scroller_left, scroller_right, scroller_top) =
      (scroller.left(), scroller.right(), scroller.top());
    let scroller_width = scroller_right - scroller_left;
    let borders_scroller_right = scroller_right + BORDERS;

    buf.get_mut(scroller_left - 1, scroller_top).set_symbol("◂")
      .set_style(Style::default()
        .add_modifier(Modifier::REVERSED).bg(Color::Reset).fg(GREY));
    buf.get_mut(scroller_left, scroller_top).set_symbol("▌")
      .set_style(Style::default().fg(GREY));
    buf.get_mut(borders_scroller_right, scroller_top).set_symbol("▸")
      .set_style(Style::default()
        .add_modifier(Modifier::REVERSED).bg(Color::Reset).fg(GREY));
    buf.get_mut(borders_scroller_right - 1, scroller_top).set_symbol("▐")
      .set_style(Style::default().fg(GREY));

    let first_width = self.compute_size(self.scroll, scroller_width);
    let second_width = self.compute_size(scroller_width, scroller_width);

    let first_area =
      Rect::new(scroller_left + 1, scroller_top, first_width, SCROLLER);
    buf.set_style(first_area, Style::default().bg(Color::Black));

    let second_area = Rect::new(scroller_left + 1 + first_width, scroller_top,
      second_width, SCROLLER);
    buf.set_style(second_area, Style::default().bg(GREY));

    let third_area =
      Rect::new(scroller_left + 1 + first_width + second_width, scroller_top,
        scroller_width - first_width - second_width, SCROLLER);
    buf.set_style(third_area, Style::default().bg(Color::Black));
  }
}

impl Widget for ScrollerWidget {

  fn render(self, area: Rect, buf: &mut Buffer) {
    match self.direction {
      Direction::Vertical =>
        self.render_vertical_scroller(area, buf),
      Direction::Horizontal =>
        self.render_horizontal_scroller(area, buf),
    };
  }
}
