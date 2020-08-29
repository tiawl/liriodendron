extern crate std;
use std::convert::TryFrom;

extern crate tui;
use tui::{Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use crate::log;

use crate::userinterface::views::{BORDERS, MIN_SHORTCUTS_AREA_WIDTH, ratio};
use crate::userinterface::widgets::{shortcuts, gauge::{self, Rgb}};

const NB_GENERAL_SHORTCUTS: usize = 3;
const NB_GRIDS_SHORTCUTS: usize = 0;
const NB_BRUSH_SHORTCUTS: usize = 2;
const NB_TEXTURE_SHORTCUTS: usize = 2;
const NB_COLOR_SHORTCUTS: usize = 0;
const NB_PIXEL_SHORTCUTS: usize = 0;

const SHORTCUTS_AREAS: usize = 6;

pub const LARGER_COLOR_ROW: u16 = 0;
pub const LARGER_PIXEL_ROW: u16 = 0;
pub const LARGER_TEXTURE_ROW: u16 = 16;
pub const LARGER_GRID_PARAM_ROW: u16 = 0;
pub const LARGER_BRUSH_PARAM_ROW: u16 = 43;


const BRUSH_SHORTCUTS_HEIGHT: usize = NB_BRUSH_SHORTCUTS + BORDERS as usize;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, log: &mut log::Log,
  (nb_todo_tasks, nb_max_tasks): (usize, usize)) {
    terminal.draw(|frame| {
      let frame_area = frame.size();

      let left = frame_area.left();
      let top = frame_area.top();
      let width = frame_area.right() - left;
      let height = frame_area.bottom() - top;

      let parameters_width = width - MIN_SHORTCUTS_AREA_WIDTH;
      let shortcuts_height = height - gauge::GAUGE_HEIGHT -
        u16::try_from(BRUSH_SHORTCUTS_HEIGHT).unwrap();

      let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Percentage(50),
          Constraint::Percentage(50),
        ].as_ref())
        .split(frame.size());

      let shortcuts_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Percentage(20),
          Constraint::Percentage(20),
          Constraint::Percentage(20),
          Constraint::Percentage(20),
          Constraint::Percentage(20),
        ].as_ref())
        .split(Rect::new(left, top, width, shortcuts_height));

      let brush_area = Rect::new(left, top, parameters_width/2, 2 + BORDERS);
      let brush_text_area = Rect::new(brush_area.left() + 1,
        brush_area.top() + 1,
        brush_area.right() - brush_area.left() - BORDERS,
        brush_area.bottom() - brush_area.top() - BORDERS);
      let grids_area = Rect::new(left, brush_area.bottom(),
        parameters_width/2, height).intersection(vertical_chunks[0]);
      let grids_text_area = Rect::new(grids_area.left() + 1,
        grids_area.top() + 1,
        grids_area.right() - grids_area.left() - BORDERS,
        grids_area.bottom() - grids_area.top() - BORDERS);
      let color_area = Rect::new(left, brush_area.bottom(),
        parameters_width/2, height).intersection(vertical_chunks[1]);
      let texture_area = Rect::new(grids_area.right(), top,
        parameters_width/2, height).intersection(vertical_chunks[0]);
      let texture_text_area = Rect::new(texture_area.left() + 1,
        texture_area.top() + 1,
        texture_area.right() - texture_area.left() - BORDERS,
        texture_area.bottom() - texture_area.top() - BORDERS);
      let pixel_area = Rect::new(grids_area.right(), texture_area.bottom(),
        parameters_width/2, height).intersection(vertical_chunks[1]);
      let mut shortcuts_areas = Vec::<Rect>::with_capacity(SHORTCUTS_AREAS);
      for i in 0..(SHORTCUTS_AREAS - 1) {
        shortcuts_areas.push(Rect::new(texture_area.right(), top,
          MIN_SHORTCUTS_AREA_WIDTH, shortcuts_height)
          .intersection(shortcuts_chunks[i]));
      }
      shortcuts_areas.push(Rect::new(texture_area.right(),
        shortcuts_areas[4].bottom(), MIN_SHORTCUTS_AREA_WIDTH,
        u16::try_from(BRUSH_SHORTCUTS_HEIGHT).unwrap()));
      let gauge_tasks_area = Rect::new(texture_area.right(),
        shortcuts_areas[5].bottom(), MIN_SHORTCUTS_AREA_WIDTH,
        gauge::GAUGE_HEIGHT);

      render_grids(frame, log, grids_area, grids_text_area);
      render_brush(frame, log, brush_area, brush_text_area);
      render_color(frame, color_area);
      render_texture(frame, texture_area, texture_text_area, log);
      render_pixel(frame, pixel_area);

      render_shortcuts(frame, shortcuts_areas);
      render_tasks_gauge(
        frame, gauge_tasks_area, (nb_todo_tasks, nb_max_tasks));
  }).unwrap();
}

fn render_grids<B: Backend>(frame: &mut Frame<B>, log: &log::Log,
  grids_area: Rect, text_area: Rect) {
    let grids_block = Block::default().title(" Grids Parameters ")
      .borders(Borders::ALL);
    let grids_text = Paragraph::new(vec![
      Spans::from(vec![Span::raw(format!("Width: {}",
        log.grids_getwidth::<u16>()))]),
      Spans::from(vec![Span::raw(format!("Height: {}",
        log.grids_getheight::<u16>()))]),
    ]);
    frame.render_widget(grids_text, text_area);
    frame.render_widget(grids_block, grids_area);
}

