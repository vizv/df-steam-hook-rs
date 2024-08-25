use super::{common, enums, offsets, utils};

pub fn deref_coord(addr: usize) -> common::Coord<i32> {
  common::Coord::at(addr + offsets::GRAPHIC_SCREENX)
}

pub fn deref_dim(addr: usize) -> common::Dimension<i32> {
  common::Dimension::at(addr + offsets::GRAPHIC_DIMX)
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
  } = utils::deref(addr + offsets::GRAPHIC_SCREENF);

  if use_old_16_colors {
    let fg = (screenf + if screenbright { 8 } else { 0 }) as usize;
    let uccolor_base = addr + offsets::GRAPHIC_SCREENF + offsets::GRAPHIC_SCREENF_UCCOLOR;
    common::Color::at(uccolor_base + fg * 3)
  } else {
    common::Color::rgb(r, g, b)
  }
}

pub fn get_uccolor(addr: usize, color: enums::CursesColor) -> common::Color {
  common::Color::at(addr + offsets::GRAPHIC_UCCOLOR + 3 * color as usize)
}
