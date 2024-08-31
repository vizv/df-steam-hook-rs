use super::utils;

// TODO: change to Coord
#[derive(Debug, Default)]
pub struct Coordinate<T> {
  pub x: T,
  pub y: T,
}

impl<T> Coordinate<T> {
  pub fn at(addr: usize) -> Self {
    utils::deref(addr)
  }
}
