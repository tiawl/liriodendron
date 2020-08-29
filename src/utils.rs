//! Shared structs and enums between modules

/// Possible content for a filled cell
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FullPixel {
  Body,
  Border,
  BodyBorder,
  BodyEmpty,
  SpecificColor(u8, u8, u8),
}
