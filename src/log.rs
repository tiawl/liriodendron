//! Communication interface between other modules

extern crate num;
use num::PrimInt;

extern crate std;
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

mod action;

mod tools;
use tools::{brush, grids, texturessettings, setter::Setter};

use crate::utils::FullPixel;

use crate::task::generation::palette::PaletteGeneration;

/// Represents a communication interface between crates. Methods are using
/// this notation to optimize readability: `tool_toolmethod()`
/// when a method is directly referencing to a member tool.
pub struct Log {

  /// Stocks last user actions
  actions: VecDeque<action::Action>,

  brush: brush::Brush,
  brushcolor: brush::brushcolor::BrushColor,

  /// Delayed state of grids
  delayedgrids: grids::Grids,

  generationnumber: texturessettings::generationnumber::GenerationNumber,
  gridname: grids::gridname::GridName,

  /// Actual state of grids
  grids: grids::Grids,
  texturessettings: texturessettings::TexturesSettings,

  /// When <i>true</i>, a <i>grids</i>.<i>state</i>'s overflow occured
  overflow: bool,

  /// Cusomizable member
  queuecapacity: usize,

  /// Stocks last canceled actions
  canceledactions: Vec<action::Action>,
}

impl Log {
  pub fn new(map_capacity: usize, queue_capacity: usize) -> Log {
    Log {
      actions: VecDeque::with_capacity(queue_capacity),
      brush: brush::Brush::new(),
      brushcolor: brush::brushcolor::BrushColor::new(),
      delayedgrids: grids::Grids::new(map_capacity),
      generationnumber:
        texturessettings::generationnumber::GenerationNumber::new(),
      gridname: grids::gridname::GridName::new(),
      grids: grids::Grids::new(map_capacity),
      texturessettings: texturessettings::TexturesSettings::new(),
      overflow: false,
      queuecapacity: queue_capacity,
      canceledactions: Vec::with_capacity(queue_capacity),
    }
  }

  /************************ TOOLS MEMBERS METHODS ***************************/

                 /*************** BRUSH ******************/

  pub fn brush_next(&mut self) {
    self.brush.next();
  }

  pub fn brush_previous(&mut self) {
    self.brush.previous();
  }

  pub fn brush_getcolor<T>(&self) -> (T, T, T)
    where T: PrimInt + std::convert::From<u8>,
  {
    let (red, blue, green) = self.brush.get_color();
    (T::try_from(red).unwrap(), T::try_from(blue).unwrap(),
      T::try_from(green).unwrap())
  }

  pub fn brush_setcolor(&mut self) {
    let (red, green, blue) = self.brushcolor.get_value();
    let color = (red.parse::<u8>().unwrap(), green.parse::<u8>().unwrap(),
      blue.parse::<u8>().unwrap());
    self.brush.set_color(color);
  }

  pub fn brush_getcurrentaction(&self) -> FullPixel {
    self.brush.get_current_action()
  }

  pub fn brush_getbodyderatio(&self) -> f64 {
    self.brush.get_body_de_ratio() as f64 / 100.
  }

  pub fn brush_incrbodyderatio(&mut self) {
    self.brush.incr_body_de_ratio();
  }

  pub fn brush_decrbodyderatio(&mut self) {
    self.brush.decr_body_de_ratio();
  }

  pub fn brush_getbodydrratio(&self) -> f64 {
    self.brush.get_body_dr_ratio() as f64 / 100.
  }

  pub fn brush_incrbodydrratio(&mut self) {
    self.brush.incr_body_dr_ratio();
  }

  pub fn brush_decrbodydrratio(&mut self) {
    self.brush.drcr_body_dr_ratio();
  }

          /*************** BRUSH COLOR SETTER ******************/

  pub fn brushcolor_getcursor(&self) -> (u16, bool) {
    (u16::try_from(self.brushcolor.get_pos()).unwrap(),
      self.brushcolor.cursor_is_blinking())
  }

  pub fn brushcolor_cursorleft(&mut self) {
    self.brushcolor.cursor_left();
  }

  pub fn brushcolor_resetcursor(&mut self) {
    self.brushcolor.reset_cursor();
  }

  pub fn brushcolor_cursorright(&mut self) {
    self.brushcolor.cursor_right();
  }

  pub fn brushcolor_getvalue(&self) -> (String, String, String) {
    self.brushcolor.get_value()
  }

  pub fn brushcolor_setvalue(&mut self, number: char) {
    self.brushcolor.set_value(number);
  }

          /************ GENERATION NUMBER SETTER ***************/

  pub fn generationnumber_getcursor(&self) -> (u16, bool) {
    (u16::try_from(self.generationnumber.get_pos()).unwrap(),
      self.generationnumber.cursor_is_blinking())
  }

  pub fn generationnumber_cursorleft(&mut self) {
    self.generationnumber.cursor_left();
  }

  pub fn generationnumber_resetcursor(&mut self) {
    self.generationnumber.reset_cursor();
  }

  pub fn generationnumber_cursorright(&mut self) {
    self.generationnumber.cursor_right();
  }

  pub fn generationnumber_getvalue(&self) -> String {
    self.generationnumber.get_value()
  }

  pub fn generationnumber_setvalue(&mut self, number: char) {
    self.generationnumber.set_value(number);
  }

