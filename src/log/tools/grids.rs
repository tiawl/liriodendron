extern crate std;
use std::cmp::min;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use crate::utils::FullPixel;
use crate::log::{action,
  tools::{Pixel, Grid, GridTool, MAX_SIZE, Tool, WorkspaceTool}};

/// Module for grids' cells
mod cell;

pub mod gridname;

const DEFAULT_GRID_SIZE: u16 = 10;
const SIZE_LIMIT: u16 = 512;
const MAX_GRIDS: usize = 10;

const OVERFLOW_OCCURED: bool = true;

/// Represents <i>user interface</i> set of grids
#[derive(Clone)]
pub struct Grids {
  current_grid_id: usize,
  current_grid: Grid,

  /// Names of grids
  names: Vec<String>,

  /// If an error occured during an operation, a <i>String</i> is made to be
  /// displayed on the graphic interface
  error: String,

  height: u16,
  width: u16,

  /// Determines the visible part of the grid when <i>user interface</i> grid
  /// area is smaller than the grid
  scroll_x: u16,

  /// Determines the visible part of the grid when <i>user interface</i> grid
  /// area is smaller than the grid
  scroll_y: u16,

  /// Grids' filled cells
  state: HashMap<cell::Cell, FullPixel>,

  /// Customizable member
  map_capacity: usize,

  /// Determines which grid will be swapped with the current grid
  switch_cursor: usize,
}

pub fn size_limit() -> u16 {
  return SIZE_LIMIT
}

impl Grids {

  pub fn new(map_capacity: usize) -> Grids {
    let mut grids = Grids {
      current_grid_id: 0,
      current_grid: Grid::Generation,
      names: Vec::with_capacity(MAX_GRIDS),
      error: String::new(),
      height: DEFAULT_GRID_SIZE,
      width: DEFAULT_GRID_SIZE,
      scroll_x: 0,
      scroll_y: 0,
      state: HashMap::with_capacity(map_capacity),
      map_capacity: map_capacity,
      switch_cursor: 0,
    };
    let grid_name =
      grids.check_name(String::from("Grid1")).unwrap().iter().collect();
    grids.add(grid_name);
    grids
  }

  pub fn get_current_grid_info(&self) -> (Grid, usize) {
    (self.current_grid, self.current_grid_id)
  }

  pub fn is_on_current_grid(&self, id: usize, grid: Grid) -> bool {
    (id == self.current_grid_id) && (grid == self.current_grid)
  }

  pub fn get_length_capacity_state(&self) -> (usize, usize) {
    (self.state.len(), self.map_capacity)
  }

  /// Returns current grid's filled cells in a Vec of tuples <i>(cell content,
  /// cell X coordinate, cell Y coordinate)</i>
  pub fn get_current_grid(&self) -> Vec<(FullPixel, u16, u16)> {
    self.state.iter()
      .filter(|&(c, _)| self.is_on_current_grid(c.get_grid_id(), c.get_grid()))
      .map(|(&c, &pixel)| (pixel, c.get_x(), c.get_y()))
      .collect()
  }

  pub fn get_names(&self) -> Vec<String> {
    self.names.clone()
  }

  pub fn get_nb(&self) -> usize {
    self.names.len()
  }

  pub fn is_full(&self) -> bool {
    self.names.len() >= MAX_GRIDS
  }

  pub fn is_alone(&self) -> bool {
    self.names.len() == 1
  }

  fn clear_current_grid(&mut self) {
    let current_grid = self.current_grid;
    let current_grid_id = self.current_grid_id;
    self.state.retain(|key, _|
      !((key.get_grid_id() == current_grid_id) &&
      (key.get_grid() == current_grid)));
  }

  fn add(&mut self, name: String) {
    self.names.push(name);
    self.current_grid_id = self.names.len() - 1;
  }

  fn rename_current_grid(&mut self, name: String) {
    self.names[self.current_grid_id].clone_from(&name);
  }

  /// Clears content of the current grid, deletes it and decrements the grid
  /// id of cells with a grid id greater than the deleted grid
  fn delete_current_grid(&mut self) {
    self.clear_current_grid();
    self.names.remove(self.current_grid_id);
    self.state = self.state.iter().map(|(&cell, &pixel)| {
      if cell.get_grid_id() > self.current_grid_id {
        (cell.set_grid_id(cell.get_grid_id() - 1), pixel)
      } else {
        (cell, pixel)
      }
    }).collect();
    if self.current_grid_id >= self.names.len() {
      self.current_grid_id -= 1;
    }
  }

