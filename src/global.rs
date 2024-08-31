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

// TODO: use config
#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static GAME: usize = 0x21d4380;

// TODO: from config
#[allow(non_upper_case_globals)]
#[cfg(target_os = "linux")]
#[static_init::dynamic]
pub static get_key_display: fn(usize, usize, i32) =
  unsafe { utils::symbol_handle("libg_src_lib.so", "_ZN15enabler_inputst13GetKeyDisplayB5cxx11Ei") };
