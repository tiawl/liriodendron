extern crate tui;
use tui::{Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Layout, Constraint, Direction, Rect};

use crate::log;

use crate::userinterface::widgets::{brushselector, grid,
  shortcuts, gauge::{self, Rgb}};
use crate::userinterface::views::{MIN_BRUSH_AREA_WIDTH,
  MIN_SHORTCUTS_AREA_WIDTH, ratio};

const NB_BRUSH_SHORTCUTS: usize = 3;
const NB_GRID_SHORTCUTS: usize = 4;
const NB_WORKSPACE_SHORTCUTS: usize = 3;
const NB_GENERAL_SHORTCUTS: usize = 4;

const SHORTCUTS_AREAS: usize = 4;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, log: &mut log::Log,
  (nb_todo_tasks, nb_max_tasks): (usize, usize)) {

    terminal.draw(|frame| {
      let frame_area = frame.size();

      let left = frame_area.left();
      let top = frame_area.top();
      let width = frame_area.right() - left;
      let height = frame_area.bottom() - top;

      let shortcuts_height = height - 2 * gauge::GAUGE_HEIGHT;

      let shortcuts_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Percentage(25),
          Constraint::Percentage(25),
          Constraint::Percentage(25),
          Constraint::Percentage(25),
        ].as_ref())
        .split(Rect::new(left, top, width, shortcuts_height));

      let brush_area = Rect::new(0, 0, MIN_BRUSH_AREA_WIDTH, height);
      let workspace_area = Rect::new(brush_area.right(), 0,
        width - MIN_BRUSH_AREA_WIDTH - MIN_SHORTCUTS_AREA_WIDTH, height);
      let mut shortcuts_areas = Vec::<Rect>::with_capacity(SHORTCUTS_AREAS);
      for i in 0..SHORTCUTS_AREAS {
        shortcuts_areas.push(Rect::new(workspace_area.right(), top,
          MIN_SHORTCUTS_AREA_WIDTH, shortcuts_height)
          .intersection(shortcuts_chunks[i]));
      }
      let gauge_tasks_area = Rect::new(workspace_area.right(),
        shortcuts_areas.last().unwrap().bottom(), MIN_SHORTCUTS_AREA_WIDTH,
        gauge::GAUGE_HEIGHT);
      let gauge_map_area = Rect::new(workspace_area.right(),
        gauge_tasks_area.bottom(), MIN_SHORTCUTS_AREA_WIDTH,
        gauge::GAUGE_HEIGHT);

      frame.render_widget(
        brushselector::BrushSelectorWidget::new(log), brush_area);
      frame.render_widget(grid::GridWidget::new(log), workspace_area);

      render_shortcuts(frame, shortcuts_areas);

      render_tasks_gauge(
        frame, gauge_tasks_area, (nb_todo_tasks, nb_max_tasks));

      render_map_gauge(frame, log, gauge_map_area);

    }).unwrap();
}

fn render_shortcuts<B: Backend>(frame: &mut Frame<B>, areas: Vec<Rect>) {

  let mut brush_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_BRUSH_SHORTCUTS, String::from(" Brush Shortcuts "));
  brush_shortcuts.push_action("Brush | Eraser");
  brush_shortcuts.push_action("Next Brush | Previous Brush");
  brush_shortcuts.push_action("Set Brush Color");
  brush_shortcuts.push_instruction("< L-Click | R-Click >");
  brush_shortcuts.push_instruction("< q | a >");
  brush_shortcuts.push_instruction("< c >");
  frame.render_widget(brush_shortcuts, areas[0]);

  let mut grid_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_GRID_SHORTCUTS, String::from(" Grid Shortcuts "));
  grid_shortcuts.push_action("(+|-) Width | (+|-) Height");
  grid_shortcuts.push_action("Scroll Grid");
  grid_shortcuts.push_action("Clear Grid");
  grid_shortcuts.push_action("Rename Grid");
  grid_shortcuts.push_instruction("< w | W > | < h | H >");
  grid_shortcuts.push_instruction("< ←  | ↑  | ↓  | →  >");
  grid_shortcuts.push_instruction("< C >");
  grid_shortcuts.push_instruction("< r >");
  frame.render_widget(grid_shortcuts, areas[1]);

  let mut workspace_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_WORKSPACE_SHORTCUTS, String::from(" Workspace Shortcuts "));
  workspace_shortcuts.push_action("Add | Delete Grid");
  workspace_shortcuts.push_action("Next Grid | Previous Grid");
  workspace_shortcuts.push_action("Switch Grid Order");
  workspace_shortcuts.push_instruction("< + | - >");
  workspace_shortcuts.push_instruction("< n | N >");
  workspace_shortcuts.push_instruction("< S >");
  frame.render_widget(workspace_shortcuts, areas[2]);

  let mut general_shortcuts = shortcuts::ShortcutsWidget::new(
    NB_GENERAL_SHORTCUTS, String::from(" General Shortcuts "));
  general_shortcuts.push_action("Undo | Redo");
  general_shortcuts.push_action("Generate Texture");
  general_shortcuts.push_action("Switch View");
  general_shortcuts.push_action("Exit");
  general_shortcuts.push_instruction("< u | U >");
  general_shortcuts.push_instruction("< g >");
  general_shortcuts.push_instruction("< s >");
  general_shortcuts.push_instruction("< Esc >");
  frame.render_widget(general_shortcuts, areas[3]);
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

fn render_map_gauge<B: Backend>(frame: &mut Frame<B>, log: &log::Log,
  area: Rect) {
    let (map_length, map_capacity) = log.grids_getlengthcapacity();
    let gauge_map_ratio = ratio(map_length, map_capacity);
    let gauge_map_label =
      format!("Memory: {}/{}", map_length, map_capacity);
    let gauge_map;
    if gauge_map_ratio < 0.5 {
      gauge_map = gauge::GaugeWidget::new(gauge_map_ratio,
        &gauge_map_label, (Rgb::GREEN, Rgb::YELLOW, gauge_map_ratio * 2.));
    } else {
      gauge_map = gauge::GaugeWidget::new(gauge_map_ratio, &gauge_map_label,
        (Rgb::YELLOW, Rgb::RED, (gauge_map_ratio - 0.5) * 2.));
    }
    frame.render_widget(gauge_map, area);
}
