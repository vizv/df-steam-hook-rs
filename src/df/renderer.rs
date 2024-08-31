use super::{offsets, utils};

#[derive(Debug)]
#[repr(C)]
pub struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
}

pub fn deref_screen_info(addr: usize) -> ScreenInfo {
  utils::deref(addr + offsets::RENDERER_DISPX_Z)
}
