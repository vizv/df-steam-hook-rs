use super::utils;

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
