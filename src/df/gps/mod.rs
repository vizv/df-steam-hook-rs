#[cfg(target_os = "windows")]
use std::ptr;

use crate::offsets;

use super::{common, enums};

pub fn read_coord(addr: usize) -> common::Coord<i32> {
  common::Coord::read(addr + offsets::FIELDS.get("gps.screenx").unwrap())
}

#[cfg(target_os = "windows")]
pub fn set_coord(addr: usize, coord: &common::Coord<i32>) {
  let p: *mut common::Coord<i32> = (addr + offsets::FIELDS.get("gps.screenx").unwrap()) as *mut common::Coord<i32>;
  unsafe { ptr::copy_nonoverlapping(coord, p, 1) };
}

pub fn borrow_dim(addr: usize) -> &'static common::Dimension<i32> {
  common::Dimension::borrow(addr + offsets::FIELDS.get("gps.dimx").unwrap())
}

#[derive(Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ColorInfo {
  pub screenf: u8,
  pub screenb: u8,
  pub screenbright: bool,
  pub use_old_16_colors: bool,
  pub screen_color_r: u8,
  pub screen_color_g: u8,
  pub screen_color_b: u8,
  pub screen_color_br: u8,
  pub screen_color_bg: u8,
  pub screen_color_bb: u8,
}

pub fn borrow_color_info(addr: usize) -> &'static ColorInfo {
  raw::as_ref(addr + offsets::FIELDS.get("gps.screenf").unwrap())
}

#[cfg(target_os = "windows")]
pub fn read_color_info(addr: usize) -> ColorInfo {
  raw::read(addr + offsets::FIELDS.get("gps.screenf").unwrap())
}

#[cfg(target_os = "windows")]
pub fn set_color_info(addr: usize, color_info: &ColorInfo) {
  let p = (addr + offsets::FIELDS.get("gps.screenf").unwrap()) as *mut ColorInfo;
  unsafe { ptr::copy_nonoverlapping(color_info, p, 1) };
}

pub fn read_colors(addr: usize) -> (common::Color, common::Color) {
  let info = borrow_color_info(addr);
  let ColorInfo {
    use_old_16_colors,
    screenf,
    screenb,
    screenbright,
    screen_color_r: r,
    screen_color_g: g,
    screen_color_b: b,
    screen_color_br: br,
    screen_color_bg: bg,
    screen_color_bb: bb,
  } = info;
  // FIXME: use screentexpos_lower to determine the transparent background

  let colors = if *use_old_16_colors {
    let fg = (screenf + if *screenbright { 8 } else { 0 }) as usize;
    let bg = (screenb + if *screenbright { 8 } else { 0 }) as usize;
    let uccolor_base = addr + offsets::FIELDS.get("gps.uccolor").unwrap();
    (
      common::Color::read(uccolor_base + fg * 3),
      common::Color::read(uccolor_base + bg * 3),
    )
  } else {
    (common::Color::rgb(*r, *g, *b), common::Color::rgb(*br, *bg, *bb))
  };

  colors
}

pub fn get_uccolor(addr: usize, color: enums::CursesColor) -> common::Color {
  common::Color::read(addr + offsets::FIELDS.get("gps.uccolor").unwrap() + 3 * color as usize)
}

pub fn top_in_use(addr: usize) -> bool {
  raw::read(addr + offsets::FIELDS.get("gps.top_in_use").unwrap())
}
