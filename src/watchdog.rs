extern crate device_query;
use device_query::{DeviceState, Keycode};

use crate::hooks;

#[static_init::dynamic]
static mut KILL: bool = false;

pub fn install() {
  std::thread::spawn(move || {
    let state = DeviceState::new();
    let mut hook_enabled: bool = true;

    // TODO: only disable hook after render loop
    while !*KILL.read() {
      let keys = state.query_keymap();
      if keys.contains(&Keycode::F2) && keys.contains(&Keycode::LControl) {
        if hook_enabled {
          hook_enabled = false;
          log::info!("汉化已禁用");
          let _ = unsafe { hooks::disable_all() };
        } else {
          hook_enabled = true;
          log::info!("汉化已启用");
          let _ = unsafe { hooks::enable_all() };
        }
      };
      std::thread::sleep(std::time::Duration::from_millis(150));
    }
  });
}

pub fn uninstall() {
  *KILL.write() = true;
}