  /// <i>switched_grid</i> is a data from the userinterface where names of
  /// each grid is displayed except the current grid. So if the
  /// <i>switched_grid</i> has a greater id than the current grid, this
  /// <i>switched_grid</i> has a <i>userinterface</i> id equal to its
  /// <i>log</i> id minus 1. So when this data arrived in this function, we
  /// have to increments it to match with this <i>userinterface</i>
  /// representation to the <i>log</i> representation.
  fn switch_current_grid(&mut self, mut switched_grid: usize) {
    if switched_grid > self.current_grid_id {
      switched_grid += 1;
    }
    self.state = self.state.iter().map(|(&cell, &pixel)| {
      if cell.get_grid_id() == self.current_grid_id {
        (cell.set_grid_id(switched_grid), pixel)
      } else if cell.get_grid_id() == switched_grid {
        (cell.set_grid_id(self.current_grid_id), pixel)
      } else {
        (cell, pixel)
      }
    }).collect();
    self.names.swap(self.current_grid_id, switched_grid);
  }

  pub fn get_switch_cursor(&self) -> usize {
    self.switch_cursor
  }

  pub fn next_switch_cursor(&mut self) {
    self.switch_cursor += 1;
    if self.switch_cursor >= self.names.len() - 1 {
      self.switch_cursor = 0;
    }
  }

  pub fn previous_switch_cursor(&mut self) {
    if self.switch_cursor == 0 {
      self.switch_cursor = self.names.len() - 2;
    } else {
      self.switch_cursor -= 1;
    }
  }

  /// Chechs if the name typed by the user have a length between 1 and 16 and
  /// if the name is not already used
  pub fn check_name(&mut self, mut name: String) -> Option<[char; MAX_SIZE]> {
    let res;
    if name.is_empty() {
      self.error = String::from("Grid name is empty");
      res = None;
    } else {
      while name.len() < MAX_SIZE {
        name.push(' ');
      }
      if self.names.iter().any(|n| n.eq(&name)) {
        self.error = String::from("This grid name is already used");
        res = None;
      } else {
        self.error.clear();
        res =
          Some((&name.chars().collect::<Vec<char>>()[..]).try_into().unwrap());
      }
    }
    res
  }

  pub fn error_occured(&self) -> bool {
    self.error.len() > 0
  }

  pub fn get_error(&self) -> String {
    self.error.clone()
  }


  pub fn next(&mut self) {
    self.current_grid_id += 1;
    if self.current_grid_id == self.names.len() {
      self.current_grid_id = 0;
    }
  }

  pub fn previous(&mut self) {
    if self.current_grid_id == 0 {
      self.current_grid_id = self.names.len() - 1;
    } else {
      self.current_grid_id -= 1;
    }
  }

  pub fn get_nb_filledcells_current_grid(&self) -> usize {
    self.state.len()
  }

  /// Returns grids' filled cells in a HashMap of key-tuples
  /// <i>(cell X coordinate, cell Y coordinate)</i> and value-tuples
  /// <i>(grid ID, cell content)</i>
  pub fn get_grids(&self) -> HashMap<(u16, u16), (usize, FullPixel)> {
    let mut grids = HashMap::<(u16, u16), (usize, FullPixel)>::with_capacity(
      usize::try_from(self.height * self.width).unwrap());
    let mut current_grid: HashMap<(u16, u16), (usize, FullPixel)>;
    let mut current_grid_id: usize = self.names.len();
    loop {
      current_grid = self.state.iter()
        .filter_map(|(&cell, &pixel)| match cell.get_grid_id() {
                      id if id == (current_grid_id - 1) =>
                        Some(((cell.get_x(), cell.get_y()), (id, pixel))),
                      _ => None,
                    }
        ).collect();
      grids.extend(current_grid);
      current_grid_id -= 1;
      if current_grid_id == 0 {
        break;
      }
    }
    grids
  }

