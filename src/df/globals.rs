use crate::offsets;
use crate::utils::OFFSETS;

#[static_init::dynamic]
pub static ENABLER: usize = OFFSETS.get(offsets::GLOBALS.get("enabler").unwrap());

#[static_init::dynamic]
pub static GAME: usize = OFFSETS.get(offsets::GLOBALS.get("game").unwrap());

#[static_init::dynamic]
pub static GPS: usize = OFFSETS.get(offsets::GLOBALS.get("gps").unwrap());

#[static_init::dynamic]
pub static GVIEW: usize = OFFSETS.get(offsets::GLOBALS.get("gview").unwrap());

#[cfg(target_os = "linux")]
pub fn get_key_display(str_ptr: usize, enabler: usize, binding: i32) {
  let get_key_display_impl: fn(usize, usize, i32) =
    unsafe { std::mem::transmute(OFFSETS.get(offsets::FUNCTIONS.get("get_key_display").unwrap())) };
  get_key_display_impl(str_ptr, enabler, binding);
}

#[cfg(target_os = "windows")]
pub fn get_key_display(str_ptr: usize, enabler: usize, binding: i32) {
  let get_key_display_impl: fn(usize, usize, i32) =
    unsafe { std::mem::transmute(OFFSETS.get(offsets::FUNCTIONS.get("get_key_display").unwrap())) };
  get_key_display_impl(enabler, str_ptr, binding);
}
