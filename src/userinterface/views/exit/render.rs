extern crate tui;
use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};

use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES};

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 13;

pub const WIN_WIDTH: u16 = LARGER_ROW + (EMPTY_SPACES + EXTRA_SPACES) * 2
  + BORDERS;

const TEXT_HEIGHT: u16 = 4;

pub const WIN_HEIGHT: u16 = TEXT_HEIGHT + EMPTY_SPACES + BORDERS;

pub fn render<B: Backend>(terminal: &mut Terminal<B>) {
  terminal.draw(|frame| {
    let frame_area = frame.size();

    let frame_left = frame_area.left();
    let frame_top = frame_area.top();
    let frame_width = frame_area.right() - frame_left;
    let frame_height = frame_area.bottom() - frame_top;

    let window_area = Rect::new(frame_left + frame_width/2 - WIN_WIDTH/2,
      frame_top + frame_height/2 - WIN_HEIGHT/2, WIN_WIDTH, WIN_HEIGHT);
    let window_left = window_area.left();
    let window_top = window_area.top();
    let window_width = window_area.right() - window_left;
    let window_height = window_area.bottom() - window_top;

    let text_area =
      Rect::new(window_left + 1, window_top + window_height/2 - BORDERS,
        window_width - BORDERS, window_height - BORDERS);

    render_borders(frame, &window_area);
    render_text(frame, &text_area);

  }).unwrap();
}

fn render_borders<B: Backend>(frame: &mut Frame<B>, area: &Rect) {
  let block = Block::default().borders(Borders::ALL);
  frame.render_widget(block, *area);
}

fn render_text<B: Backend>(frame: &mut Frame<B>, area: &Rect) {

    let exit_text = Paragraph::new(Text::from("Exit ?

< y > for YES
< n > for NO")).alignment(Alignment::Center);

    frame.render_widget(exit_text, *area);
}
