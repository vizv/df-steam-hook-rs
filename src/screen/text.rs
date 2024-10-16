use crate::df;

use super::{constants, data};

#[derive(Debug)]
pub struct Text {
  pub coord: df::common::Coord<i32>,
  pub data: data::Data,
  pub render: bool,
  pub horizontal_shift: i32,
}

impl Text {
  pub fn new((content, horizontal_shift): (&str, i32)) -> Self {
    Self {
      coord: Default::default(),
      data: data::Data::new(content.to_owned()),
      horizontal_shift,
      render: true,
    }
  }

  pub fn by_coord(mut self, coord: df::common::Coord<i32>) -> Self {
    self.coord = coord;
    self
  }

  pub fn by_gps(self, gps: usize) -> Self {
    self.color_by_gps(gps).coord_by_gps(gps)
  }

  pub fn color_by_gps(mut self, gps: usize) -> Self {
    let colors = df::gps::read_colors(gps);
    self.data = self.data.with_fg_color(colors.0);
    self.data = self.data.with_bg_color(colors.1);
    self
  }

  pub fn coord_by_gps(self, gps: usize) -> Self {
    let mut coord = df::gps::read_coord(gps);
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

  pub fn with_fg_color(mut self, color: df::common::Color) -> Self {
    self.data = self.data.with_fg_color(color);
    self
  }
}