fn render_brush<B: Backend>(frame: &mut Frame<B>, log: &log::Log,
  brush_area: Rect, text_area: Rect) {
    let brush_block = Block::default().title(" Brush Parameters ")
      .borders(Borders::ALL);
    let body_de_ratio =
      (log.brush_getbodyderatio() * 100.).round() as u8;
    let body_dr_ratio =
      (log.brush_getbodydrratio() * 100.).round() as u8;
    let brush_text = Paragraph::new(vec![
      Spans::from(vec![
        Span::styled(" D|E Pixel Ratio ",
          Style::default().bg(Color::Green)),
        Span::raw(format!(" Body = {}% | Empty = {}%", 100 - body_de_ratio,
          body_de_ratio)),
      ]),
      Spans::from(vec![
        Span::styled(" D|R Pixel Ratio ",
          Style::default().bg(Color::Blue)),
        Span::raw(format!(" Body = {}% | Border = {}%", 100 - body_dr_ratio,
          body_dr_ratio)),
      ]),
    ]);
    frame.render_widget(brush_text, text_area);
    frame.render_widget(brush_block, brush_area);
}

fn render_color<B:Backend>(frame: &mut Frame<B>, area: Rect) {
  let color_block = Block::default().title(" Color Parameters ")
    .borders(Borders::ALL);
  frame.render_widget(color_block, area);
}

fn render_texture<B:Backend>(frame: &mut Frame<B>, area: Rect,
  text_area: Rect, log: &mut log::Log) {
    let texture_block = Block::default().title(" Textures Parameters ")
      .borders(Borders::ALL);
      let texture_text = Paragraph::new(vec![
        Spans::from(vec![Span::raw(format!("Format = {}",
          format!("{:?}", log.texturessettings_getformat()).to_uppercase())),
        ]),
        Spans::from(vec![Span::raw(format!("Pixel Ratio = {}",
          log.texturessettings_getpixelratio::<u16>())),
        ]),
      ]);
      frame.render_widget(texture_text, text_area);
    frame.render_widget(texture_block, area);
}

fn render_pixel<B:Backend>(frame: &mut Frame<B>, area: Rect) {
  let pixel_block = Block::default().title(" Pixel Parameters ")
    .borders(Borders::ALL);
  frame.render_widget(pixel_block, area);
}

fn render_shortcuts<B:Backend>(frame: &mut Frame<B>, areas: Vec<Rect>) {
  let mut general_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_GENERAL_SHORTCUTS, String::from(" General Shortcuts "));
  general_shortcuts.push_action("Generate Texture");
  general_shortcuts.push_action("Switch View");
  general_shortcuts.push_action("Exit");
  general_shortcuts.push_instruction("< g >");
  general_shortcuts.push_instruction("< s >");
  general_shortcuts.push_instruction("< Esc >");
  frame.render_widget(general_shortcuts, areas[0]);

  let /* mut */ grids_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_GRIDS_SHORTCUTS, String::from(" Grids Shortcuts "));
  frame.render_widget(grids_shortcuts, areas[1]);

  let mut texture_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_TEXTURE_SHORTCUTS, String::from(" Texture Shortcuts "));
  texture_shortcuts.push_action("Set Format");
  texture_shortcuts.push_action("(+|-) Pixel Ratio");
  texture_shortcuts.push_instruction("< f >");
  texture_shortcuts.push_instruction("< p | P >");
  frame.render_widget(texture_shortcuts, areas[2]);

  let /* mut */ color_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_COLOR_SHORTCUTS, String::from(" Color Shortcuts "));
  frame.render_widget(color_shortcuts, areas[3]);

  let /* mut */ pixel_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_PIXEL_SHORTCUTS, String::from(" Pixel Shortcuts "));
  frame.render_widget(pixel_shortcuts, areas[4]);

  let mut brush_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_BRUSH_SHORTCUTS, String::from(" Brush Shortcuts "));
  brush_shortcuts.push_action("(+|-) BodyEmpty Pixel Ratio");
  brush_shortcuts.push_action("(+|-) BodyBorder Pixel Ratio");
  brush_shortcuts.push_instruction("< b | B >");
  brush_shortcuts.push_instruction("< n | N >");
  frame.render_widget(brush_shortcuts, areas[5]);
}

fn render_tasks_gauge<B: Backend>(frame: &mut Frame<B>, area: Rect,
  (nb_todo_tasks, nb_max_tasks): (usize, usize)) {
    let gauge_tasks_ratio = ratio(nb_todo_tasks, nb_max_tasks);
    let gauge_tasks_label =
      format!("Tasks: {}/{}", nb_todo_tasks, nb_max_tasks);
    let gauge_tasks;
    if gauge_tasks_ratio < 0.5 {
      gauge_tasks = gauge::GaugeWidget::new(gauge_tasks_ratio,
        &gauge_tasks_label,
          (Rgb::GREEN, Rgb::YELLOW, gauge_tasks_ratio * 2.));
    } else {
      gauge_tasks = gauge::GaugeWidget::new(gauge_tasks_ratio,
        &gauge_tasks_label,
          (Rgb::YELLOW, Rgb::RED, (gauge_tasks_ratio - 0.5) * 2.));
    }
    frame.render_widget(gauge_tasks, area);
}
