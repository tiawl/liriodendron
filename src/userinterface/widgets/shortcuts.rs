extern crate std;
use std::convert::TryFrom;
use std::cmp::min;

extern crate tui;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Direction, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::userinterface::widgets::BORDERS;
use crate::userinterface::widgets::scroller::{self, ARROWS, SCROLLER};

pub const VERTICAL_BAR: u16 = 1;
pub const ACTIONS_AREA_WIDTH: u16 = 29;
pub const KEYBOARD_AREA_WIDTH: u16 = 27;

/// Widget to render shortcuts' instructions area
pub struct ShortcutsWidget<'a> {
  actions: Vec<Spans<'a>>,
  keyboard_instructions: Vec<Spans<'a>>,
  title: String,
}

impl<'a> ShortcutsWidget<'a> {

  pub fn new(nb_rows: usize, title: String) -> ShortcutsWidget<'a> {
    ShortcutsWidget {
      actions: Vec::<Spans<'a>>::with_capacity(nb_rows),
      keyboard_instructions: Vec::<Spans<'a>>::with_capacity(nb_rows),
      title: title,
    }
  }

  pub fn push_action(&mut self, line: &'a str) {
    if self.actions.len() % 2 == 0 {
      self.actions.push(Spans::<'a>::from(vec![Span::raw(line)]));
    } else {
      let count = ACTIONS_AREA_WIDTH -
        u16::try_from(line.chars().collect::<Vec<char>>().len()).unwrap();
      let mut span = String::from(line);
      for i in 0..=count {
        if i < count / 2 {
          span.insert(0, ' ');
        } else {
          span.push(' ');
        }
      }
      let style = Style::default().bg(Color::Black);
      self.actions.push(Spans::<'a>::from(vec![Span::styled(span, style)]));
    }
  }

  pub fn push_instruction(&mut self, line: &'a str) {
    if self.keyboard_instructions.len() % 2 == 0 {
      self.keyboard_instructions.push(Spans::<'a>::from(
        vec![Span::raw(line)]));
    } else {
      let count = KEYBOARD_AREA_WIDTH -
        u16::try_from(line.chars().collect::<Vec<char>>().len()).unwrap();
      let mut span = String::from(line);
      for i in 0..=count {
        if i < count / 2 {
          span.insert(0, ' ');
        } else {
          span.push(' ');
        }
      }
      let style = Style::default().bg(Color::Black);
      self.keyboard_instructions.push(
        Spans::<'a>::from(vec![Span::styled(span, style)]));
    }
  }
}

impl<'a> Widget for ShortcutsWidget<'a> {

  fn render(self, area: Rect, buf: &mut Buffer) {

    let left = area.left();
    let top = area.top();
    let height = area.bottom() - top;

    // Render action block
    let actions_block = Block::default().title(self.title)
      .borders(Borders::ALL);
    let actions_area = Rect::new(left, top,
      ACTIONS_AREA_WIDTH + BORDERS + SCROLLER, height);
    actions_block.render(actions_area, buf);

    let shortcuts_height = u16::try_from(self.actions.len()).unwrap();
    let actions_paragraph = Paragraph::new(self.actions)
      .alignment(Alignment::Center);
    let actions_paragraph_area = Rect::new(left + 1 + SCROLLER, top + 1,
      ACTIONS_AREA_WIDTH, height - BORDERS);
    actions_paragraph.render(actions_paragraph_area, buf);

    // Render scroller
    if (BORDERS + ARROWS <= height) && (ARROWS <= shortcuts_height) {
      let scroll_y = 0; // TODO: scroll for shortcuts
      let inner_height =
        min(height - BORDERS - ARROWS, shortcuts_height - ARROWS);

      let vertical_scroller = scroller::ScrollerWidget::new(
          Direction::Vertical, scroll_y, shortcuts_height - ARROWS);
      let vscroller_area =
        Rect::new(left + 1, top + 1, SCROLLER, inner_height);
      vertical_scroller.render(vscroller_area, buf);
    }

    // Render keyboard instructions block
    let keyboard_block = Block::default().borders(Borders::ALL);
    let keyboard_area = Rect::new(left + ACTIONS_AREA_WIDTH + SCROLLER + 1,
      top, KEYBOARD_AREA_WIDTH + BORDERS, height);
    keyboard_block.render(keyboard_area, buf);

    let keyboard_paragraph = Paragraph::new(self.keyboard_instructions)
      .alignment(Alignment::Center);
    let keyboard_paragraph_area =
      Rect::new(left + ACTIONS_AREA_WIDTH + 1 + SCROLLER + VERTICAL_BAR,
        top + 1, KEYBOARD_AREA_WIDTH, height - BORDERS);
    keyboard_paragraph.render(keyboard_paragraph_area, buf);

    buf.get_mut(left + ACTIONS_AREA_WIDTH + 1 + SCROLLER, top)
      .set_symbol("┬");
    buf.get_mut(left + ACTIONS_AREA_WIDTH + 1 + SCROLLER, top + height - 1)
      .set_symbol("┴");
  }
}
