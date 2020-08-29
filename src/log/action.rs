use crate::log::tools;

/// Represents a user action
#[derive(Clone, Copy)]
pub struct Action {
  grid_id: usize,
  grid: tools::Grid,
  tool: tools::Tool,

  /// The <i>log</i> grid's coordinates and the <i>user interface</i> grid's
  /// coordinates are different. In <i>log</i> grid's coordinates starts
  /// to 0, but in the the <i>user interface</i>, it depends of the terminal
  /// size and where the grid area is supposed to be on the <i>user
  /// interface</i>. The <i>log</i> have to modify it's own grid's coordinates
  /// system to fit with the grid's coordinates system. The <i>checked</i>
  /// member allows to know if the <i>log</i> fitted its grid's coordinates
  /// system for the tested action. The <i>checked</i> member only matters for
  /// cells setter tools.
  checked: bool,
}

impl PartialEq for Action {
  fn eq(&self, other: &Self) -> bool {
    (self.grid_id == other.grid_id) && (self.grid == other.grid) &&
    (self.tool == other.tool)
  }
}

impl Action {
  pub fn new((grid, grid_id): (tools::Grid, usize), tool: tools::Tool)
    -> Action {
      Action {
        grid_id: grid_id,
        grid: grid,
        tool: tool,
        checked: false,
      }
  }

  pub fn get_grid_id(&self) -> usize {
    self.grid_id
  }

  pub fn get_grid(&self) -> tools::Grid {
    self.grid
  }

  pub fn get_tool(&self) -> tools::Tool {
    self.tool
  }

  pub fn is_checked(&self) -> bool {
    self.checked
  }

  /// Creates a new Action object with fitted grid's coordinates to the
  /// <i>user interface</i> system.
  pub fn corrected(&self, left: Option<u16>, top: Option<u16>) -> Action {
    if !self.is_checked() {
      Action {
        grid_id: self.grid_id,
        grid: self.grid,
        tool: match self.tool {
                tools::Tool::WorkspaceSetter(_) => self.tool,
                tools::Tool::GridSetter(_) => self.tool,
                tools::Tool::CellSetter(cell_tool) => {
                  match cell_tool {
                    tools::CellTool::PixelBrush(pixel, x, y) =>
                      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
                        pixel, x - left.unwrap(), y - top.unwrap())),
                    tools::CellTool::PixelEraser(x, y) =>
                      tools::Tool::CellSetter(tools::CellTool::PixelEraser(
                        x - left.unwrap(), y - top.unwrap())),
                  }
                },
              },
        checked: true,
      }
    } else {
      *self
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::utils::FullPixel;

  #[test]
  fn it_corrects_a_brush_action() {
    let (left_correction, top_correction) = (Some(5), Some(2));
    let (brush_x, brush_y) = (8, 8);
    let mut action = Action::new((tools::Grid::Generation, 0),
      tools::Tool::CellSetter(tools::CellTool::PixelBrush(
        FullPixel::Body, brush_x, brush_y)));
    let not_checked = action.checked;
    action = action.corrected(left_correction, top_correction);
    if let tools::Tool::CellSetter(cell_tool) = action.get_tool() {
      if let tools::CellTool::PixelBrush(_pixel, x, y) = cell_tool {
        assert!(!not_checked && action.checked &&
          (x == brush_x - left_correction.unwrap()) &&
          (y == brush_y - top_correction.unwrap()));
      }
    }
  }

  #[test]
  fn it_corrects_an_eraser_action() {
    let (left_correction, top_correction) = (Some(2), Some(5));
    let (eraser_x, eraser_y) = (3, 18);
    let mut action = Action::new((tools::Grid::Generation, 0),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(
        eraser_x, eraser_y)));
    let not_checked = action.checked;
    action = action.corrected(left_correction, top_correction);
    if let tools::Tool::CellSetter(cell_tool) = action.get_tool() {
      if let tools::CellTool::PixelEraser(x, y) = cell_tool {
        assert!(!not_checked && action.checked &&
          (x == eraser_x - left_correction.unwrap()) &&
          (y == eraser_y - top_correction.unwrap()));
      }
    }
  }

  #[test]
  fn it_does_not_correct_a_checked_action() {
    let (left_correction, top_correction) = (Some(2), Some(5));
    let (eraser_x, eraser_y) = (3, 18);
    let mut action = Action::new((tools::Grid::Generation, 0),
      tools::Tool::CellSetter(tools::CellTool::PixelEraser(
        eraser_x, eraser_y)));
    action.checked = true;
    let checked = action.checked;
    action = action.corrected(left_correction, top_correction);
    if let tools::Tool::CellSetter(cell_tool) = action.get_tool() {
      if let tools::CellTool::PixelEraser(x, y) = cell_tool {
        assert!(checked && action.checked && (x == eraser_x) &&
          (y == eraser_y));
      }
    }
  }
}
