use crate::offsets;

pub fn get_curses_surface(addr: usize, code: u8) -> usize {
  let texture_base: usize = raw::read(addr + offsets::FIELDS.get("enabler.textures").unwrap());
  raw::read(texture_base + (code as usize * 8))
}
