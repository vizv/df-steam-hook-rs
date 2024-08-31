use super::{common, utils};

const SCREENX_OFFSET: usize = 0x84;
const SCREENF_OFFSET: usize = 0x8c; // TODO: check this on Windows
const UCCOLOR_SCREENF_OFFSET: usize = 0xcc; // TODO: check this on Windows

pub fn deref_coord(addr: usize) -> common::Coord<i32> {
  common::Coord::at(addr + SCREENX_OFFSET)
}

#[derive(Debug)]
#[repr(C)]
pub struct ColorInfo {
  pub screenf: u8,
  pub screenb: u8,
  pub screenbright: bool,
  pub use_old_16_colors: bool,
  pub screen_color_r: u8,
  pub screen_color_g: u8,
  pub screen_color_b: u8,
}

pub fn deref_color(addr: usize) -> common::Color {
  let ColorInfo {
    use_old_16_colors,
    screenf,
    screenbright,
    screen_color_r: r,
    screen_color_g: g,
    screen_color_b: b,
    ..
  } = utils::deref(addr + SCREENF_OFFSET);

  if use_old_16_colors {
    let fg = (screenf + if screenbright { 8 } else { 0 }) as usize;
    let uccolor_base = addr + SCREENF_OFFSET + UCCOLOR_SCREENF_OFFSET;
    common::Color::at(uccolor_base + fg * 3)
  } else {
    common::Color::rgb(r, g, b)
  }
}
