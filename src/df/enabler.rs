use super::{offsets, utils};

pub fn deref_curses_surface(addr: usize, code: u8) -> usize {
  let texture_base: usize = utils::deref(addr + offsets::ENABLER_TEXTURES);
  utils::deref(texture_base + (code as usize * 8))
}
