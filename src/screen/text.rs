use crate::df;

use super::{constants, data};

pub struct Text {
  pub coord: df::common::Coord<i32>,
  pub data: data::Data,
  pub render: bool,
}

impl Text {
  pub fn new(content: String) -> Self {
    Self {
      coord: Default::default(),
      data: data::Data::new(content),
      render: true,
    }
  }

  pub fn by_coord(mut self, coord: df::common::Coord<i32>) -> Self {
    self.coord = coord;
    self
  }

  pub fn by_graphic(self, gps: usize) -> Self {
    self.color_by_graphic(gps).coord_by_graphic(gps)
  }

  pub fn color_by_graphic(mut self, gps: usize) -> Self {
    let color = df::graphic::deref_color(gps);
    self.data = self.data.with_color(color);
    self
  }

  pub fn coord_by_graphic(self, gps: usize) -> Self {
    let mut coord = df::graphic::deref_coord(gps);
    coord.x *= constants::CANVAS_FONT_WIDTH;
    coord.y *= constants::CANVAS_FONT_HEIGHT;
    self.by_coord(coord)
  }

  pub fn with_offset(mut self, offset_x: i32, offset_y: i32) -> Self {
    self.coord.x += offset_x;
    self.coord.y += offset_y;
    self
  }

  pub fn with_sflag(mut self, sflag: u32) -> Self {
    let flag = df::flags::ScreenTexPosFlag::from_bits_retain(sflag);

    if flag.contains(df::flags::ScreenTexPosFlag::TOP_OF_TEXT) {
      self.render = false;
    }

    if flag.contains(df::flags::ScreenTexPosFlag::BOTTOM_OF_TEXT) {
      self.coord.y -= constants::CANVAS_FONT_HEIGHT / 2
    }

    self
  }

  pub fn with_color(mut self, color: df::common::Color) -> Self {
    self.data = self.data.with_color(color);
    self
  }
}
