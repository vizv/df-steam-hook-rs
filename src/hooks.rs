use anyhow::Result;
use cxx::{let_cxx_string, CxxVector};
use retour::static_detour;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use crate::enums::ScreenTexPosFlag;
use crate::global::GPS;
use crate::markup::MARKUP;
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

  attach_add_paragraph()?;
  attach_mtb_process_string_to_lines()?;
  attach_mtb_set_width()?;
  attach_render_help_dialog()?;

  // attach_debug_get_main_interface_dims()?;
  // attach_debug_get_dialog_size()?;
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

  enable_add_paragraph()?;
  // always enable mtb_process_string_to_lines:
  // enable_mtb_process_string_to_lines()?;
  enable_mtb_set_width()?;
  enable_render_help_dialog()?;
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

  disable_add_paragraph()?;
  // always enable mtb_process_string_to_lines:
  // disable_mtb_process_string_to_lines()?;
  disable_mtb_set_width()?;
  disable_render_help_dialog()?;
  Ok(())
}

fn gps_get_screen_coord(addr: usize) -> (i32, i32) {
  (
    raw::deref::<i32>(addr + 0x84), // gps.screenx
    raw::deref::<i32>(addr + 0x88), // gps.screeny
  )
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
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_top(gps: usize, string: usize, just: u8, space: i32) {
  let (x, y) = gps_get_screen_coord(gps);
  let content = translate(string);

  let width = SCREEN_TOP.write().add(gps, x, y, content, 0);
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_flag(gps: usize, string: usize, just: u8, space: i32, sflag: u32) {
  let (x, y) = gps_get_screen_coord(gps);
  let content = translate(string);

  let width = SCREEN.write().add(gps, x, y, content, sflag);
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space, sflag) };
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

#[cfg_attr(
  target_os = "linux",
  hook(
    module = "self",
    symbol = "_ZN17curses_text_boxst13add_paragraphERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEi"
  )
)]
fn add_paragraph(text_box: usize, src: usize, para_width: i32) {
  let mut content = raw::deref_string(src);
  unsafe {
    original!(text_box, src, para_width);
  }
}

#[cfg_attr(target_os = "linux", hook(offset = "018b77c0"))]
fn mtb_process_string_to_lines(markup_text_box: usize, src: usize) {
  let content = translate(src);
  log::warn!("0x{markup_text_box:x}: content = {content}");

  unsafe { original!(markup_text_box, src) };

  // TODO: translate the whole text like help texts (0x21da0f0)
  // TODO: may need regexp for some scenarios like world generation status (0x22fa459)
  // TODO: log unknown markup_text_box (during world generation)
  // examples: (they are coming from "data/vanilla/vanilla_buildings/objects/building_custom.txt")
  // * 0x7ffda475bbb8 Use tallow (rendered fat) or oil here with lye to make soap. 24
  // * 0x7ffda4663918 A useful workshop for pressing liquids from various sources. Some plants might need to be milled first before they can be used.  Empty jugs are required to store the liquid products. 24
  // log::info!("??? 0x{:x} {}", markup_text_box, content);
  MARKUP.write().add(markup_text_box, &content);
}

#[cfg_attr(target_os = "linux", hook(offset = "018b7340"))]
fn mtb_set_width(markup_text_box: usize, current_width: i32) {
  // log::info!("??? mtb_set_width 0x{markup_text_box:x} {current_width}");
  unsafe { original!(markup_text_box, current_width) };
  MARKUP.write().layout(markup_text_box, current_width);
  unsafe { *((markup_text_box + 0x30) as *mut i32) = 0 };
}

static mut SAVED_CONTEXT: u32 = u32::MAX;

// fn x() {
//   let mut vec = CxxVector::<usize>::new();
//   // let vec_ptr: usize = unsafe { core::mem::transmute(vec) };
//   let mut vecz = CxxVector::<usize>::new();
//   core::mem::swap(&mut vec, &mut vecz);
// }

#[cfg_attr(target_os = "linux", hook(offset = "01193fe0"))]
fn render_help_dialog(help: usize) {
  // let save = CxxVector::<usize>::new();
  // let save_ptr: usize = unsafe { core::mem::transmute(save) };
  // let stub = CxxVector::<usize>::new();
  // let stub_ptr: usize = unsafe { core::mem::transmute(stub) };
  let target = help + 0x30;

  // unsafe {
  //   core::ptr::copy_nonoverlapping(target as *mut u8, save_ptr as *mut u8, 24);
  //   core::ptr::copy_nonoverlapping(stub_ptr as *mut u8, target as *mut u8, 24);
  //   // *((target + 0x34) as *mut i32) = 1;
  // }

  // log::info!("??? render_help_dialog");
  unsafe {
    let context = raw::deref::<u32>(help + 0xc);
    if SAVED_CONTEXT != context {
      SAVED_CONTEXT = context;
      // log::warn!("context = {}", context);
    }
  };
  unsafe {
    original!(help);
  }

  // unsafe {
  //   core::ptr::copy_nonoverlapping(save_ptr as *mut u8, target as *mut u8, 24);
  //   // *((target + 0x30) as *mut i32) = *((target + 0x30) as *const i32) + 2;
  // }
}

#[cfg_attr(target_os = "linux", hook(offset = "0136e770"))]
fn debug_get_main_interface_dims(game: usize, p1: usize, p2: usize) {
  unsafe { original!(game, p1, p2) };
  let p1 = raw::deref::<i32>(p1);
  let p2 = raw::deref::<i32>(p2);
  log::info!("debug_get_main_interface_dims({p1},{p2})");
}

#[cfg_attr(target_os = "linux", hook(offset = "0118fd00"))]
fn debug_get_dialog_size(help: usize, p1: usize, p2: usize, p3: usize, p4: usize) {
  unsafe { original!(help, p1, p2, p3, p4) };
  let p1 = raw::deref::<i32>(p1);
  let p2 = raw::deref::<i32>(p2);
  let p3 = raw::deref::<i32>(p3);
  let p4 = raw::deref::<i32>(p4);
  log::info!("debug_get_dialog_size({p1},{p2},{p3},{p4})");
}
