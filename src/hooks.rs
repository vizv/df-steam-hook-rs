use anyhow::Result;
use retour::static_detour;

use crate::config::CONFIG;
use crate::cxxstring::CxxString;
use crate::dictionary::DICTIONARY;
use crate::enums::ScreenTexPosFlag;
use crate::global::GPS;
use crate::screen::{SCREEN, SCREEN_TOP};
use crate::{raw, utils};

use r#macro::hook;

pub unsafe fn attach_all() -> Result<()> {
  attach_addst()?;
  attach_addst_top()?;
  attach_addst_flag()?;
  attach_addchar_flag()?;
  attach_gps_allocate()?;
  attach_update_all()?;
  attach_update_tile()?;

  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_addst()?;
  enable_addst_top()?;
  enable_addst_flag()?;
  enable_addchar_flag()?;
  enable_gps_allocate()?;
  enable_update_all()?;
  enable_update_tile()?;

  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_addst()?;
  disable_addst_top()?;
  disable_addst_flag()?;
  disable_addchar_flag()?;
  disable_gps_allocate()?;
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
  unsafe { CxxString::new(ptr, len - 1) }
}

// FIXME: render the font, get real width, divided by 2, ceil it to curses font width
fn translate(string: usize) -> String {
  let mut content = raw::deref_string(string);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }
  content
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst(gps: usize, string: usize, just: u8, space: i32) {
  let (x, y) = gps_get_screen_coord(gps);
  let content = translate(string);

  let width = SCREEN.write().add(gps, x, y, content, 0);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_top(gps: usize, string: usize, just: u8, space: i32) {
  let (x, y) = gps_get_screen_coord(gps);
  let content = translate(string);

  let width = SCREEN_TOP.write().add(gps, x, y, content, 0);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_flag(gps: usize, string: usize, just: u8, space: i32, sflag: u32) {
  let (x, y) = gps_get_screen_coord(gps);
  let content = translate(string);

  let width = SCREEN.write().add(gps, x, y, content, sflag);
  unsafe { original!(gps, dummy_content(width).as_ptr() as usize, just, space, sflag) };
}

#[cfg_attr(target_os = "linux", hook(bypass))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addchar_flag(gps: usize, c: u8, advance: i8, sflag: u32) {
  if ScreenTexPosFlag::from_bits_retain(sflag).contains(ScreenTexPosFlag::TOP_OF_TEXT) {
    return;
  }

  unsafe { original!(gps, c, advance, sflag) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn gps_allocate(renderer: usize, w: u32, h: u32, screen_x: u32, screen_y: u32, tile_dim_x: u32, tile_dim_y: u32) {
  unsafe { original!(renderer, w, h, screen_x, screen_y, tile_dim_x, tile_dim_y) };
  SCREEN.write().resize(w, h);
  SCREEN_TOP.write().resize(w, h);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };
  SCREEN_TOP.write().render(renderer);
  SCREEN.write().clear();
  SCREEN_TOP.write().clear();
}

struct Dimension {
  x: i32,
  y: i32,
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = raw::deref::<Dimension>(GPS.to_owned() + CONFIG.offset.as_ref().unwrap().gps_offset_dimension.unwrap());

  // hack to render text after the last update_tile in update_all
  // TODO: consider re-write update_all function completely according to g_src
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  SCREEN.write().render(renderer);
}
