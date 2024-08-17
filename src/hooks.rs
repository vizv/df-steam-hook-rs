use anyhow::Result;
use retour::static_detour;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
// use crate::dictionary::DICTIONARY;
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
  // attach_erasescreen_clip()?;
  // attach_erasescreen_rect()?;
  // attach_add_lower_tile()?;
  // attach_add_top_lower_tile()?;
  attach_resize()?;
  attach_update_all()?;
  attach_update_tile()?;
  attach_update_anchor_tile()?;
  attach_update_top_tile()?;
  attach_update_top_anchor_tile()?;
  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_addst()?;
  enable_addst_top()?;
  enable_addst_flag()?;
  enable_erasescreen()?;
  enable_erasescreen_clip()?;
  enable_erasescreen_rect()?;
  enable_add_lower_tile()?;
  enable_add_top_lower_tile()?;
  enable_resize()?;
  enable_update_all()?;
  enable_update_tile()?;
  enable_update_anchor_tile()?;
  enable_update_top_tile()?;
  enable_update_top_anchor_tile()?;
  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_addst()?;
  disable_addst_top()?;
  disable_addst_flag()?;
  disable_erasescreen()?;
  disable_erasescreen_clip()?;
  disable_erasescreen_rect()?;
  disable_add_lower_tile()?;
  disable_add_top_lower_tile()?;
  disable_resize()?;
  disable_update_all()?;
  disable_update_tile()?;
  disable_update_anchor_tile()?;
  disable_update_top_tile()?;
  disable_update_top_anchor_tile()?;
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
  let (x, y) = gps_get_screen_coord(gps);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }
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
  let content = raw::deref_string(str);
  let (x, y) = gps_get_screen_coord(gps);
  SCREEN.write().add(x, y, content, sflag);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen(gps: usize) {
  unsafe { original!(gps) };
  SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen_clip(gps: usize) {
  log::debug!("erasescreen_clip");
  unsafe { original!(gps) };
  // SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen_rect(gps: usize, x1: i32, x2: i32, y1: i32, y2: i32) {
  log::debug!("erasescreen_rect: ({},{}) -> ({},{})", x1, y1, x2, y2);
  unsafe { original!(gps, x1, x2, y1, y2) };
  // SCREEN.write().clear();
}

struct ClipCoords {
  x1: i32,
  x2: i32,
  y1: i32,
  y2: i32,
}
// fn gps_get_screen_coord(addr: usize) -> (i32, i32) {
//   (
//     raw::deref::<i32>(addr + 0x84), // gps.screenx
//     raw::deref::<i32>(addr + 0x88), // gps.screeny
//   )
// }

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn add_lower_tile(gps: usize, texpos: i64) {
  // log::debug!("add_lower_tile: {}", texpos);
  unsafe { original!(gps, texpos) };
  let clip_coords = raw::deref::<ClipCoords>(gps + 0x280);
  SCREEN.write().clear_rect(clip_coords.x1, clip_coords.x2, clip_coords.y1, clip_coords.y2);
  // SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn add_top_lower_tile(gps: usize, texpos: i64) {
  // log::debug!("add_top_lower_tile: {}", texpos);
  unsafe { original!(gps, texpos) };
  let clip_coords = raw::deref::<ClipCoords>(gps + 0x280);
  SCREEN.write().clear_rect(clip_coords.x1, clip_coords.x2, clip_coords.y1, clip_coords.y2);
  // SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn resize(renderer: usize, w: u32, h: u32) {
  unsafe { original!(renderer, w, h) };
  // let gps = GPS.to_owned();
  SCREEN.write().resize(w, h);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };
  log::debug!("update_all: 0x{:x}", renderer);
  // SCREEN.write().render(renderer);
}

struct Dimension {
  x: i32,
  y: i32,
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + 0xa00);
  if (x == dim.x - 1 && y == dim.y - 1) {
    // log::debug!("update_tile: {} {}", x, y);
    SCREEN.write().render(renderer);
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_anchor_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + 0xa00);
  if (x == dim.x - 1 && y == dim.y - 1) {
    log::debug!("update_anchor_tile: {} {}", x, y);
    // SCREEN.write().render(renderer);
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_top_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + 0xa00);
  // if (x == dim.x - 1 && y == dim.y - 1) {
  if (x == 0 && y == 0) {
    log::debug!("update_top_tile: {} {}", x, y);
    // SCREEN.write().render(renderer);
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn update_top_anchor_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + 0xa00);
  // if (x == dim.x - 1 && y == dim.y - 1) {
  if (x == 0 && y == 0) {
    log::debug!("update_top_anchor_tile: {} {}", x, y);
    // SCREEN.write().render(renderer);
  }
}
