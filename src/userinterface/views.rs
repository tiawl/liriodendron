extern crate num;
use num::PrimInt;

pub mod edit;
pub mod exit;
pub mod generationerror;
pub mod gridname;
pub mod logoverflow;
pub mod gridnumbererror;
pub mod parameters;
pub mod resizeerror;
pub mod setbrushcolor;
pub mod setgenerationnumber;
pub mod settextureformat;
pub mod shutdown;
pub mod switchgridserror;
pub mod switchgridsorder;
pub mod unavailablethread;

use crate::userinterface::widgets::{
  BORDERS,
  gauge::GAUGE_HEIGHT,
  grid::TAB_WIDTH,
  scroller::{ARROWS, SCROLLER},
  brushselector::{SELECTOR_HEIGHT, SELECTOR_WIDTH, RGB_BOX_HEIGHT, NB_BRUSHES},
  shortcuts::{VERTICAL_BAR, KEYBOARD_AREA_WIDTH, ACTIONS_AREA_WIDTH}
};

use crate::userinterface::views::parameters::render::{LARGER_COLOR_ROW,
  LARGER_TEXTURE_ROW, LARGER_PIXEL_ROW, LARGER_GRID_PARAM_ROW,
  LARGER_BRUSH_PARAM_ROW
};

/********************************* Edit view ********************************/

pub const MIN_BRUSH_AREA_WIDTH: u16 = BORDERS + SELECTOR_WIDTH;

pub const MIN_SHORTCUTS_AREA_WIDTH: u16 = SCROLLER + BORDERS + VERTICAL_BAR +
  KEYBOARD_AREA_WIDTH + ACTIONS_AREA_WIDTH;

const MIN_GRID_SIZE: u16 = 1;
const MIN_GRID_AREA_WIDTH: u16 = TAB_WIDTH + BORDERS * 2 + SCROLLER +
  MIN_GRID_SIZE;

pub const MIN_WIDTH_EDIT: u16 = MIN_BRUSH_AREA_WIDTH +
  MIN_SHORTCUTS_AREA_WIDTH + MIN_GRID_AREA_WIDTH;

pub const MIN_BRUSH_AREA_HEIGHT: u16 = SELECTOR_HEIGHT * NB_BRUSHES +
  RGB_BOX_HEIGHT + BORDERS;

pub const MIN_GRID_AREA_HEIGHT: u16 = BORDERS * 2 + SCROLLER + MIN_GRID_SIZE;

pub const MIN_SHORTCUTS_AREA_HEIGHT: u16 = 4 * (ARROWS + BORDERS) +
  2 * GAUGE_HEIGHT;

/****************************** Parameters view *****************************/

pub const MIN_HEIGHT_PARAM: u16 = (BORDERS + 1) * 6;
pub const MIN_WIDTH_TEXTURE: u16 = LARGER_TEXTURE_ROW + BORDERS;
pub const MIN_WIDTH_BRUSH_PARAM: u16 = LARGER_BRUSH_PARAM_ROW + BORDERS;
pub const MIN_WIDTH_GRID_PARAM: u16 = LARGER_GRID_PARAM_ROW + BORDERS;
pub const MIN_WIDTH_COLOR: u16 = LARGER_COLOR_ROW + BORDERS;
pub const MIN_WIDTH_PIXEL: u16 = LARGER_PIXEL_ROW + BORDERS;

/********************************* Exit view ********************************/

pub const MIN_WIDTH_EXIT: u16 = exit::render::WIN_WIDTH;
pub const MIN_HEIGHT_EXIT: u16 = exit::render::WIN_HEIGHT;

/**************************** Set Brush Color view **************************/

pub const MIN_WIDTH_SETBRUSHCOLOR: u16 = setbrushcolor::render::WIN_WIDTH;
pub const MIN_HEIGHT_SETBRUSHCOLOR: u16 = setbrushcolor::render::WIN_HEIGHT;

/************************* Set Generation Number view ***********************/

pub const MIN_WIDTH_SETGENERATIONNUMBER: u16 =
  setgenerationnumber::render::WIN_WIDTH;
pub const MIN_HEIGHT_SETGENERATIONNUMBER: u16 =
  setgenerationnumber::render::WIN_HEIGHT;

/************************** Set Texture Format view *************************/

pub const MIN_WIDTH_SETTEXTUREFORMAT: u16 =
  settextureformat::render::WIN_WIDTH;
pub const MIN_HEIGHT_SETTEXTUREFORMAT: u16 =
  settextureformat::render::WIN_HEIGHT;

/*************************** Generation Error view **************************/

pub const MIN_WIDTH_GENERATIONERROR: u16 = generationerror::render::WIN_WIDTH;
pub const MIN_HEIGHT_GENERATIONERROR: u16 =
  generationerror::render::WIN_HEIGHT;

/******************************* Grid Name view *****************************/

pub const MIN_WIDTH_GRIDNAME: u16 = gridname::render::WIN_WIDTH;
pub const MIN_HEIGHT_GRIDNAME: u16 = gridname::render::WIN_HEIGHT;

/************************** Grid Number Error view **************************/

pub const MIN_WIDTH_GRIDNUMBERERROR: u16 = gridnumbererror::render::WIN_WIDTH;
pub const MIN_HEIGHT_GRIDNUMBERERROR: u16 =
  gridnumbererror::render::WIN_HEIGHT;

/***************************** Log Overflow view ****************************/

pub const MIN_WIDTH_LOGOVERFLOW: u16 = logoverflow::render::WIN_WIDTH;
pub const MIN_HEIGHT_LOGOVERFLOW: u16 = logoverflow::render::WIN_HEIGHT;

/*******************************- Shutdown View *****************************/

pub const MIN_WIDTH_SHUTDOWN: u16 = shutdown::render::WIN_WIDTH;
pub const MIN_HEIGHT_SHUTDOWN: u16 = shutdown::render::MAX_WIN_HEIGHT;

/*************************** Swith Grid Order View **************************/

pub const MIN_WIDTH_SWITCHGRIDSORDER: u16 =
  switchgridsorder::render::WIN_WIDTH;
pub const MIN_HEIGHT_SWITCHGRIDSORDER: u16 =
  switchgridsorder::render::WIN_HEIGHT;

/*************************** Swith Grid Order View **************************/

pub const MIN_WIDTH_SWITCHGRIDSERROR: u16 =
  switchgridserror::render::WIN_WIDTH;
pub const MIN_HEIGHT_SWITCHGRIDSERROR: u16 =
  switchgridserror::render::WIN_HEIGHT;

/************************** Unavailable Thread view *************************/

pub const MIN_WIDTH_UNAVAILABLETHREAD: u16 =
  unavailablethread::render::WIN_WIDTH;
pub const MIN_HEIGHT_UNAVAILABLETHREAD: u16 =
  unavailablethread::render::WIN_HEIGHT;

/****************************************************************************/

pub const MIN_WIDTH: u16 = 60;
pub const MIN_HEIGHT: u16 = 30;

#[derive(Clone, Copy)]
pub enum View {
  Edit,
  Parameters,
}

pub fn ratio<T>(partial: T, total: T) -> f64
  where T: PrimInt,
{
  num::cast::<T, f64>(partial).unwrap() / num::cast::<T, f64>(total).unwrap()
}
