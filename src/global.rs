// TODO: move to df::globals

use crate::config::CONFIG;
use crate::offsets::OFFSETS;

#[static_init::dynamic]
pub static ENABLER: usize = OFFSETS.get(CONFIG.offsets.globals.enabler.0, CONFIG.offsets.globals.enabler.1);

#[static_init::dynamic]
pub static GAME: usize = OFFSETS.get(CONFIG.offsets.globals.game.0, CONFIG.offsets.globals.game.1);

#[static_init::dynamic]
pub static GPS: usize = OFFSETS.get(CONFIG.offsets.globals.gps.0, CONFIG.offsets.globals.gps.1);

#[static_init::dynamic]
pub static GVIEW: usize = OFFSETS.get(CONFIG.offsets.globals.gview.0, CONFIG.offsets.globals.gview.1);

#[cfg(target_os = "linux")]
pub fn get_key_display(str_ptr: usize, enabler: usize, binding: i32) {
  let get_key_display_impl: fn(usize, usize, i32) = unsafe {
    std::mem::transmute(OFFSETS.get(
      CONFIG.offsets.functions.get_key_display.0,
      CONFIG.offsets.functions.get_key_display.1,
    ))
  };
  get_key_display_impl(str_ptr, enabler, binding);
}

#[cfg(target_os = "windows")]
pub fn get_key_display(str_ptr: usize, enabler: usize, binding: i32) {
  let get_key_display_impl: fn(usize, usize, i32) = unsafe {
    std::mem::transmute(OFFSETS.get(
      CONFIG.offsets.functions.get_key_display.0,
      CONFIG.offsets.functions.get_key_display.1,
    ))
  };
  get_key_display_impl(enabler, str_ptr, binding);
}
