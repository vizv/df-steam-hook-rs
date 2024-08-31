use super::utils;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Coord<T> {
  pub x: T,
  pub y: T,
}

impl<T> Coord<T> {
  pub fn at(addr: usize) -> Self {
    utils::deref(addr)
  }
}

pub type Dimension<T> = Coord<T>;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub fn at(addr: usize) -> Self {
    utils::deref(addr)
  }

  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}
