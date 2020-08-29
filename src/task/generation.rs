extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

extern crate image;
use image::{Rgba, RgbaImage};

extern crate radix_fmt;
use radix_fmt::radix;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate std;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;

use crate::log;

/// Module for colors generation
pub mod palette;

use crate::task::Task;
use crate::utils::FullPixel;

const TRANSPARENT_PIXEL: Rgba<u8> = Rgba([0; 4]);

/// Task which builds a directory and generate textures inside
pub struct Generation {
  grids: HashMap<(u32, u32), (usize, FullPixel)>,
  grids_rows: u32,
  grids_cols: u32,
  nb_grids: usize,
  pixel_ratio: u32,
  border_ratio: u8,
  palette_generation: palette::PaletteGeneration,
  number_generations: u16,
  format: image::ImageFormat,
  directory_name: String,
  body_de_ratio: f64,
  body_dr_ratio: f64,
}

impl Task for Generation {

  /// Generates one texture called <i>current_generation</i>.png
  ///
  /// <i>current_generation</i> is an alphanumeric number
  fn run(&self, current_generation: u16) {
    let mut image = image::ImageBuffer::from_pixel(
      self.get_img_width(), self.get_img_height(), TRANSPARENT_PIXEL);
    let mut palette = Vec::<palette::Palette>::with_capacity(self.nb_grids);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(self.grids_rows * self.grids_cols).unwrap());
    let mut rng = thread_rng();
    for _ in 0..(self.grids_rows * self.grids_cols) {
      rd.push(rng.gen_range(0., 1.));
    }
    for _ in 0..self.nb_grids {
      palette.push(self.palette_generation.run(self.border_ratio));
    }
    for row in 0..self.grids_rows {
      for col in 0..self.grids_cols {
        self.fix_pixel(&mut image, &palette, (row, col), &rd);
      }
    }
    image.save_with_format(format!("{}/{:#}.{}", self.directory_name,
      radix(current_generation, 36), self.format.extensions_str()[0]),
      self.format).unwrap();
  }

  fn get_nb_subtasks(&self) -> u16 {
    self.number_generations
  }
}

impl Generation {

  /// Return an error if the created directory name is already taken
  pub fn new(log: &log::Log) -> std::io::Result<Generation> {
    let now = Utc::now();
    let dir = format!("liriodendron_generation_{}-{}-{}_{}:{}:{}.{}",
      now.day(), now.month(), now.year(), now.hour(), now.minute(),
      now.second(), now.nanosecond());
    fs::create_dir(&dir)?;
    Ok(Generation {
      grids: log.grids_getgrids(),
      grids_rows: log.grids_getwidth(),
      grids_cols: log.grids_getheight(),
      nb_grids: log.grids_getnb(),
      pixel_ratio: log.texturessettings_getpixelratio(),
      border_ratio: log.texturessettings_getborderratio(),
      palette_generation: log.texturessettings_getpalette(),
      number_generations: log.texturessettings_getgenerationnumber(),
      format: log.texturessettings_getformat(),
      directory_name: dir,
      body_de_ratio: log.brush_getbodyderatio(),
      body_dr_ratio: log.brush_getbodydrratio(),
    })
  }

  fn get_img_width(&self) -> u32 {
    self.grids_rows * self.pixel_ratio
  }

  fn get_img_height(&self) -> u32 {
    self.grids_cols * self.pixel_ratio
  }

