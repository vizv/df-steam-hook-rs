use super::common::Coordinate;

const SCREENX_OFFSET: usize = 0x84;

pub fn deref_coordinate(addr: usize) -> Coordinate<i32> {
  Coordinate::at(addr + SCREENX_OFFSET)
}
