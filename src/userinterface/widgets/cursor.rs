extern crate tui;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::Widget;

/// Widget to render a blinking cursor
pub struct CursorWidget {
  pos: u16,
  is_blinking: bool,
}

impl CursorWidget {
  pub fn new((pos, is_blinking): (u16, bool)) -> CursorWidget {
    CursorWidget {
      pos,
      is_blinking,
    }
  }
}

impl Widget for CursorWidget {

  fn render(self, area: Rect, buf: &mut Buffer) {
    let (left, top) = (area.left() + self.pos, area.top());
    if !self.is_blinking {
      buf.get_mut(left, top).set_style(Style::default());
    } else {
      buf.get_mut(left, top).set_style(Style::default()
        .add_modifier(Modifier::REVERSED));
    }
  }
}
