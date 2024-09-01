use anyhow::Result;
use cxx::let_cxx_string;
use retour::static_detour;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use crate::global::{GAME, GPS};
use crate::markup::MARKUP;
use crate::screen::{ScreenText, SCREEN, SCREEN_TOP};
use crate::{df, utils};

use r#macro::hook;

pub unsafe fn attach_all() -> Result<()> {
  attach_addst()?;
  attach_addst_top()?;
  attach_addst_flag()?;
  attach_addchar_flag()?;
  attach_gps_allocate()?;
  attach_update_all()?;
  attach_update_tile()?;
  attach_mtb_process_string_to_lines()?;
  attach_mtb_set_width()?;
  attach_render_help_dialog()?;

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
  // always enable mtb_process_string_to_lines:
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
  // always enable mtb_process_string_to_lines:
  disable_mtb_set_width()?;
  disable_render_help_dialog()?;

  Ok(())
}

// FIXME: render the font, get real width, divided by 2, ceil it to curses font width
fn translate(string: usize) -> String {
  let mut content = df::utils::deref_string(string);
  if let Some(translated) = DICTIONARY.get(&content) {
    content = translated.to_owned();
  }
  content
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst(gps: usize, string: usize, just: u8, space: i32) {
  let content = translate(string);

  let text = ScreenText::new(content).by_graphic(gps);
  let width = SCREEN.write().add_text(text);
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_top(gps: usize, string: usize, just: u8, space: i32) {
  let content = translate(string);

  let help = df::game::GameMainInterfaceHelp::deref(GAME.to_owned());
  for text in &help.text {
    if let Some(word) = text.word.first_address() {
      if string == word.to_owned() {
        MARKUP.write().render(gps, text.ptr());
        return;
      }
    }
  }

  let text = ScreenText::new(content).by_graphic(gps);
  let width = SCREEN_TOP.write().add_text(text);
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_flag(gps: usize, string: usize, just: u8, space: i32, sflag: u32) {
  let content = translate(string);

  let text = ScreenText::new(content).by_graphic(gps).with_sflag(sflag);
  let width = SCREEN.write().add_text(text);
  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space, sflag) };
}

#[cfg_attr(target_os = "linux", hook(bypass))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addchar_flag(gps: usize, c: u8, advance: i8, sflag: u32) {
  let flag = df::flags::ScreenTexPosFlag::from_bits_retain(sflag);
  if flag.contains(df::flags::ScreenTexPosFlag::TOP_OF_TEXT) {
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

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = df::graphic::deref_dim(GPS.to_owned());

  // hack to render text after the last update_tile in update_all
  // TODO: consider re-write update_all function completely according to g_src
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  SCREEN.write().render(renderer);
}

#[cfg_attr(target_os = "linux", hook(offset = "018b77c0"))]
fn mtb_process_string_to_lines(text: usize, src: usize) {
  let content = translate(src);

  unsafe { original!(text, src) };

  // TODO: may need regexp for some scenarios like world generation status (0x22fa459)
  // TODO: log unknown text (during world generation)
  // examples: (they are coming from "data/vanilla/vanilla_buildings/objects/building_custom.txt")
  // * 0x7ffda475bbb8 Use tallow (rendered fat) or oil here with lye to make soap. 24
  // * 0x7ffda4663918 A useful workshop for pressing liquids from various sources. Some plants might need to be milled first before they can be used.  Empty jugs are required to store the liquid products. 24

  MARKUP.write().add(text, &content);
}

#[cfg_attr(target_os = "linux", hook(offset = "018b7340"))]
fn mtb_set_width(text: usize, current_width: i32) {
  let max_y = MARKUP.write().layout(text, current_width);

  let mut text = df::game::MarkupTextBox::at_mut(text);
  if let Some(word) = text.word.first_mut::<df::game::MarkupTextWord>() {
    word.px = 0;
    word.py = 0;
  }

  text.current_width = 0;
  text.max_y = max_y;
}

#[cfg_attr(target_os = "linux", hook(offset = "01193fe0"))]
fn render_help_dialog(help: usize) {
  let target = help + 0x30;
  let mut stored_end = [0; 20];
  for i in 0..20 {
    let begin_ptr = (target + i * 64) as *const usize;
    let end_ptr = (target + i * 64 + 8) as *mut usize;
    unsafe {
      stored_end[i] = *end_ptr;
      *end_ptr = *begin_ptr + 8;
    };
  }

  unsafe { original!(help) };

  for i in 0..20 {
    let end_ptr = (target + i * 64 + 8) as *mut usize;
    unsafe { *end_ptr = stored_end[i] };
  }
}
