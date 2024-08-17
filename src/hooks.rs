use anyhow::Result;
use retour::static_detour;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use crate::screen::{SCREEN, SCREEN_TOP};
use crate::{raw, utils};

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
  attach_resize()?;
  attach_update_all()?;
  attach_update_tile()?;
  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_addst()?;
  enable_addst_top()?;
  enable_addst_flag()?;
  enable_erasescreen()?;
  enable_resize()?;
  enable_update_all()?;
  enable_update_tile()?;
  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_addst()?;
  disable_addst_top()?;
  disable_addst_flag()?;
  disable_erasescreen()?;
  disable_resize()?;
  disable_update_all()?;
  disable_update_tile()?;
  Ok(())
}

fn gps_get_screen_coord(addr: usize) -> (i32, i32) {
  (
    raw::deref::<i32>(addr + 0x84), // gps.screenx
    raw::deref::<i32>(addr + 0x88), // gps.screeny
  )
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst(gps: usize, str: usize, just: u8, space: i32) {
  let mut content = raw::deref_string(str);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }

  let (x, y) = gps_get_screen_coord(gps);
  SCREEN.write().add(x, y, content, 0);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_top(gps: usize, str: usize, just: u8, space: i32) {
  let content = raw::deref_string(str);
  let (x, y) = gps_get_screen_coord(gps);
  SCREEN_TOP.write().add(x, y, content, 0);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_flag(gps: usize, str: usize, just: u8, space: i32, sflag: u32) {
  let mut content = raw::deref_string(str);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }

  let (x, y) = gps_get_screen_coord(gps);
  SCREEN.write().add(x, y, content, sflag);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen(gps: usize) {
  unsafe { original!(gps) };
  SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn resize(renderer: usize, w: u32, h: u32) {
  unsafe { original!(renderer, w, h) };
  SCREEN.write().resize(w, h);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };
  SCREEN_TOP.write().render(renderer);
}

struct Dimension {
  x: i32,
  y: i32,
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + 0xa00);

  // hack to render text after the last update_tile in update_all
  // TODO: consider re-write update_all function completely according to g_src
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  SCREEN.write().render(renderer);
}