  /// Updates <i>state</i> depending of <i>action</i>. Returns the old value if
  /// <i>action</i> deletes of modifies a value in <i>state</i>. Returns a bool
  /// for the <i>user interface</i> if <i>action</i> inserts and overflows
  /// <i>state</i> capacity.
  pub fn update(&mut self, action: &action::Action) ->
    (Option<FullPixel>, bool) {

      match action.get_tool() {
        Tool::CellSetter(tool) => {
          self.current_grid_id = action.get_grid_id();
          let (key, value) =
            cell::Cell::new((action.get_grid_id(), action.get_grid(), tool));
          match value {
            Pixel::Empty => (self.state.remove(&key), !OVERFLOW_OCCURED),
            Pixel::Full(content) => {
              if (self.state.len() < self.map_capacity) ||
                ((self.state.len() >= self.map_capacity) &&
                self.state.contains_key(&key)) {
                  return (self.state.insert(key, content), !OVERFLOW_OCCURED);
              } else {
                return (None, OVERFLOW_OCCURED);
              }
            },
          }
        },
        Tool::GridSetter(tool) => {
          match tool {
            GridTool::ClearGrid => {
              self.current_grid_id = action.get_grid_id();
              self.clear_current_grid();
            },
            GridTool::WidthIncrementor => {
              self.increment_width();
            },
            GridTool::WidthDecrementor => {
              self.decrement_width();
            },
            GridTool::HeightIncrementor => {
              self.increment_height();
            },
            GridTool::HeightDecrementor => {
              self.decrement_height();
            },
          };
          return (None, !OVERFLOW_OCCURED);
        },
        Tool::WorkspaceSetter(tool) => {
          match tool {
            WorkspaceTool::AddGrid(name) => {
              self.add(name.iter().collect());
            },
            WorkspaceTool::RenameGrid(name) => {
              self.current_grid_id = action.get_grid_id();
              self.rename_current_grid(name.iter().collect());
            },
            WorkspaceTool::DeleteGrid => {
              self.current_grid_id = action.get_grid_id();
              self.delete_current_grid();
            },
            WorkspaceTool::SwitchGrid(switched_grid) => {
              self.current_grid_id = action.get_grid_id();
              self.switch_current_grid(switched_grid);
            },
          };
          return (None, !OVERFLOW_OCCURED);
        },
      }
  }

  pub fn get_grid_width(&self) -> u16 {
    self.width
  }

  pub fn get_grid_height(&self) -> u16 {
    self.height
  }

  fn increment_width(&mut self) {
    if self.width < SIZE_LIMIT {
      self.width += 1;
    }
  }

  fn decrement_width(&mut self) {
    if self.width > 1 {
      self.width -= 1;
      let width = self.width;
      self.state.retain(|key, _| key.get_x() < width);
    }
  }

  fn increment_height(&mut self) {
    if self.height < SIZE_LIMIT {
      self.height += 1;
    }
  }

  fn decrement_height(&mut self) {
    if self.height > 1 {
      self.height -= 1;
      let height = self.height;
      self.state.retain(|key, _| key.get_y() < height);
    }
  }

  pub fn get_scroll(&self) -> (u16, u16) {
    (self.scroll_x, self.scroll_y)
  }

  pub fn check_scroll(&mut self, (area_width, area_height): &(u16, u16)) {
    if self.width <= *area_width {
      self.scroll_x = 0;
    } else {
      self.scroll_x = min(self.scroll_x, self.width - area_width);
    }
    if self.height <= *area_height {
      self.scroll_y = 0;
    } else {
      self.scroll_y = min(self.scroll_y, self.height - area_height);
    }
  }

  pub fn scroll_up(&mut self) {
    if self.scroll_y > 0 {
      self.scroll_y -= 1;
    }
  }

  pub fn scroll_down(&mut self) {
    self.scroll_y += 1;
  }

  pub fn scroll_left(&mut self) {
    if self.scroll_x > 0 {
      self.scroll_x -= 1;
    }
  }

