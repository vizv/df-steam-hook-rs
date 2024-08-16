use anyhow::Result;
use retour::static_detour;

use crate::config::CONFIG;
use crate::cxxstring::CxxString;
// use crate::cxxstring::CxxString;
// use crate::dictionary::DICTIONARY;
use crate::utils;
use crate::screen::{SCREEN, SCREEN_TOP};

use r#macro::hook;

#[cfg(target_os = "linux")]
#[static_init::dynamic]
static ENABLER: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => {
      utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().enabler.as_ref().unwrap()[1]) as usize
    }
    false => 0 as usize,
  }
};

#[cfg(target_os = "linux")]
#[static_init::dynamic]
static GPS: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().gps.as_ref().unwrap()[1]) as usize,
    false => 0 as usize,
  }
};

pub unsafe fn attach_all() -> Result<()> {
  attach_addst()?;
  attach_addst_top()?;
  attach_addst_flag()?;
  attach_erasescreen()?;
  attach_update_tile()?;
  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_addst()?;
  enable_addst_top()?;
  enable_addst_flag()?;
  enable_erasescreen()?;
  enable_update_tile()?;
  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_addst()?;
  disable_addst_top()?;
  disable_addst_flag()?;
  disable_erasescreen()?;
  disable_update_tile()?;
  Ok(())
}

fn deref<T>(addr: usize) -> T {
  unsafe { (addr as *const T).read() }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst(gps: usize, src: *const u8, justify: u8, space: u32) {
  let text = unsafe { String::from_utf8_lossy(CxxString::from_ptr(src).to_bytes_without_nul()).into_owned() };
  let x = deref::<i32>(gps + 0x84); // gps.screenx
  let y = deref::<i32>(gps + 0x88); // gps.screeny
  SCREEN.write().add(x, y, text);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_top(gps: usize, src: *const u8, justify: u8, space: u32) {}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_flag(gps: usize, src: *const u8, justify: u8, space: u32) {}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen(gps: usize) {
  unsafe { original!(gps) };
  SCREEN.write().clear()
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_tile(render: usize, x: u32, y: u32) {
  unsafe { original!(render, x, y) };
  SCREEN.write().render()
}
