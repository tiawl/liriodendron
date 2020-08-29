extern crate std;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::thread::ThreadId;

extern crate tui;
use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::widgets::{Block, Borders, Paragraph};

use crate::userinterface::views::ratio;
use crate::userinterface::widgets::{BORDERS, EMPTY_SPACES, gauge::{self, Rgb}};

const EXTRA_SPACES: u16 = 3;
const LARGER_ROW: u16 = 29;

pub const WIN_WIDTH: u16 = LARGER_ROW + (EMPTY_SPACES + EXTRA_SPACES) * 2
  + BORDERS;

const TEXT_HEIGHT: u16 = 2;
const GAUGE_NUMBERS: u16 = 4;
pub const MAX_WIN_HEIGHT: u16 = EMPTY_SPACES + BORDERS + TEXT_HEIGHT +
  gauge::GAUGE_HEIGHT * (GAUGE_NUMBERS + 1);

pub fn render<B: Backend>(terminal: &mut Terminal<B>,
  threads: &HashMap<ThreadId, (u16, u16)>, tasks: (usize, usize)) {
    terminal.draw(|frame| {
      let frame_area = frame.size();

      let frame_left = frame_area.left();
      let frame_top = frame_area.top();
      let frame_width = frame_area.right() - frame_left;
      let frame_height = frame_area.bottom() - frame_top;

      let window_height = EMPTY_SPACES + BORDERS + TEXT_HEIGHT +
        gauge::GAUGE_HEIGHT * u16::try_from(threads.len() + 1).unwrap();
      let window_area = Rect::new(frame_left + frame_width/2 - WIN_WIDTH/2,
        frame_top + frame_height/2 - window_height/2, WIN_WIDTH, window_height);
      let window_left = window_area.left();
      let window_top = window_area.top();
      let window_width = window_area.right() - window_left;

      let inner_area = Rect::new(window_left + 1, window_top + 2,
        window_width - BORDERS, window_height - BORDERS);

      render_borders(frame, &window_area);
      render_text(frame, &inner_area);
      render_gauges(frame, &inner_area, threads, tasks);

    }).unwrap();
}

fn render_borders<B: Backend>(frame: &mut Frame<B>, area: &Rect) {
  let block = Block::default().borders(Borders::ALL);
  frame.render_widget(block, *area);
}

fn render_text<B: Backend>(frame: &mut Frame<B>, area: &Rect) {
  let paragraph = Paragraph::new("Liriodendron have to complete
tasks before to exit").alignment(Alignment::Center);

  frame.render_widget(paragraph, *area);
}

fn render_gauges<B: Backend>(frame: &mut Frame<B>, area: &Rect,
  threads: &HashMap<ThreadId, (u16, u16)>,
  (tasks_done, tasks_todo): (usize, usize)) {

    let mut gauge_label = format!("Tasks: {}/{}", tasks_done, tasks_todo);
    let mut gauge_ratio = ratio(tasks_done, tasks_todo);
    let mut area = Rect::new(area.left(), area.top() + gauge::GAUGE_HEIGHT,
      area.right() - area.left(), gauge::GAUGE_HEIGHT);
    if gauge_ratio < 0.5 {
      frame.render_widget(gauge::GaugeWidget::new( gauge_ratio, &gauge_label,
        (Rgb::RED, Rgb::YELLOW, gauge_ratio * 2.)), area);
    } else {
      frame.render_widget(gauge::GaugeWidget::new(gauge_ratio, &gauge_label,
        (Rgb::YELLOW, Rgb::GREEN, (gauge_ratio - 0.5) * 2.)), area);
    }

    let mut id = 1;
    for (_, &(gen_done, gen_todo)) in threads.iter() {
      area = Rect::new(area.left(), area.top() + gauge::GAUGE_HEIGHT,
        area.right() - area.left(), gauge::GAUGE_HEIGHT);
      gauge_label = format!("Process {}: {}/{}", id, gen_done, gen_todo);
      gauge_ratio = ratio(gen_done, gen_todo);
      if gauge_ratio < 0.5 {
        frame.render_widget(gauge::GaugeWidget::new(gauge_ratio, &gauge_label,
          (Rgb::RED, Rgb::YELLOW, gauge_ratio * 2.)), area);
      } else {
        frame.render_widget(gauge::GaugeWidget::new(gauge_ratio, &gauge_label,
          (Rgb::YELLOW, Rgb::GREEN, (gauge_ratio - 0.5) * 2.)), area);
      }
      id += 1;
    }
}