  pub fn scroll_right(&mut self) {
    self.scroll_x += 1;
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::log::tools;

  #[test]
  fn it_scrolls_right() {
    let mut grids = Grids::new(0);
    let init_scroll = grids.get_scroll();
    grids.scroll_right();
    assert!((init_scroll == (0, 0)) && (grids.get_scroll() == (1, 0)));
  }

  #[test]
  fn it_scrolls_left() {
    let mut grids = Grids::new(0);
    grids.scroll_right();
    let init_scroll = grids.get_scroll();
    grids.scroll_left();
    let second_scroll = grids.get_scroll();
    assert!((init_scroll == (1, 0)) && (second_scroll == (0, 0)) &&
      (grids.get_scroll() == (0, 0)));
  }

  #[test]
  fn it_scrolls_down() {
    let mut grids = Grids::new(0);
    let init_scroll = grids.get_scroll();
    grids.scroll_down();
    assert!((init_scroll == (0, 0)) && (grids.get_scroll() == (0, 1)));
  }

  #[test]
  fn it_scrolls_up() {
    let mut grids = Grids::new(0);
    grids.scroll_down();
    let init_scroll = grids.get_scroll();
    grids.scroll_up();
    let second_scroll = grids.get_scroll();
    assert!((init_scroll == (0, 1)) && (second_scroll == (0, 0)) &&
      (grids.get_scroll() == (0, 0)));
  }

  #[test]
  fn it_scrolls_in_each_direction_but_area_is_bigger_than_grid() {
    let mut grids = Grids::new(0);
    let area = &(12, 12);
    let init_scroll = grids.get_scroll();
    grids.scroll_right();
    grids.check_scroll(area);
    let second_scroll = grids.get_scroll();
    grids.scroll_left();
    grids.check_scroll(area);
    let third_scroll = grids.get_scroll();
    grids.scroll_down();
    grids.check_scroll(area);
    let forth_scroll = grids.get_scroll();
    grids.scroll_up();
    grids.check_scroll(area);
    assert!((init_scroll == (0, 0)) && (second_scroll == (0, 0)) &&
      (third_scroll == (0, 0)) && (forth_scroll == (0, 0)) &&
      (grids.get_scroll() == (0, 0)));
  }

  #[test]
  fn it_scrolls_2_times_on_the_right_but_grid_is_only_1_cell_wider_than_area() {
    let mut grids = Grids::new(0);
    let area = &(9, 10);
    let init_scroll = grids.get_scroll();
    grids.scroll_right();
    grids.check_scroll(area);
    let second_scroll = grids.get_scroll();
    grids.scroll_right();
    grids.check_scroll(area);
    assert!((init_scroll == (0, 0)) && (second_scroll == (1, 0)) &&
      (grids.get_scroll() == (1, 0)));
  }

  #[test]
  fn it_scrolls_2_times_on_the_down_but_grid_is_only_1_cell_higher_than_area() {
    let mut grids = Grids::new(0);
    let area = &(10, 9);
    let init_scroll = grids.get_scroll();
    grids.scroll_down();
    grids.check_scroll(area);
    let second_scroll = grids.get_scroll();
    grids.scroll_down();
    grids.check_scroll(area);
    assert!((init_scroll == (0, 0)) && (second_scroll == (0, 1)) &&
      (grids.get_scroll() == (0, 1)));
  }

  #[test]
  fn it_increments_height() {
    let mut grids = Grids::new(0);
    let init_height = grids.get_grid_height();
    grids.increment_height();
    let second_height = grids.get_grid_height();
    grids.height = SIZE_LIMIT;
    let third_height = grids.get_grid_height();
    grids.increment_height();
    assert!((init_height == DEFAULT_GRID_SIZE) &&
      (second_height == DEFAULT_GRID_SIZE + 1) &&
      (third_height == SIZE_LIMIT) && (grids.get_grid_height() == SIZE_LIMIT));
  }

  #[test]
  fn it_increments_width() {
    let mut grids = Grids::new(0);
    let init_width = grids.get_grid_width();
    grids.increment_width();
    let second_width = grids.get_grid_width();
    grids.width = SIZE_LIMIT;
    let third_width = grids.get_grid_width();
    grids.increment_width();
    assert!((init_width == DEFAULT_GRID_SIZE) &&
      (second_width == DEFAULT_GRID_SIZE + 1) &&
      (third_width == SIZE_LIMIT) && (grids.get_grid_width() == SIZE_LIMIT));
  }

  #[test]
  fn it_decrements_height() {
    let mut grids = Grids::new(0);
    let init_height = grids.get_grid_height();
    grids.decrement_height();
    let second_height = grids.get_grid_height();
    grids.height = 1;
    let third_height = grids.get_grid_height();
    grids.decrement_height();
    assert!((init_height == DEFAULT_GRID_SIZE) &&
      (second_height == DEFAULT_GRID_SIZE - 1) &&
      (third_height == 1) && (grids.get_grid_height() == 1));
  }

  #[test]
  fn it_decrements_width() {
    let mut grids = Grids::new(0);
    let init_width = grids.get_grid_width();
    grids.decrement_width();
    let second_width = grids.get_grid_width();
    grids.width = 1;
    let third_width = grids.get_grid_width();
    grids.decrement_width();
    assert!((init_width == DEFAULT_GRID_SIZE) &&
      (second_width == DEFAULT_GRID_SIZE - 1) &&
      (third_width == 1) && (grids.get_grid_width() == 1));
  }

  #[test]
  fn it_updates_state_with_one_brush_action_one_missed_eraser_action_and_one_eraser_action() {
    let mut grids = Grids::new(1);
    let init_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let second_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(0, 1))));
    let third_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(1, 1))));
    assert!((init_state_len == 0) && (second_state_len == 1) &&
      (third_state_len == 1) && (grids.state.len() == 0));
  }

  #[test]
  fn it_brushes_a_filled_cell_with_another_brush_and_update_function_does_not_return_overflow() {
    let mut grids = Grids::new(2);
    let init_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let second_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 1))));
    let third_state_len = grids.state.len();
    let (_, overflow) = grids.update(&action::Action::new(
      grids.get_current_grid_info(), tools::Tool::CellSetter(
        tools::CellTool::PixelBrush(FullPixel::Border, 2, 1))));
    assert!((init_state_len == 0) && (second_state_len == 1) &&
      (third_state_len == 2) && (grids.state.len() == 2) && !overflow);
  }

  #[test]
  fn it_brushes_an_empty_cell_and_update_function_returns_overflow() {
    let mut grids = Grids::new(2);
    let init_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let second_state_len = grids.state.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 1))));
    let third_state_len = grids.state.len();
    let (_, overflow) = grids.update(&action::Action::new(
      grids.get_current_grid_info(), tools::Tool::CellSetter(
        tools::CellTool::PixelBrush(FullPixel::Border, 1, 2))));
    assert!((init_state_len == 0) && (second_state_len == 1) &&
      (third_state_len == 2) && (grids.state.len() == 2) && overflow);
  }

  #[test]
  fn it_get_correct_grid_after_one_brush_action() {
    let mut grids = Grids::new(1);
    let mut current_grid = grids.get_current_grid();
    let len_init_current_grid = current_grid.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    current_grid = grids.get_current_grid();
    let check_content_current_grid =
      (current_grid[0].0 == FullPixel::Body) &&
      (current_grid[0].1 == 1) && (current_grid[0].2 == 1);
    assert!(check_content_current_grid && (len_init_current_grid == 0) &&
      (current_grid.len() == 1));
  }

  #[test]
  fn it_get_correct_grid_after_one_eraser_action() {
    let mut grids = Grids::new(1);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let mut current_grid = grids.get_current_grid();
    let len_init_current_grid = current_grid.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(1, 1))));
    current_grid = grids.get_current_grid();
    assert!((len_init_current_grid == 1) && (current_grid.len() == 0));
  }

  #[test]
  fn it_get_correct_grid_after_one_missed_eraser_action() {
    let mut grids = Grids::new(1);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let mut current_grid = grids.get_current_grid();
    let len_init_current_grid = current_grid.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(0, 1))));
    current_grid = grids.get_current_grid();
    let check_content_current_grid =
      (current_grid[0].0 == FullPixel::Body) &&
      (current_grid[0].1 == 1) && (current_grid[0].2 == 1);
    assert!(check_content_current_grid && (len_init_current_grid == 1) &&
      (current_grid.len() == 1));
  }

  #[test]
  fn it_get_correct_grid_after_one_clear_action() {
    let mut grids = Grids::new(1);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    let mut current_grid = grids.get_current_grid();
    let len_init_current_grid = current_grid.len();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::ClearGrid)));
    current_grid = grids.get_current_grid();
    assert!((len_init_current_grid == 1) && (current_grid.len() == 0));
  }

  #[test]
  fn it_goes_next_grid() {
    let mut grids = Grids::new(1);
    let current_pos = grids.current_grid_id;
    grids.add(String::from("grid2"));
    grids.add(String::from("grid3"));
    let current_pos1 = grids.current_grid_id;
    grids.next();
    let current_pos2 = grids.current_grid_id;
    grids.next();
    let current_pos3 = grids.current_grid_id;
    grids.next();
    assert!((current_pos == 0) && (current_pos1 == 2) && (current_pos2 == 0)
      && (current_pos3 == 1) && (grids.current_grid_id == 2));
  }

  #[test]
  fn it_goes_previous_grid() {
    let mut grids = Grids::new(1);
    let current_pos = grids.current_grid_id;
    grids.add(String::from("grid2"));
    grids.add(String::from("grid3"));
    let current_pos1 = grids.current_grid_id;
    grids.previous();
    let current_pos2 = grids.current_grid_id;
    grids.previous();
    let current_pos3 = grids.current_grid_id;
    grids.previous();
    assert!((current_pos == 0) && (current_pos1 == 2) && (current_pos2 == 1)
      && (current_pos3 == 0) && (grids.current_grid_id == 2));
  }

  #[test]
  fn it_checks_an_empty_grid_name() {
    let mut grids = Grids::new(1);
    grids.check_name(String::new());
    assert!(grids.error_occured() && (grids.names.len() == 1) &&
      grids.error.eq(&String::from("Grid name is empty")));
  }

  #[test]
  fn it_checks_a_correct_grid_name() {
    let mut grids = Grids::new(1);
    let name: String =
      grids.check_name(String::from("0")).unwrap().iter().collect();
    grids.add(name);
    assert!(!grids.error_occured() && (grids.names.len() == 2) &&
      grids.error.eq(&String::new()));
  }

  #[test]
  fn it_checks_an_used_grid_name() {
    let mut grids = Grids::new(1);
    let name: String =
      grids.check_name(String::from("0")).unwrap().iter().collect();
    grids.add(name.clone());
    grids.check_name(name);
    assert!(grids.error_occured() && (grids.names.len() == 2) &&
      grids.error.eq(&String::from("This grid name is already used")));
  }

  #[test]
  fn it_get_correct_grids_after_brush_actions_on_same_cells_but_on_different_grids() {
    let mut grids = Grids::new(6);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['0', '0',
      '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Border, 1, 1))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['1', '1',
      '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::BodyBorder, 1, 1))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Border, 2, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 3, 3))));
    let map_grids = grids.get_grids();
    let check_grids = (*map_grids.get(&(1, 1)).unwrap() == (0, FullPixel::Body))
      && (*map_grids.get(&(2, 2)).unwrap() == (1, FullPixel::Body))
      && (*map_grids.get(&(3, 3)).unwrap() == (2, FullPixel::Body));
    assert!(check_grids && (map_grids.len() == 3));
  }

  #[test]
  fn it_decrements_grid_id_of_cells_greater_than_the_deleted_grid_id() {
    let mut grids = Grids::new(6);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['0', '0',
      '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 3))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['1', '1',
      '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 3, 3))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 3, 4))));
    let init_len = grids.get_nb();
    grids.previous();
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::DeleteGrid)));
    let map_grids = grids.get_grids();
    let check_grids = (*map_grids.get(&(1, 1)).unwrap() == (0, FullPixel::Body))
      && (*map_grids.get(&(1, 2)).unwrap() == (0, FullPixel::Body))
      && (*map_grids.get(&(3, 3)).unwrap() == (1, FullPixel::Body))
      && (*map_grids.get(&(3, 4)).unwrap() == (1, FullPixel::Body))
      && (map_grids.get(&(2, 2)).is_none())
      && (map_grids.get(&(2, 3)).is_none());
    assert!(check_grids && (map_grids.len() == 4) && (init_len == 3) &&
      (grids.get_nb() == init_len - 1));
  }

  #[test]
  fn it_switches_grids_correctly() {
    let mut grids = Grids::new(6);
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 1))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 1, 3))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['0', '0',
      '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(['1', '1',
      '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',]))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 2))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 3))));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, 2, 4))));
    let map_grids_before = grids.get_grids();
    let check_before_switch =
      (*map_grids_before.get(&(1, 1)).unwrap() == (0, FullPixel::Body))
      && (*map_grids_before.get(&(1, 2)).unwrap() == (0, FullPixel::Body))
      && (*map_grids_before.get(&(1, 3)).unwrap() == (0, FullPixel::Body))
      && (*map_grids_before.get(&(2, 2)).unwrap() == (2, FullPixel::Body))
      && (*map_grids_before.get(&(2, 3)).unwrap() == (2, FullPixel::Body))
      && (*map_grids_before.get(&(2, 4)).unwrap() == (2, FullPixel::Body));
    grids.update(&action::Action::new(grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::SwitchGrid(0))));
    let map_grids_after = grids.get_grids();
    let check_after_switch =
      (*map_grids_after.get(&(1, 1)).unwrap() == (2, FullPixel::Body))
      && (*map_grids_after.get(&(1, 2)).unwrap() == (2, FullPixel::Body))
      && (*map_grids_after.get(&(1, 3)).unwrap() == (2, FullPixel::Body))
      && (*map_grids_after.get(&(2, 2)).unwrap() == (0, FullPixel::Body))
      && (*map_grids_after.get(&(2, 3)).unwrap() == (0, FullPixel::Body))
      && (*map_grids_after.get(&(2, 4)).unwrap() == (0, FullPixel::Body));
    assert!(check_before_switch && check_after_switch &&
      (map_grids_before.len() == 6) && (map_grids_after.len() == 6));
  }
}