          /******************* GRID NAME ***********************/

  pub fn gridname_getcursor(&self) -> (u16, bool) {
    (u16::try_from(self.gridname.get_pos()).unwrap(),
      self.gridname.cursor_is_blinking())
  }

  pub fn gridname_cursorleft(&mut self) {
    self.gridname.cursor_left();
  }

  pub fn gridname_cursorright(&mut self) {
    self.gridname.cursor_right();
  }

  pub fn gridname_getvalue(&self) -> String {
    self.gridname.get_value()
  }

  pub fn gridname_setvalue(&mut self, letter: char) {
    self.gridname.set_value(letter);
  }

  pub fn gridname_backspace(&mut self) {
    self.gridname.backspace();
  }

  pub fn gridname_reset(&mut self) {
    self.gridname.reset();
  }

          /************** GRIDS & GRID SCROLL ******************/

  pub fn grids_getwidth<T>(&self) -> T
    where T: PrimInt + std::convert::From<u16>,
  {
    T::try_from(self.grids.get_grid_width()).unwrap()
  }

  pub fn grids_getheight<T>(&self) -> T
    where T: PrimInt + std::convert::From<u16>,
  {
    T::try_from(self.grids.get_grid_height()).unwrap()
  }

  pub fn grids_getlengthcapacity(&self) -> (usize, usize) {
    self.grids.get_length_capacity_state()
  }

  pub fn grids_getcurrentgrid(&self) -> Vec<(FullPixel, u16, u16)> {
    self.grids.get_current_grid()
  }

  pub fn grids_getgrids<T>(&self) -> HashMap<(T, T), (usize, FullPixel)>
    where T: PrimInt + std::convert::From<u16> + std::hash::Hash,
  {
    self.grids.get_grids().iter().map(|(key, &value)|
      ((T::try_from(key.0).unwrap(), T::try_from(key.1).unwrap()), value)
    ).collect()
  }

  pub fn grids_getnames(&self) -> Vec<String> {
    self.grids.get_names()
  }

  pub fn grids_getcurrentgridid(&self) -> usize {
    self.grids.get_current_grid_info().1
  }

  pub fn grids_getnb(&self) -> usize {
    self.grids.get_nb()
  }

  pub fn grids_add(&mut self) {
    self.check_queue_size();
    if let Some(name) = self.grids.check_name(self.gridname.get_value()) {
      let action = action::Action::new(self.grids.get_current_grid_info(),
        tools::Tool::WorkspaceSetter(tools::WorkspaceTool::AddGrid(name)));
      self.actions.push_back(action);
      self.canceledactions.clear();
    }
  }

  pub fn grids_renamecurrentgrid(&mut self) {
    self.check_queue_size();
    if let Some(name) = self.grids.check_name(self.gridname.get_value()) {
      let action = action::Action::new(self.grids.get_current_grid_info(),
        tools::Tool::WorkspaceSetter(tools::WorkspaceTool::RenameGrid(name)));
      self.actions.push_back(action);
      self.canceledactions.clear();
    }
  }

  pub fn grids_deletecurrentgrid(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(tools::WorkspaceTool::DeleteGrid));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn grids_erroroccured(&self) -> bool {
    self.grids.error_occured()
  }

  pub fn grids_geterror(&self) -> String {
    self.grids.get_error()
  }

  pub fn grids_isfull(&self) -> bool {
    self.grids.is_full()
  }

  pub fn grids_isalone(&self) -> bool {
    self.grids.is_alone()
  }

  pub fn grids_nextswitchcursor(&mut self) {
    self.grids.next_switch_cursor();
  }

  pub fn grids_previousswitchcursor(&mut self) {
    self.grids.previous_switch_cursor();
  }

  pub fn grids_getswitchcursor(&self) -> usize {
    self.grids.get_switch_cursor()
  }

  pub fn grids_switchorder(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::WorkspaceSetter(
      tools::WorkspaceTool::SwitchGrid(self.grids.get_switch_cursor())));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn grids_next(&mut self) {
    self.grids.next();
  }

  pub fn grids_previous(&mut self) {
    self.grids.previous();
  }

  pub fn gridscroll_getscroll(&self) -> (u16, u16) {
    self.grids.get_scroll()
  }

  pub fn gridscroll_checkscroll(&mut self, area_size: &(u16, u16)) {
    self.grids.check_scroll(area_size);
  }

  pub fn gridscroll_scrollup(&mut self) {
    self.grids.scroll_up();
  }

  pub fn gridscroll_scrolldown(&mut self) {
    self.grids.scroll_down();
  }

  pub fn gridscroll_scrollright(&mut self) {
    self.grids.scroll_right();
  }

  pub fn gridscroll_scrollleft(&mut self) {
    self.grids.scroll_left();
  }

          /************************ IMAGE SETTINGS **************************/

  pub fn texturessettings_getpixelratio<T>(&self) -> T
    where T: PrimInt + std::convert::From<u16>,
  {
    T::try_from(self.texturessettings.get_pixel_ratio()).unwrap()
  }

  pub fn texturessettings_incrpixelratio(&mut self) {
    self.texturessettings.incr_pixel_ratio();
  }

  pub fn texturessettings_decrpixelratio(&mut self) {
    self.texturessettings.decr_pixel_ratio();
  }

