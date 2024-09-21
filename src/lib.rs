#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![feature(vec_into_raw_parts)]

mod config;
mod constants;
mod df;
mod encodings;
mod font;
mod global;
mod hooks;
mod markup;
mod offsets;
mod screen;
mod translator;
mod utils;
mod watchdog;

use crate::config::CONFIG;

#[static_init::constructor]
#[no_mangle]
extern "C" fn attach() {
  std::env::set_var("RUST_BACKTRACE", "1");
  // simple_logging::log_to_file(&CONFIG.settings.log_file, utils::log_level(CONFIG.settings.log_level)).unwrap();
  simple_logging::log_to_file(&CONFIG.settings.log_file, CONFIG.settings.log_level).unwrap();

  log::info!("dfint 版本: {}", CONFIG.version);
  log::info!("程序校验和: 0x{:x}", CONFIG.checksum);
  log::info!("字体文件: {:?}", CONFIG.settings.font_file);

  match unsafe { hooks::attach_all() } {
    Ok(_) => log::debug!("汉化已启用"),
    Err(err) => {
      log::error!("unable to attach hooks, {:?}", err);
      utils::message_box("dfint 错误", "无法启用汉化", utils::MessageIconType::Error);
      return;
    }
  };
  watchdog::install();
}

#[static_init::destructor]
#[no_mangle]
extern "C" fn detach() {
  unsafe {
    watchdog::uninstall();
    let _ = hooks::disable_all();
    log::debug!("汉化已禁用");
  }
}

#[no_mangle]
extern "C" fn super_secret_dfint_sign() -> u8 {
  69
}