  fn fix_pixel(&self, image: &mut RgbaImage, palette: &Vec<palette::Palette>,
    (row, col): (u32, u32), rd: &Vec<f64>) {
      match self.grids.get(&(row, col)) {
        None => {
          self.fix_empty_pixel(image, palette, (row, col), rd);
        },
        Some((id, pixel)) => {
          match pixel {
            FullPixel::BodyEmpty => {
              if rd[usize::try_from(row * self.grids_cols + col).unwrap()]
                > self.body_de_ratio {
                  self.fix_full_pixel(image, palette.get(*id).unwrap(),
                    (row, col), FullPixel::Body);
              } else {
                self.fix_empty_pixel(image, palette, (row, col), rd);
              }
            },
            FullPixel::BodyBorder => {
              if rd[usize::try_from(row * self.grids_cols + col).unwrap()]
                > self.body_dr_ratio {
                  self.fix_full_pixel(image, palette.get(*id).unwrap(),
                    (row, col), FullPixel::Body);
              } else {
                self.fix_full_pixel(image, palette.get(*id).unwrap(),
                  (row, col), FullPixel::Border);
              }
            },
            _ => self.fix_full_pixel(image, palette.get(*id).unwrap(),
                  (row, col), *pixel),
          };
        },
      };
  }

  fn fix_empty_pixel(&self, image: &mut RgbaImage,
    palette: &Vec<palette::Palette>, (row, col): (u32, u32), rd: &Vec<f64>) {
      let mut high_priority_id: Option<usize> = None;
      if row > 0 {
        if let Some((neighbour_id, pixel)) = self.grids.get(&(row - 1, col)) {
          if self.is_body_pixel((row - 1, col), pixel, rd) {
            self.check_higher_priority_id(&mut high_priority_id, neighbour_id);
          }
        }
      }
      if row + 1 < self.grids_rows {
        if let Some((neighbour_id, pixel)) = self.grids.get(&(row + 1, col)) {
          if self.is_body_pixel((row + 1, col), pixel, rd) {
            self.check_higher_priority_id(&mut high_priority_id, neighbour_id);
          }
        }
      }
      if col > 0 {
        if let Some((neighbour_id, pixel)) = self.grids.get(&(row, col - 1)) {
          if self.is_body_pixel((row, col - 1), pixel, rd) {
            self.check_higher_priority_id(&mut high_priority_id, neighbour_id);
          }
        }
      }
      if col + 1 < self.grids_cols {
        if let Some((neighbour_id, pixel)) = self.grids.get(&(row, col + 1)) {
          if self.is_body_pixel((row, col + 1), pixel, rd) {
            self.check_higher_priority_id(&mut high_priority_id, neighbour_id);
          }
        }
      }

      if let Some(id) = high_priority_id {
        self.fix_full_pixel(
          image, palette.get(id).unwrap(), (row, col), FullPixel::Border);
      }
  }

  fn is_body_pixel(&self, (row, col): (u32, u32), pixel: &FullPixel,
    rd: &Vec<f64>) -> bool{
      match pixel {
        FullPixel::Body => return true,
        FullPixel::BodyEmpty
          if rd[usize::try_from(row * self.grids_cols + col).unwrap()] >
            self.body_de_ratio =>
              return true,
        FullPixel::BodyBorder
          if rd[usize::try_from(row * self.grids_cols + col).unwrap()] >
            self.body_dr_ratio =>
            return true,
        _ => return false,
      }
  }

  fn check_higher_priority_id(&self, id: &mut Option<usize>,
    neighbour_id: &usize) {
      match id {
        None => *id = Some(*neighbour_id),
        Some(lower_priority_id) if *lower_priority_id > *neighbour_id =>
          *id = Some(*neighbour_id),
        Some(_) => {},
      };
  }

  fn fix_full_pixel(&self, image: &mut RgbaImage, palette: &palette::Palette,
    (row, col): (u32, u32), pixel: FullPixel) {
      match pixel {
        FullPixel::Body => {
          self.colorize_image(image, (row, col), palette.get_body_color());
        },
        FullPixel::Border => self.colorize_image(image, (row, col),
          palette.get_border_color()),
        FullPixel::SpecificColor(r, g, b) => self.colorize_image(image,
            (row, col), Rgba([r, g, b, 255])),
        _ => {},
      }
  }