  pub fn texturessettings_getborderratio(&self) -> u8 {
    self.texturessettings.get_border_ratio()
  }

  pub fn texturessettings_getpalette(&self) -> PaletteGeneration {
    self.texturessettings.get_palette()
  }

  pub fn texturessettings_getgenerationnumber(&self) -> u16 {
    self.texturessettings.get_number()
  }

  pub fn texturessettings_setgenerationnumber(&mut self) {
    let number = self.generationnumber.get_value();
    self.texturessettings.set_number(number.parse::<u16>().unwrap());
  }

  pub fn texturessettings_getformat(&self) -> image::ImageFormat {
    self.texturessettings.get_format()
  }

  pub fn texturessettings_nextformat(&mut self) {
    self.texturessettings.next_format();
  }

  pub fn texturessettings_previousformat(&mut self) {
    self.texturessettings.previous_format();
  }

  pub fn texturessettings_getformatinfo(&self) ->
    ([image::ImageFormat; 2], usize) {
      self.texturessettings.get_format_info()
  }

          /********************* self.actions METHODS ***********************/

  fn check_queue_size(&mut self) {
    if self.actions.len() >= self.queuecapacity {
      self.delayedgrids.update(&self.actions.pop_front().unwrap());
    }
  }

  pub fn overflow(&mut self) -> bool {
    let res = self.overflow;
    self.overflow = false;
    res
  }

  pub fn brush(&mut self, (x, y): (u16, u16)) {
    self.check_queue_size();
    let (scroll_x, scroll_y) = self.grids.get_scroll();
    let new_value = self.brush.get_current_action();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        new_value, x + scroll_x, y + scroll_y)));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn erase(&mut self, (x, y): (u16, u16)) {
    self.check_queue_size();
    let (scroll_x, scroll_y) = self.grids.get_scroll();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::CellSetter(
        tools::CellTool::PixelEraser(x + scroll_x, y + scroll_y)));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn clear(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::ClearGrid));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn increment_width(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::WidthIncrementor));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn decrement_width(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::WidthDecrementor));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn increment_height(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::HeightIncrementor));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn decrement_height(&mut self) {
    self.check_queue_size();
    let action = action::Action::new(self.grids.get_current_grid_info(),
      tools::Tool::GridSetter(tools::GridTool::HeightDecrementor));
    self.actions.push_back(action);
    self.canceledactions.clear();
  }

  pub fn undo(&mut self) {
    if !self.actions.is_empty() {
      self.grids.clone_from(&self.delayedgrids);
      self.canceledactions.push(self.actions.pop_back().unwrap());
      for action in self.actions.iter() {
        self.grids.update(&action);
      }
    }
  }

  pub fn redo(&mut self) {
    if !self.canceledactions.is_empty() {
      self.actions.push_back(self.canceledactions.pop().unwrap());
      self.grids.update(self.actions.back().unwrap());
    }
  }

  pub fn check_last_action(&mut self,
    (left, right, top, bottom): (u16, u16, u16, u16)) {
      if let Some(last) = self.actions.pop_back() {
        if !last.is_checked() {
          match last.get_tool() {
            tools::Tool::CellSetter(tool) => {
              match tool {
                tools::CellTool::PixelBrush(pixel, x, y) => {
                  if (x >= left) && (x < right) &&
                    (y >= top) && (y < bottom) {
                      let corrected_last_action =
                        last.corrected(Some(left), Some(top));
                      match self.grids.update(&corrected_last_action) {
                        (Some(old_value), _) => {
                          if old_value != pixel {
                            self.actions.push_back(corrected_last_action);
                          }
                          self.overflow = false;
                        },
                        (None, overflow) => {
                          if !overflow {
                            self.actions.push_back(corrected_last_action);
                          }
                          self.overflow = overflow;
                        },
                      };
                  }
                },
                tools::CellTool::PixelEraser(x, y) => {
                  if (x >= left) && (x < right) && (y >= top) && (y < bottom) {
                    let corrected_last_action =
                      last.corrected(Some(left), Some(top));
                    match self.grids.update(&corrected_last_action) {
                      (Some(_), _) =>
                        self.actions.push_back(corrected_last_action),
                      (None, _) => {},
                    };
                    self.overflow = false;
                  }
                },
              };
            },
            tools::Tool::GridSetter(tool) => {
              let corrected_last_action = last.corrected(None, None);
              match tool {
                tools::GridTool::ClearGrid => {
                  if self.grids.get_nb_filledcells_current_grid() > 0 {
                    self.grids.update(&corrected_last_action);
                    self.actions.push_back(corrected_last_action);
                  }
                },
                tools::GridTool::WidthIncrementor => {
                  if self.grids.get_grid_width() < grids::size_limit() {
                    self.grids.update(&corrected_last_action);
                    self.actions.push_back(corrected_last_action);
                  }
                },
                tools::GridTool::WidthDecrementor => {
                  if self.grids.get_grid_width() > 1 {
                    self.grids.update(&corrected_last_action);
                    self.actions.push_back(corrected_last_action);
                  }
                },
                tools::GridTool::HeightIncrementor => {
                  if self.grids.get_grid_height() < grids::size_limit() {
                    self.grids.update(&corrected_last_action);
                    self.actions.push_back(corrected_last_action);
                  }
                },
                tools::GridTool::HeightDecrementor => {
                  if self.grids.get_grid_height() > 1 {
                    self.grids.update(&corrected_last_action);
                    self.actions.push_back(corrected_last_action);
                  }
                },
              };
              self.overflow = false;
            },
            tools::Tool::WorkspaceSetter(tool) => {
              let corrected_last_action = last.corrected(None, None);
              match tool {
                tools::WorkspaceTool::AddGrid(_) |
                tools::WorkspaceTool::RenameGrid(_) |
                tools::WorkspaceTool::DeleteGrid |
                tools::WorkspaceTool::SwitchGrid(_) => {
                  self.grids.update(&corrected_last_action);
                  self.actions.push_back(corrected_last_action);
                },
              };
              self.overflow = false;
            },
          };
        } else {
          self.actions.push_back(last);
        }
      }
  }
}

