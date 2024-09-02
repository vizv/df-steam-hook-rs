use std::cmp::max;

use crate::df;

use super::constants;

pub struct Cover {
  pub coord: df::common::Coord<i32>,
  pub dimension: df::common::Dimension<u32>,
}

impl Cover {
  pub fn new(sx: i32, sy: i32, ex: i32, ey: i32) -> Self {
    Self {
      coord: df::common::Coord {
        x: sx * constants::CANVAS_FONT_WIDTH,
        y: sy * constants::CANVAS_FONT_HEIGHT,
      },
      dimension: df::common::Dimension {
        x: (max(ex - sx + 1, 0) * constants::CANVAS_FONT_WIDTH) as u32,
        y: (max(ey - sy + 1, 0) * constants::CANVAS_FONT_HEIGHT) as u32,
      },
    }
  }
}
