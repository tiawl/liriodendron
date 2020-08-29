extern crate std;
use std::convert::TryFrom;

extern crate tui;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::userinterface::widgets::BORDERS;

/// Widget to render a popup querying user's input
pub struct SelectorPopupWidget<'a> {
  keyboard_instructions: Text<'a>,
  value: Vec<String>,
  cursor: usize,
}

impl<'a> SelectorPopupWidget<'a> {
  pub fn new(keyboard_instructions: Text<'a>, value: Vec<String>,
    cursor: usize) -> SelectorPopupWidget<'a> {
      SelectorPopupWidget {
        keyboard_instructions,
        value,
        cursor,
      }
  }
}

impl<'a> Widget for SelectorPopupWidget<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let (left, top, width, height) = (area.left(), area.top(),
      area.right() - area.left(), area.bottom() - area.top());

    let kb_height =
      u16::try_from(self.keyboard_instructions.height()).unwrap();

    /* selector */
    let value_area = Rect::new(left, top + 4 + kb_height, width,
      u16::try_from(self.value.len()).unwrap());
    let selector = self.value.iter().enumerate().map(|(index, key)| {
      if index == self.cursor {
        Spans::from(vec![Span::styled(format!(" ▸ {} ◂ ", key),
          Style::default().add_modifier(Modifier::REVERSED))])
      } else {
        Spans::from(vec![Span::raw(key)])
      }
    }).collect::<Vec<Spans>>();
    Paragraph::new(selector).alignment(Alignment::Center)
      .render(value_area, buf);

    /* keyboard instructions */
    let keyboard_instructions_area =
      Rect::new(left + 1, top + 2, width - BORDERS, height - BORDERS);
    Paragraph::new(self.keyboard_instructions).alignment(Alignment::Center)
      .render(keyboard_instructions_area, buf);

    /* window box */
    let window_block = Block::default().borders(Borders::ALL);
    window_block.render(area, buf);
  }
}
