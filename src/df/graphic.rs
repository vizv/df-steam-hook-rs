use super::common::Coord;

const SCREENX_OFFSET: usize = 0x84;

pub fn deref_coordinate(addr: usize) -> Coord<i32> {
  Coord::at(addr + SCREENX_OFFSET)
}
