extern crate tui;
use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::Paragraph;

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES};

const TEXT_HEIGHT: u16 = 6;

const MIN_WIDTH: u16 = 50;
const MIN_HEIGHT: u16 = TEXT_HEIGHT + EMPTY_SPACES;

pub fn render<B: Backend>(terminal: &mut Terminal<B>) {
  terminal.draw(|frame| {
    let frame_area = frame.size();

    let frame_left = frame_area.left();
    let frame_top = frame_area.top();
    let frame_width = frame_area.right() - frame_left;
    let frame_height = frame_area.bottom() - frame_top;

    if (frame_width >= MIN_WIDTH) && (frame_height >= MIN_HEIGHT) {
      let text_area =
        Rect::new(frame_left + 1, frame_top + frame_height/2 - TEXT_HEIGHT/2,
          frame_width - BORDERS, TEXT_HEIGHT);

      render_text(frame, &text_area);
    }

  }).unwrap();
}

fn render_text<B: Backend>(frame: &mut Frame<B>, area: &Rect) {

    let exit_text = Paragraph::new(Text::from("The interface can't be displayed
correctly if your terminal size is too small.
If you continue to reduce the terminal
size, this message will be erased but the
application will still run until your
terminal will be bigger."))
      .alignment(Alignment::Center)
      .style(Style::default().fg(Color::Red));

    frame.render_widget(exit_text, *area);
}