#[cfg(test)]
pub mod tests {

  use super::*;

  #[test]
  fn it_adds_a_brush_action() {
    let mut log = Log::new(1, 1);
    let len = log.actions.len();
    log.brush((1, 1));
    assert_eq!(log.actions.len(), len + 1);
  }

  #[test]
  fn it_adds_an_erase_action() {
    let mut log = Log::new(1, 1);
    let len = log.actions.len();
    log.erase((1, 1));
    assert_eq!(log.actions.len(), len + 1);
  }

  #[test]
  fn it_adds_a_clear_action() {
    let mut log = Log::new(1, 2);
    log.brush((1, 1));
    let len = log.actions.len();
    log.clear();
    assert_eq!(log.actions.len(), len + 1);
  }

  #[test]
  fn it_adds_an_increment_width_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.increment_width();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_an_increment_height_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.increment_height();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_a_decrement_width_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.decrement_width();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_a_decrement_height_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.decrement_height();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_an_addgrid_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.gridname_setvalue('0');
    log.grids_add();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_a_renamegrid_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.gridname_setvalue('0');
    log.grids_renamecurrentgrid();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_a_deletegrid_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.grids_deletecurrentgrid();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_adds_a_switchgrid_action() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.grids_switchorder();
    assert!(log.actions.len() == init_len + 1);
  }

  #[test]
  fn it_does_not_add_an_addgrid_action_with_an_empty_grid_name() {
    let mut log = Log::new(0, 1);
    let init_len = log.actions.len();
    log.grids_add();
    assert!(log.actions.len() == init_len);
  }

  #[test]
  fn it_does_not_add_an_addgrid_action_with_an_used_grid_name() {
    let mut log = Log::new(0, 2);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.gridname_reset();
    log.gridname_setvalue('0');
    log.grids_add();
    assert!(log.actions.len() == init_len);
  }

  #[test]
  fn it_brushes_the_correct_grid_after_an_addgrid_action() {
    let mut log = Log::new(2, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();
    log.brush((2, 2));
    log.check_last_action(grid_area);
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    assert!((log.actions.len() == 3) && (filled_cells_grid1_a == 0) &&
      (filled_cells_grid2_a == 1) && (filled_cells_grid1_b == 1) &&
      (filled_cells_grid2_b == 1));
  }

  #[test]
  fn it_erases_the_correct_grid_after_an_addgrid_action() {
    let mut log = Log::new(2, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.erase((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();

    log.brush((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid1_c = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_c = log.grids.get_current_grid().len();

    log.erase((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid2_d = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_d = log.grids.get_current_grid().len();
    log.grids_next();

    log.brush((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid2_e = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_e = log.grids.get_current_grid().len();

    log.erase((1, 1));
    log.check_last_action(grid_area);
    let filled_cells_grid1_f = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_f = log.grids.get_current_grid().len();

    assert!((filled_cells_grid1_a == 0) && (filled_cells_grid2_a == 1) &&
      (filled_cells_grid1_b == 0) && (filled_cells_grid2_b == 1) &&
      (filled_cells_grid1_c == 1) && (filled_cells_grid2_c == 1) &&
      (filled_cells_grid1_d == 1) && (filled_cells_grid2_d == 0) &&
      (filled_cells_grid1_e == 1) && (filled_cells_grid2_e == 1) &&
      (filled_cells_grid1_f == 0) && (filled_cells_grid2_f == 1));
  }

  #[test]
  fn it_clears_the_correct_grid_after_an_addgrid_action() {
    let mut log = Log::new(2, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.brush((1, 1));
    log.check_last_action(grid_area);
    log.grids_add();
    log.check_last_action(grid_area);

    log.brush((2, 2));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.clear();
    log.check_last_action(grid_area);
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_b = log.grids.get_current_grid().len();

    log.grids_previous();
    log.brush((2, 2));
    log.check_last_action(grid_area);
    let filled_cells_grid1_c = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_c = log.grids.get_current_grid().len();

    log.clear();
    log.check_last_action(grid_area);
    let filled_cells_grid2_d = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_d = log.grids.get_current_grid().len();

    assert!((filled_cells_grid1_a == 1) && (filled_cells_grid2_a == 1) &&
      (filled_cells_grid1_b == 0) && (filled_cells_grid2_b == 1) &&
      (filled_cells_grid1_c == 1) && (filled_cells_grid2_c == 1) &&
      (filled_cells_grid1_d == 1) && (filled_cells_grid2_d == 0));
  }

  #[test]
  fn it_renames_the_correct_grids_after_an_addgrid_action() {
    let mut log = Log::new(2, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    let check1 =
      log.grids_getnames()[0].eq(&String::from("Grid1           ")) &&
      log.grids_getnames()[1].eq(&String::from("0               ")) &&
      log.grids_getnb() == 2;

    log.grids_previous();
    log.gridname_reset();
    log.gridname_setvalue('b');
    log.gridname_setvalue('l');
    log.gridname_setvalue('i');
    log.gridname_setvalue('b');
    log.gridname_setvalue('l');
    log.gridname_setvalue('i');
    log.grids_renamecurrentgrid();
    log.check_last_action(grid_area);
    let check2 =
      log.grids_getnames()[0].eq(&String::from("blibli          ")) &&
      log.grids_getnames()[1].eq(&String::from("0               ")) &&
      log.grids_getnb() == 2;

    log.grids_next();
    log.gridname_reset();
    log.gridname_setvalue('t');
    log.gridname_setvalue('l');
    log.gridname_setvalue('i');
    log.gridname_setvalue('t');
    log.gridname_setvalue('l');
    log.gridname_setvalue('i');
    log.grids_renamecurrentgrid();
    log.check_last_action(grid_area);
    let check3 =
      log.grids_getnames()[0].eq(&String::from("blibli          ")) &&
      log.grids_getnames()[1].eq(&String::from("tlitli          ")) &&
      log.grids_getnb() == 2;

    log.undo();
    let check4 =
      log.grids_getnames()[0].eq(&String::from("blibli          ")) &&
      log.grids_getnames()[1].eq(&String::from("0               ")) &&
      log.grids_getnb() == 2;

    log.redo();
    let check5 =
      log.grids_getnames()[0].eq(&String::from("blibli          ")) &&
      log.grids_getnames()[1].eq(&String::from("tlitli          ")) &&
      log.grids_getnb() == 2;

    log.undo();
    log.undo();
    let check6 =
      log.grids_getnames()[0].eq(&String::from("Grid1           ")) &&
      log.grids_getnames()[1].eq(&String::from("0               ")) &&
      log.grids_getnb() == 2;

    log.undo();
    let check7 =
      log.grids_getnames()[0].eq(&String::from("Grid1           ")) &&
      log.grids_getnb() == 1;

    assert!(check1 && check2 && check3 && check4 && check5 && check6 && check7);
  }

  #[test]
  fn it_deletes_a_grid() {
    let mut log = Log::new(0, 2);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    let init_len = log.grids_getnb();
    log.grids_deletecurrentgrid();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getnb() == init_len - 1));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_width_and_keeps_cells_of_first_grid() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((5, 1));
    log.check_last_action(grid_area);
    log.brush((5, 2));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((9, 1));
    log.check_last_action(grid_area);
    log.brush((9, 2));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.decrement_width();
    log.check_last_action(grid_area);
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 2) && (filled_cells_grid2_b == 0));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_width_and_keeps_cells_of_second_grid() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((5, 1));
    log.check_last_action(grid_area);
    log.brush((5, 2));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((9, 1));
    log.check_last_action(grid_area);
    log.brush((9, 2));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_width();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 2) && (filled_cells_grid2_b == 0));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_width_and_keeps_cells_of_none_of_them() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((9, 1));
    log.check_last_action(grid_area);
    log.brush((9, 2));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((9, 3));
    log.check_last_action(grid_area);
    log.brush((9, 4));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_width();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 0) && (filled_cells_grid2_b == 0));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_width_and_keeps_cells_of_both_of_them() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((7, 1));
    log.check_last_action(grid_area);
    log.brush((7, 2));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((8, 3));
    log.check_last_action(grid_area);
    log.brush((8, 4));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_width();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 2) && (filled_cells_grid2_b == 2));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_height_and_keeps_cells_of_first_grid() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((5, 8));
    log.check_last_action(grid_area);
    log.brush((5, 7));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((7, 9));
    log.check_last_action(grid_area);
    log.brush((6, 9));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.decrement_height();
    log.check_last_action(grid_area);
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    log.grids_next();
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 2) && (filled_cells_grid2_b == 0));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_height_and_keeps_cells_of_second_grid() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getheight(), 0, log.grids_getheight());
    log.brush((4, 9));
    log.check_last_action(grid_area);
    log.brush((3, 9));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((6, 8));
    log.check_last_action(grid_area);
    log.brush((5, 8));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_height();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 0) && (filled_cells_grid2_b == 2));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_height_and_keeps_cells_of_none_of_them() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getheight(), 0, log.grids_getheight());
    log.brush((4, 9));
    log.check_last_action(grid_area);
    log.brush((5, 9));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((6, 9));
    log.check_last_action(grid_area);
    log.brush((7, 9));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_height();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 0) && (filled_cells_grid2_b == 0));
  }

  #[test]
  fn it_brushes_2_grids_and_decrements_height_and_keeps_cells_of_both_of_them() {
    let mut log = Log::new(4, 3);
    let grid_area = (0, log.grids_getheight(), 0, log.grids_getheight());
    log.brush((4, 6));
    log.check_last_action(grid_area);
    log.brush((5, 6));
    log.check_last_action(grid_area);
    log.gridname_setvalue('0');
    log.grids_add();
    log.check_last_action(grid_area);
    log.brush((6, 6));
    log.check_last_action(grid_area);
    log.brush((7, 6));
    log.check_last_action(grid_area);
    let filled_cells_grid2_a = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_a = log.grids.get_current_grid().len();

    log.grids_next();
    log.decrement_height();
    log.check_last_action(grid_area);
    let filled_cells_grid2_b = log.grids.get_current_grid().len();
    log.grids_previous();
    let filled_cells_grid1_b = log.grids.get_current_grid().len();
    assert!((filled_cells_grid1_a == 2) && (filled_cells_grid2_a == 2) &&
      (filled_cells_grid1_b == 2) && (filled_cells_grid2_b == 2));
  }

  #[test]
  fn it_canceled_the_last_action() {
    let mut log = Log::new(1, 1);
    log.brush((1, 1));
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.undo();
    assert!((actions_len == 1) && (canceled_actions_len == 0) &&
      (log.actions.len() == 0) && (log.canceledactions.len() == 1));
  }

  #[test]
  fn it_canceled_several_last_actions() {
    let mut log = Log::new(3, 3);
    log.brush((1, 1));
    log.brush((2, 1));
    log.brush((1, 2));
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.undo();
    log.undo();
    log.undo();
    assert!((actions_len == 3) && (canceled_actions_len == 0) &&
      (log.actions.len() == 0) && (log.canceledactions.len() == 3));
  }

  #[test]
  fn it_did_not_cancel_last_action_because_there_are_no_last_action() {
    let mut log = Log::new(0, 1);
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.undo();
    assert!((actions_len == 0) && (canceled_actions_len == 0) &&
      (log.actions.len() == 0) && (log.canceledactions.len() == 0));
  }

  #[test]
  fn it_clears_canceled_actions_after_an_action() {
    let mut log = Log::new(3, 3);
    log.brush((1, 1));
    log.brush((2, 1));
    log.brush((1, 2));
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.undo();
    log.undo();
    let actions_len2 = log.actions.len();
    let canceled_actions_len2 = log.canceledactions.len();
    log.brush((1, 2));
    assert!((actions_len == 3) && (canceled_actions_len == 0) &&
      (actions_len2 == 1) && (canceled_actions_len2 == 2) &&
      (log.actions.len() == 2) && (log.canceledactions.len() == 0));
  }

  #[test]
  fn it_redid_the_last_action() {
    let mut log = Log::new(1, 1);
    log.brush((1, 1));
    log.undo();
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.redo();
    assert!((actions_len == 0) && (canceled_actions_len == 1) &&
      (log.actions.len() == 1) && (log.canceledactions.len() == 0));
  }

  #[test]
  fn it_redid_several_last_actions() {
    let mut log = Log::new(3, 3);
    log.brush((1, 1));
    log.brush((2, 1));
    log.brush((1, 2));
    log.undo();
    log.undo();
    log.undo();
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.redo();
    log.redo();
    log.redo();
    assert!((actions_len == 0) && (canceled_actions_len == 3) &&
      (log.actions.len() == 3) && (log.canceledactions.len() == 0));
  }

  #[test]
  fn it_did_not_redo_last_action_because_there_are_no_canceled_action() {
    let mut log = Log::new(1, 1);
    log.brush((1, 1));
    let actions_len = log.actions.len();
    let canceled_actions_len = log.canceledactions.len();
    log.redo();
    assert!((actions_len == 1) && (canceled_actions_len == 0) &&
      (log.actions.len() == 1) && (log.canceledactions.len() == 0));
  }

  #[test]
  fn it_pushed_last_poped_action_inside_delayed_grid_when_log_is_full() {
    let mut log = Log::new(3, 1);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((1, 1));
    log.check_last_action(grid_area);
    let actions_len = log.actions.len();
    let delayed_len = log.delayedgrids.get_nb_filledcells_current_grid();
    log.brush((1, 2));
    log.check_last_action(grid_area);
    let actions_len2 = log.actions.len();
    let delayed_len2 = log.delayedgrids.get_nb_filledcells_current_grid();
    log.brush((2, 2));
    log.check_last_action(grid_area);
    assert!((actions_len == 1) && (delayed_len == 0) &&
      (actions_len2 == 1) && (delayed_len2 == 1) && (log.actions.len() == 1)
      && (log.delayedgrids.get_nb_filledcells_current_grid() == 2));
  }

  #[test]
  fn it_pushed_last_poped_action_inside_delayed_grid_when_log_is_full_but_this_last_action_is_repetitive_so_log_poped_it_from_actions() {
    let mut log = Log::new(3, 2);
    let grid_area = (0, log.grids_getwidth(), 0, log.grids_getheight());
    log.brush((1, 1));
    log.check_last_action(grid_area);
    log.brush((1, 2));
    log.check_last_action(grid_area);
    let actions_len = log.actions.len();
    let delayed_len = log.delayedgrids.get_nb_filledcells_current_grid();
    log.brush((1, 2));
    log.check_last_action(grid_area);
    let actions_len2 = log.actions.len();
    let delayed_len2 = log.delayedgrids.get_nb_filledcells_current_grid();
    log.brush((2, 2));
    log.check_last_action(grid_area);
    assert!((actions_len == 2) && (delayed_len == 0) &&
      (actions_len2 == 1) && (delayed_len2 == 1) && (log.actions.len() == 2)
      && (log.delayedgrids.get_nb_filledcells_current_grid() == 1));
  }

  #[test]
  fn it_pops_1_action_when_1_new_action_is_pushed_inside_a_full_log() {
    let mut log = Log::new(2, 256);
    while log.actions.len() < log.queuecapacity {
      log.brush((1, 1));
    }
    let init_len = log.actions.len();
    log.brush((2, 2));
    if let Some(last_brush) = log.actions.back() {
      assert!((init_len == log.queuecapacity) &&
        (log.actions.len() == log.queuecapacity) &&
        (*last_brush == action::Action::new(log.grids.get_current_grid_info(),
          tools::Tool::CellSetter(tools::CellTool::PixelBrush(
            log.brush.get_current_action(), 2, 2)))));
    }
  }

  #[test]
  fn it_checks_6_last_action_outside_the_grid_and_pops_it(){
    let mut log = Log::new(6, 6);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (5, init_width, 5, init_height);
    log.brush((init_width + 4, init_height + 4));
    log.check_last_action(grid_area);
    log.brush((init_width + 4, init_height - 1));
    log.check_last_action(grid_area);
    log.brush((init_width - 1, init_height + 4));
    log.check_last_action(grid_area);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((4, init_width - 1));
    log.check_last_action(grid_area);
    log.brush((init_width - 1, 4));
    log.check_last_action(grid_area);
    assert_eq!(log.actions.len(), 0);
  }

  #[test]
  fn it_checks_last_brush_action_inside_the_grid_and_keeps_it(){
    let mut log = Log::new(1, 1);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    assert_eq!(log.actions.len(), 1);
  }

  #[test]
  fn it_checks_3_same_brush_action_inside_the_grid_and_pops_2_of_them(){
    let mut log = Log::new(1, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    assert_eq!(log.actions.len(), 1);
  }

  #[test]
  fn it_checks_3_same_eraser_action_inside_the_grid_and_pops_2_of_them(){
    let mut log = Log::new(1, 4);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.erase((4, 4));
    log.check_last_action(grid_area);
    log.erase((4, 4));
    log.check_last_action(grid_area);
    log.erase((4, 4));
    log.check_last_action(grid_area);
    assert_eq!(log.actions.len(), 2);
  }

  #[test]
  fn it_checks_3_same_clear_action_and_pops_2_of_them(){
    let mut log = Log::new(1, 4);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.clear();
    log.check_last_action(grid_area);
    log.clear();
    log.check_last_action(grid_area);
    log.clear();
    log.check_last_action(grid_area);
    assert_eq!(log.actions.len(), 2);
  }

  #[test]
  fn it_checks_4_last_decrement_actions_and_keeps_it(){
    let mut log = Log::new(0, 4);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    log.decrement_width();
    log.check_last_action((0, init_width - 1, 0, init_height));
    log.decrement_width();
    log.check_last_action((0, init_width - 2, 0, init_height));
    log.decrement_height();
    log.check_last_action((0, init_width - 2, 0, init_height - 1));
    log.decrement_height();
    log.check_last_action((0, init_width - 2, 0, init_height - 2));
    assert!((log.actions.len() == 4) &&
      (log.grids_getwidth::<u16>() == init_width - 2) &&
      (log.grids_getheight::<u16>() == init_height - 2));
  }

  #[test]
  fn it_checks_4_last_increment_actions_and_keeps_it(){
    let mut log = Log::new(0, 4);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    log.increment_width();
    log.check_last_action((0, init_width + 1, 0, init_height));
    log.increment_width();
    log.check_last_action((0, init_width + 2, 0, init_height));
    log.increment_height();
    log.check_last_action((0, init_width + 2, 0, init_height + 1));
    log.increment_height();
    log.check_last_action((0, init_width + 2, 0, init_height + 2));
    assert!((log.actions.len() == 4) &&
      (log.grids_getwidth::<u16>() == init_width + 2) &&
      (log.grids_getheight::<u16>() == init_height + 2));
  }

  #[test]
  fn it_checks_last_brush_action_on_a_filled_cell_and_it_does_not_overflow_memory_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((5, 4));
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 2) && !log.overflow);
  }

  #[test]
  fn it_checks_last_brush_action_on_an_empty_cell_and_it_overflows_memory_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 2) && log.overflow);
  }

  #[test]
  fn it_checks_last_brush_action_on_a_filled_cell_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.brush((5, 4));
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 2) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_checks_last_eraser_action_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.erase((5, 4));
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_checks_last_missed_eraser_action_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.erase((6, 4));
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 2) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_checks_last_clear_action_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.clear();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_increments_width_grid_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.increment_width();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_increments_height_grid_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.increment_height();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_decrements_width_grid_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.decrement_width();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_decrements_height_grid_after_a_memory_grids_overflow_and_it_does_not_overflow_grids(){
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth::<u16>(), log.grids_getheight::<u16>());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((4, 4));
    log.check_last_action(grid_area);
    log.brush((5, 4));
    log.check_last_action(grid_area);
    let init_len = log.actions.len();
    log.brush((6, 4));
    log.check_last_action(grid_area);
    let was_overflow = log.overflow;
    log.decrement_height();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.actions.len() == 3) && !log.overflow &&
      was_overflow);
  }

  #[test]
  fn it_brushed_an_empty_cell_and_gets_a_filler_grid() {
    let mut log = Log::new(1, 1);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    let init_len = log.grids_getcurrentgrid().len();
    log.brush((6, 6));
    log.check_last_action(grid_area);
    assert!((init_len == 0) && (log.grids_getcurrentgrid().len() == 1));
  }

  #[test]
  fn it_brushed_a_filled_cell_and_gets_the_same_grid_and_a_len_log_equal_to_1() {
    let mut log = Log::new(2, 2);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((6, 6));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.brush((6, 6));
    log.check_last_action(grid_area);
    assert!((init_len == 1) && (log.grids_getcurrentgrid().len() == 1) &&
      (log.actions.len() == 1));
  }

  #[test]
  fn it_erased_a_filled_cell_and_gets_an_emptier_grid() {
    let mut log = Log::new(1, 2);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((6, 6));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.erase((6, 6));
    log.check_last_action(grid_area);
    assert!((init_len == 1) && (log.grids_getcurrentgrid().len() == 0));
  }

  #[test]
  fn it_erased_an_empty_cell_and_gets_the_same_grid_and_an_empty_log() {
    let mut log = Log::new(0, 1);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    let init_len = log.grids_getcurrentgrid().len();
    log.erase((6, 6));
    log.check_last_action(grid_area);
    assert!((init_len == 0) && (log.grids_getcurrentgrid().len() == 0) &&
      (log.actions.len() == 0));
  }

  #[test]
  fn it_cleared_a_filled_cell_and_gets_an_empty_grid() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((6, 6));
    log.check_last_action(grid_area);
    log.brush((7, 7));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.clear();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 0));
  }

  #[test]
  fn it_cleared_an_empty_grid_and_gets_the_same_grid_and_an_empty_log() {
    let mut log = Log::new(0, 1);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    let init_len = log.grids_getcurrentgrid().len();
    log.clear();
    log.check_last_action(grid_area);
    assert!((init_len == 0) && (log.grids_getcurrentgrid().len() == 0) &&
      (log.actions.len() == 0));
  }


  #[test]
  fn it_brushed_an_empty_cell_1_but_decrements_width_and_erase_cell_1() {
    let mut log = Log::new(1, 2);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((9, 1));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_width();
    log.check_last_action(grid_area);
    assert!((init_len == 1) && (log.grids_getcurrentgrid().len() == 0) &&
      (init_width == 10) && (log.grids_getwidth::<u16>() == init_width - 1));
  }

  #[test]
  fn it_brushed_an_empty_cell_1_but_decrements_height_and_erase_cell_1() {
    let mut log = Log::new(1, 2);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((1, 9));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_height();
    log.check_last_action(grid_area);
    assert!((init_len == 1) && (log.grids_getcurrentgrid().len() == 0) &&
      (init_height == 10) && (log.grids_getheight::<u16>() == init_height - 1));
  }

  #[test]
  fn it_brushed_2_empty_cell_1_and_2_but_decrements_width_and_erase_cell_1() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((9, 1));
    log.check_last_action(grid_area);
    log.brush((8, 2));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_width();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 1) &&
      (init_width == 10) && (log.grids_getwidth::<u16>() == init_width - 1));
  }

  #[test]
  fn it_brushed_2_empty_cell_1_and_2_but_decrements_height_and_erase_cell_1() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((1, 8));
    log.check_last_action(grid_area);
    log.brush((1, 9));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_height();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 1) &&
      (init_height == 10) && (log.grids_getheight::<u16>() == init_height - 1));
  }

  #[test]
  fn it_brushed_2_empty_cells_but_decrements_width_and_erase_both() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((9, 1));
    log.check_last_action(grid_area);
    log.brush((9, 2));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_width();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 0) &&
      (init_width == 10) && (log.grids_getwidth::<u16>() == init_width - 1));
  }

  #[test]
  fn it_brushed_2_empty_cells_but_decrements_height_and_erase_both() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((2, 9));
    log.check_last_action(grid_area);
    log.brush((1, 9));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_height();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 0) &&
      (init_height == 10) && (log.grids_getheight::<u16>() == init_height - 1));
  }

  #[test]
  fn it_brushed_2_empty_cells_and_decrements_width() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((7, 1));
    log.check_last_action(grid_area);
    log.brush((8, 1));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_width();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 2) &&
      (init_width == 10) && (log.grids_getwidth::<u16>() == init_width - 1));
  }

  #[test]
  fn it_brushed_2_empty_cells_and_decrements_height() {
    let mut log = Log::new(2, 3);
    let (init_width, init_height) =
      (log.grids_getwidth(), log.grids_getheight());
    let grid_area = (0, init_width, 0, init_height);
    log.brush((1, 7));
    log.check_last_action(grid_area);
    log.brush((1, 8));
    log.check_last_action(grid_area);
    let init_len = log.grids_getcurrentgrid().len();
    log.decrement_height();
    log.check_last_action(grid_area);
    assert!((init_len == 2) && (log.grids_getcurrentgrid().len() == 2) &&
      (init_height == 10) &&
      (log.grids_getheight::<u16>() == init_height - 1));
  }
}
