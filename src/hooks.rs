use anyhow::Result;
use retour::static_detour;
use unicode_width::UnicodeWidthStr;

use crate::config::CONFIG;
use crate::cxxstring::CxxString;
use crate::dictionary::DICTIONARY;
use crate::global::GPS;
use crate::screen::{SCREEN, SCREEN_TOP};
use crate::{raw, utils};

use r#macro::hook;

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

fn dummy_content(width: usize) -> CxxString {
  let mut dummy: Vec<u8> = Vec::new();
  dummy.resize(width + 1, 32);
  dummy[width] = 0;
  let (ptr, len, _) = dummy.into_raw_parts();
  unsafe { CxxString::new(ptr, len) }
}

fn translate(string: usize) -> (String, usize) {
  let mut content = raw::deref_string(string);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }
  let width = (content.width() as f32 * 6.0 / 8.0).ceil() as usize;
  (content, width)
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst(gps: usize, string: usize, just: u8, space: i32) {
  let (x, y) = gps_get_screen_coord(gps);
  let (content, width) = translate(string);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space) };

  SCREEN.write().add(gps, x, y, content, width);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_top(gps: usize, string: usize, just: u8, space: i32) {
  let (x, y) = gps_get_screen_coord(gps);
  let (content, width) = translate(string);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space) };

  SCREEN_TOP.write().add(gps, x, y, content, width);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn addst_flag(gps: usize, string: usize, just: u8, space: i32, sflag: u32) {
  let (x, y) = gps_get_screen_coord(gps);
  let (content, width) = translate(string);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space, sflag) };

  SCREEN.write().add(gps, x, y, content, width);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn erasescreen(gps: usize) {
  unsafe { original!(gps) };
  SCREEN.write().clear();
  SCREEN_TOP.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
fn resize(renderer: usize, w: u32, h: u32) {
  unsafe { original!(renderer, w, h) };
  SCREEN.write().resize(w, h);
  SCREEN_TOP.write().resize(w, h);
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
