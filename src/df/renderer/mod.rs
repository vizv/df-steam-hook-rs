use sdl2::sys as sdl;

use crate::offsets;

#[derive(Debug)]
#[repr(C)]
pub struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
}

pub fn borrow_screen_info(addr: usize) -> &'static ScreenInfo {
  raw::as_ref(addr + offsets::FIELDS.get("renderer.dispx_z").unwrap())
}

pub fn read_sdl_renderer(addr: usize) -> *mut sdl::SDL_Renderer {
  raw::read(addr + offsets::FIELDS.get("renderer.sdl_renderer").unwrap())
}