  fn colorize_image(&self, image: &mut RgbaImage, (x, y): (u32, u32),
    color: Rgba<u8>) {
      for offset_x in 0..self.pixel_ratio {
        for offset_y in 0..self.pixel_ratio {
          let pixel =
            image.get_pixel_mut(x * self.pixel_ratio + offset_x,
              y * self.pixel_ratio + offset_y);
          *pixel = color;
        }
      }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_colorizes_the_image_correctly() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let (a, b) = (3, 4);
    generation.colorize_image(&mut image, (a, b), Rgba([255; 4]));
    let mut image_is_correctly_colorized = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          image_is_correctly_colorized = image_is_correctly_colorized &&
            (*pixel == Rgba([255; 4]));
      } else {
        image_is_correctly_colorized = image_is_correctly_colorized &&
          (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(image_is_correctly_colorized);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_checks_grid_id_of_the_cell() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut id = None;
    let mut neighbour_id = 2;
    generation.check_higher_priority_id(&mut id, &neighbour_id);
    let mut first_check = false;
    if let Some(id) = id {
      if id == neighbour_id {
        first_check = true;
      }
    }
    neighbour_id = 1;
    generation.check_higher_priority_id(&mut id, &neighbour_id);
    let mut second_check = false;
    if let Some(id) = id {
      if id == neighbour_id {
        second_check = true;
      }
    }
    neighbour_id = 2;
    generation.check_higher_priority_id(&mut id, &neighbour_id);
    let mut third_check = false;
    if let Some(id) = id {
      if id == 1 {
        third_check = true;
      }
    }
    assert!(first_check && second_check && third_check);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_returns_if_it_is_a_body_pixel() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let rd = vec![0.6, 0.2, 0.7, 0.4, 0.0];
    assert!(generation.is_body_pixel((0, 0), &FullPixel::BodyEmpty, &rd) &&
      !generation.is_body_pixel((0, 1), &FullPixel::BodyEmpty, &rd) &&
      generation.is_body_pixel((0, 2), &FullPixel::BodyBorder, &rd) &&
      !generation.is_body_pixel((0, 3), &FullPixel::BodyBorder, &rd) &&
      generation.is_body_pixel((0, 4), &FullPixel::Body, &rd));
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_body_pixel() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let palette = generation.palette_generation.run(3);
    let (a, b) = (2, 8);
    generation.fix_full_pixel(&mut image, &palette, (a, b), FullPixel::Body);
    let mut fix_body_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_body_pixel =
            fix_body_pixel && (*pixel == palette.get_body_color());
      } else {
        fix_body_pixel = fix_body_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_body_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_border_pixel() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let palette = generation.palette_generation.run(3);
    let (a, b) = (7, 0);
    generation.fix_full_pixel(&mut image, &palette, (a, b), FullPixel::Border);
    let mut fix_border_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_border_pixel =
            fix_border_pixel && (*pixel == palette.get_border_color());
      } else {
        fix_border_pixel = fix_border_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_border_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_specific_color_pixel() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let palette = generation.palette_generation.run(3);
    let (a, b) = (7, 0);
    let (red, green, blue) = (100, 150, 30);
    let specific_pixel = FullPixel::SpecificColor(red, green, blue);
    let color_expected = Rgba([red, green, blue, 255]);
    generation.fix_full_pixel(&mut image, &palette, (a, b), specific_pixel);
    let mut fix_specific_color_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_specific_color_pixel =
            fix_specific_color_pixel && (*pixel == color_expected);
      } else {
        fix_specific_color_pixel =
          fix_specific_color_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_specific_color_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_does_not_fix_empty_pixel_with_empty_pixel_neighbours() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let mut rng = thread_rng();
    for _ in 0..(generation.grids_rows * generation.grids_cols) {
      rd.push(rng.gen_range(0., 1.));
    }
    let palette = vec![generation.palette_generation.run(3)];
    let (a, b) = (5, 1);
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (_, _, pixel) in image.enumerate_pixels() {
      fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_does_not_fix_empty_pixel_with_1_border_pixel_neighbour() {
    let log = log::Log::new(0, 0);
    let generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let mut rng = thread_rng();
    for _ in 0..(generation.grids_rows * generation.grids_cols) {
      rd.push(rng.gen_range(0., 1.));
    }
    let palette = vec![generation.palette_generation.run(3)];
    let (a, b) = (3, 6);
    generation.fix_full_pixel(
      &mut image, palette.get(0).unwrap(), (a, b + 1), FullPixel::Border);
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= (b + 1) * generation.pixel_ratio) &&
        (y < (b + 2) * generation.pixel_ratio) {
          fix_empty_pixel = fix_empty_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_empty_pixels_into_border_pixels_with_higher_priority_grid_with_several_grids() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let palette = vec![generation.palette_generation.run(3),
      generation.palette_generation.run(3),
      generation.palette_generation.run(3)];

    generation.grids.insert((0, 0), (0, FullPixel::Body));
    generation.grids.insert((2, 0), (1, FullPixel::Body));
    generation.grids.insert((1, 1), (1, FullPixel::Body));
    generation.grids.insert((0, 2), (1, FullPixel::Body));
    generation.grids.insert((2, 2), (2, FullPixel::Body));
    generation.fix_empty_pixel(&mut image, &palette, (1, 2), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (1, 2), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (1, 2), &rd);

    generation.fix_empty_pixel(&mut image, &palette, (2, 1), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (2, 1), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (2, 1), &rd);

    generation.fix_empty_pixel(&mut image, &palette, (1, 0), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (1, 0), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (1, 0), &rd);

    generation.fix_empty_pixel(&mut image, &palette, (0, 1), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (0, 1), &rd);
    generation.fix_empty_pixel(&mut image, &palette, (0, 1), &rd);

    let mut fix_empty_pixels = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x < generation.pixel_ratio) && (y >= generation.pixel_ratio) &&
        (y < 2 * generation.pixel_ratio) {
          fix_empty_pixels = fix_empty_pixels &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else if (x >= generation.pixel_ratio) &&
        (x < 2 * generation.pixel_ratio) && (y < generation.pixel_ratio) {
          fix_empty_pixels = fix_empty_pixels &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else if (x >= 2 * generation.pixel_ratio) &&
        (x < 3 * generation.pixel_ratio) &&
        (y >= generation.pixel_ratio) && (y < 2 * generation.pixel_ratio) {
          fix_empty_pixels = fix_empty_pixels &&
            (*pixel == palette.get(1).unwrap().get_border_color());
      } else if (x >= generation.pixel_ratio) &&
        (x < 2 * generation.pixel_ratio) && (y >= 3 * generation.pixel_ratio) &&
        (y < 2 * generation.pixel_ratio) {
          fix_empty_pixels = fix_empty_pixels &&
            (*pixel == palette.get(1).unwrap().get_border_color());
      }
    }
    assert!(fix_empty_pixels);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_empty_pixel_with_1_bodyborder_pixel_neighbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b + 1 {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b + 1), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_empty_pixel = fix_empty_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_does_not_fix_empty_pixel_with_1_bodyborder_pixel_neighbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_border = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b + 1 {
        rd.push(is_border);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b + 1), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (_, _, pixel) in image.enumerate_pixels() {
      fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_empty_pixel_with_1_bodyempty_pixel_neighbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b + 1 {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b + 1), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_empty_pixel =
            fix_empty_pixel &&
              (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_does_not_fix_empty_pixel_with_1_bodyempty_pixel_neighbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b + 1 {
        rd.push(is_empty);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b + 1), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_empty_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_empty_pixel = true;
    for (_, _, pixel) in image.enumerate_pixels() {
      fix_empty_pixel = fix_empty_pixel && (*pixel == TRANSPARENT_PIXEL);
    }
    assert!(fix_empty_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyborder_pixel_into_body_pixel() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_body_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyborder_pixel_into_border_pixel() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_border = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_border);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_body_pixel() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_body_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_empty_pixel_with_empty_pixels_neightbours() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (_x, _y, pixel) in image.enumerate_pixels() {
      fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_empty_pixel_surrounded_by_border_pixel_neightbours() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a + 1, b), (0, FullPixel::Border));
    generation.grids.insert((a - 1, b), (0, FullPixel::Border));
    generation.grids.insert((a, b + 1), (0, FullPixel::Border));
    generation.grids.insert((a, b - 1), (0, FullPixel::Border));
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a + 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b + 1), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b - 1), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= (a + 1) * generation.pixel_ratio) &&
        (x < (a + 2) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else if (x >= (a - 1) * generation.pixel_ratio) &&
        (x < a * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= (b + 1) * generation.pixel_ratio) &&
        (y < (b + 2) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= (b - 1) * generation.pixel_ratio) &&
        (y < b * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_border_pixel_with_1_body_pixel_neightbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    generation.grids.insert((a - 1, b), (0, FullPixel::Body));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= (a - 1) * generation.pixel_ratio) &&
        (x < a * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_body_color());
      } else if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_border_pixel_with_1_bodyempty_pixel_neightbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else if i == generation.grids_cols * (a - 1) + b {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    generation.grids.insert((a - 1, b), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= (a - 1) * generation.pixel_ratio) &&
        (x < a * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_body_color());
      } else if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_empty_pixel_with_1_bodyempty_pixel_neightbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else if i == generation.grids_cols * (a - 1) + b {
        rd.push(is_empty);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    generation.grids.insert((a - 1, b), (0, FullPixel::BodyEmpty));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (_x, _y, pixel) in image.enumerate_pixels() {
      fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_border_pixel_with_1_bodyborder_pixel_neightbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    let is_body = rng.gen_range(0.6, 1.);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else if i == generation.grids_cols * (a - 1) + b {
        rd.push(is_body);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    generation.grids.insert((a - 1, b), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= (a - 1) * generation.pixel_ratio) &&
        (x < a * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_body_color());
      } else if (x >= a * generation.pixel_ratio) &&
        (x < (a + 1) * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }

  #[test]
  fn it_fixes_bodyempty_pixel_into_empty_pixel_with_1_bodyborder_pixel_neightbour() {
    let log = log::Log::new(0, 0);
    let mut generation = Generation::new(&log).unwrap();
    let mut image = image::ImageBuffer::from_pixel(
      generation.get_img_width(), generation.get_img_height(), TRANSPARENT_PIXEL);
    let mut rd = Vec::<f64>::with_capacity(
      usize::try_from(generation.grids_rows * generation.grids_cols).unwrap());
    let (a, b) = (3, 6);
    let mut rng = thread_rng();
    let is_empty = rng.gen_range(0., 0.4);
    let is_border = rng.gen_range(0., 0.4);
    for i in 0..(generation.grids_rows * generation.grids_cols) {
      if i == generation.grids_cols * a + b {
        rd.push(is_empty);
      } else if i == generation.grids_cols * (a - 1) + b {
        rd.push(is_border);
      } else {
        rd.push(rng.gen_range(0., 1.));
      }
    }
    generation.grids.insert((a, b), (0, FullPixel::BodyEmpty));
    generation.grids.insert((a - 1, b), (0, FullPixel::BodyBorder));
    let palette = vec![generation.palette_generation.run(3)];
    generation.fix_pixel(&mut image, &palette, (a - 1, b), &rd);
    generation.fix_pixel(&mut image, &palette, (a, b), &rd);
    let mut fix_pixel = true;
    for (x, y, pixel) in image.enumerate_pixels() {
      if (x >= (a - 1) * generation.pixel_ratio) &&
        (x < a * generation.pixel_ratio) &&
        (y >= b * generation.pixel_ratio) &&
        (y < (b + 1) * generation.pixel_ratio) {
          fix_pixel = fix_pixel &&
            (*pixel == palette.get(0).unwrap().get_border_color());
      } else {
        fix_pixel = fix_pixel && (*pixel == TRANSPARENT_PIXEL);
      }
    }
    assert!(fix_pixel);
    fs::remove_dir_all(generation.directory_name).unwrap();
  }
}
