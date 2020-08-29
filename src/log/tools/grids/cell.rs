use crate::log::tools::{CellTool, Grid, Pixel};

/// Represents one cell of a grid
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cell {
  grid_id: usize,
  grid: Grid,
  x: u16,
  y: u16,
}

impl Cell {
  pub fn new((grid_id, grid, tool): (usize, Grid, CellTool)) ->
    (Cell, Pixel) {
      match tool {
        CellTool::PixelBrush(pixel, x , y) => return (Cell {
            grid_id: grid_id,
            grid: grid,
            x: x,
            y: y,
          }, Pixel::Full(pixel)),
        CellTool::PixelEraser(x, y) => return (Cell {
            grid_id: grid_id,
            grid: grid,
            x: x,
            y: y,
          }, Pixel::Empty),
      }
  }

  /// Returns a new Cell object with a new grid id and other fields cell
  /// caller
  pub fn set_grid_id(&self, new_grid_id: usize) -> Cell {
    Cell {
      grid_id: new_grid_id,
      grid: self.grid,
      x: self.x,
      y: self.y,
    }
  }

  pub fn get_grid_id(&self) -> usize {
    self.grid_id
  }

  pub fn get_grid(&self) -> Grid {
    self.grid
  }

  pub fn get_x(&self) -> u16 {
    self.x
  }

  pub fn get_y(&self) -> u16 {
    self.y
  }
}
