// TODO: move to df::globals
use crate::config::CONFIG;

use crate::utils;

#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static ENABLER: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => {
      utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().enabler.as_ref().unwrap()[1]) as usize
    }
    false => 0 as usize,
  }
};

#[cfg(target_os = "windows")]
#[static_init::dynamic]
pub static ENABLER: usize = {
  match CONFIG.offset.is_some() {
    true => utils::address(CONFIG.offset.as_ref().unwrap().enabler.unwrap()),
    false => 0 as usize,
  }
};

#[static_init::dynamic]
pub static GAME: usize = {
  match CONFIG.offset.is_some() {
    true => utils::address(CONFIG.offset.as_ref().unwrap().game.unwrap()),
    false => 0 as usize,
  }
};

#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static GPS: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().gps.as_ref().unwrap()[1]) as usize,
    false => 0 as usize,
  }
};

#[cfg(target_os = "windows")]
#[static_init::dynamic]
pub static GPS: usize = {
  match CONFIG.offset.is_some() {
    true => utils::address(CONFIG.offset.as_ref().unwrap().gps.unwrap()),
    false => 0 as usize,
  }
};

#[allow(non_upper_case_globals)]
#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static get_key_display: fn(usize, usize, i32) = unsafe {
  match CONFIG.symbol.is_some() {
    true => {
      let symbol = CONFIG.symbol.as_ref().unwrap().get_key_display.as_ref().unwrap();
      utils::symbol_handle::<fn(usize, usize, i32)>(&symbol[0], &symbol[1])
    }
    false => move |_, _, _| {},
  }
};

// #[allow(non_upper_case_globals)]
// #[cfg(target_os = "windows")]
// #[static_init::dynamic]
// pub static get_key_display: fn(usize, usize, i32) = unsafe {
//   match CONFIG.offset.is_some() {
//     true => *(utils::address(CONFIG.offset.as_ref().unwrap().get_key_display.unwrap()) as *const fn(usize, usize, i32)),
//     false => |_, _, _| {},
//   }
// };

#[allow(non_upper_case_globals)]
#[cfg(target_os = "windows")]
pub fn get_key_display(str_ptr: usize, enabler: usize, binding: i32) {
  log::warn!("??? proxy call to get_key_display(0x{str_ptr:x}, 0x{enabler:x}, {binding})");
  // get_key_display_fn(str_ptr, enabler, binding);
}
