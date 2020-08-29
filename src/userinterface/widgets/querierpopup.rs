extern crate std;
use std::convert::TryFrom;

extern crate tui;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::text::{Spans, Text};
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, cursor};

/// Widget to render a popup querying user's input
pub struct QuerierPopupWidget<'a> {
  keyboard_instructions: Text<'a>,
  value: Vec<Spans<'a>>,
  cursor: (u16, bool),
}

impl<'a> QuerierPopupWidget<'a> {
  pub fn new(keyboard_instructions: Text<'a>, value: Vec<Spans<'a>>,
    cursor: (u16, bool)) -> QuerierPopupWidget<'a> {
      QuerierPopupWidget {
        keyboard_instructions,
        value,
        cursor,
      }
  }
}

impl<'a> Widget for QuerierPopupWidget<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let (left, top, width, height) = (area.left(), area.top(),
      area.right() - area.left(), area.bottom() - area.top());

    let kb_height =
      u16::try_from(self.keyboard_instructions.height()).unwrap();
    let value_width = u16::try_from(
      self.value.iter().map(|key| key.width()).max().unwrap()).unwrap();
    let value_height = u16::try_from(self.value.len()).unwrap();

    /* keyboard instructions */
    let keyboard_instructions_area =
      Rect::new(left + 1, top + 2, width - BORDERS, height - BORDERS);
    Paragraph::new(self.keyboard_instructions).alignment(Alignment::Center)
      .render(keyboard_instructions_area, buf);

    /* value */
    for (row, span) in self.value.iter().enumerate() {
      let value_area = Rect::new(
        left + width/2 - u16::try_from(span.width()).unwrap()/2,
        top + 4 + kb_height + u16::try_from(row).unwrap(), width, 1);
      Paragraph::new(span.clone()).alignment(Alignment::Left)
        .render(value_area, buf);
    }

    /* value box */
    let box_width = value_width + BORDERS + EMPTY_SPACES;
    let value_area = Rect::new(left + width/2 - value_width/2 - 2,
      top + 4 + kb_height - 1, box_width, BORDERS + value_height);
    let value_block = Block::default().borders(Borders::ALL);
    value_block.render(value_area, buf);

    /* cursor */
    let left_cursor = left + width/2 -
      u16::try_from(self.value[0].width()).unwrap()/2;
    let top_cursor = top + 4 + kb_height;
    let cursor_area = Rect::new(left_cursor, top_cursor, 1, 1);
    cursor::CursorWidget::new(self.cursor).render(cursor_area, buf);

    /* window box */
    let window_block = Block::default().borders(Borders::ALL);
    window_block.render(area, buf);
  }
}
