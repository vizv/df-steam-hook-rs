use super::utils;

pub struct Coordinate<T> {
  pub x: T,
  pub y: T,
}

impl<T> Coordinate<T> {
  pub fn at(addr: usize) -> Self {
    utils::deref(addr)
  }
}
