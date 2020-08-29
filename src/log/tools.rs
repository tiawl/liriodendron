pub mod brush;
pub mod grids;
pub mod texturessettings;

/// Tools and settings where <i>user interface</i> requests user's
/// inputs
pub mod setter;

use crate::utils::FullPixel;

const MAX_SIZE: usize = 16;

/// Possible content for a cell
#[derive(Clone, Copy)]
pub enum Pixel {
  Empty,
  Full(FullPixel),
}

/// Grid types
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Grid {
  Generation,
}

/// Workspace tools: grids management
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum WorkspaceTool {
  AddGrid([char; MAX_SIZE]),
  RenameGrid([char; MAX_SIZE]),
  DeleteGrid,
  SwitchGrid(usize),
}

/// Grid tools: grid cleaner and grid size incrementors and decrementors
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GridTool {
  ClearGrid,
  HeightDecrementor,
  HeightIncrementor,
  WidthDecrementor,
  WidthIncrementor,
}

/// Grid cells' content setters
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CellTool {
  PixelBrush(FullPixel, u16, u16),
  PixelEraser(u16, u16),
}

/// Tool types
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Tool {
  WorkspaceSetter(WorkspaceTool),
  GridSetter(GridTool),
  CellSetter(CellTool),
}
