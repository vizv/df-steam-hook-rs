use anyhow::Result;
use cxx::let_cxx_string;
use retour::static_detour;

use crate::config::CONFIG;
use crate::dictionary::DICTIONARY;
use crate::global::{GAME, GPS};
use crate::markup::MARKUP;
use crate::screen::{self, SCREEN};
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
  attach_draw_horizontal_nineslice()?;
  attach_draw_nineslice()?;

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
  enable_draw_horizontal_nineslice()?;
  enable_draw_nineslice()?;

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
  disable_draw_horizontal_nineslice()?;
  disable_draw_nineslice()?;

  Ok(())
}

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

  let text = screen::Text::new(content).by_graphic(gps);
  let width = screen::SCREEN.write().add_text(text);

  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_top(gps: usize, string: usize, just: u8, space: i32) {
  let content = translate(string);

  // in order to get the correct coord for help markup text,
  // we need to render it here and skip the content from original text.
  let help = df::game::GameMainInterfaceHelp::borrow_from(GAME.to_owned());
  for text in &help.text {
    if let Some(word) = text.word.first_address() {
      // if we're rendering a help text - rendering its first word
      if string == word.to_owned() {
        MARKUP.write().render(gps, text.ptr());
        return;
      }
    }
  }

  let text = screen::Text::new(content).by_graphic(gps);
  let width = screen::SCREEN_TOP.write().add_text(text);

  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addst_flag(gps: usize, string: usize, just: u8, space: i32, sflag: u32) {
  let content = translate(string);

  let text = screen::Text::new(content).by_graphic(gps).with_sflag(sflag);
  let width = screen::SCREEN.write().add_text(text);

  let_cxx_string!(dummy = " ".repeat(width));
  let dummy_ptr: usize = unsafe { core::mem::transmute(dummy) };
  unsafe { original!(gps, dummy_ptr, just, space, sflag) };
}

#[cfg_attr(target_os = "linux", hook(bypass))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn addchar_flag(gps: usize, c: u8, advance: i8, sflag: u32) {
  // skip top-half character for Windows
  let flag = df::flags::ScreenTexPosFlag::from_bits_retain(sflag);
  if flag.contains(df::flags::ScreenTexPosFlag::TOP_OF_TEXT) {
    return;
  }

  unsafe { original!(gps, c, advance, sflag) };
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn draw_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    SCREEN.write().add_cover(cover);
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn draw_horizontal_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    SCREEN.write().add_cover(cover);
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn gps_allocate(renderer: usize, x: i32, y: i32, screen_x: u32, screen_y: u32, tile_dim_x: u32, tile_dim_y: u32) {
  // graphicst::resize is inlined in Windows, hook gps_allocate instead
  unsafe { original!(renderer, x, y, screen_x, screen_y, tile_dim_x, tile_dim_y) };
  screen::SCREEN.write().resize(x, y);
  screen::SCREEN_TOP.write().resize(x, y);
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };

  if df::graphic::top_in_use(GPS.to_owned()) {
    screen::SCREEN_TOP.write().render(renderer);
    screen::SCREEN_TOP.write().clear();
  }
}

#[cfg_attr(target_os = "linux", hook(by_symbol))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = df::graphic::deref_dim(GPS.to_owned());

  // hack to render text after the last update_tile in update_all
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  screen::SCREEN.write().render(renderer);
  screen::SCREEN.write().clear();
}

#[cfg_attr(target_os = "linux", hook(by_offset))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
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

#[cfg_attr(target_os = "linux", hook(by_offset))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn mtb_set_width(text_address: usize, current_width: i32) {
  let max_y = MARKUP.write().layout(text_address, current_width);

  // skip original function for help texts
  let help = df::game::GameMainInterfaceHelp::borrow_mut_from(GAME.to_owned());
  for text in &mut help.text {
    // if we're rendering a help text
    if text as *const df::game::MarkupTextBox as usize == text_address {
      // adjust the px and py to 0 (was -1 before original function call),
      // this helps the screen coord is correct for addst_top.
      if let Some(word) = text.word.first_mut::<df::game::MarkupTextWord>() {
        word.px = 0;
        word.py = 0;
      }

      // set to 0 to ensure mtb_set_width is called in every loop
      text.current_width = 0;
      // use the max_y from markup layout
      text.max_y = max_y;

      return;
    }
  }

  unsafe { original!(text_address, current_width) };
}

#[cfg_attr(target_os = "linux", hook(by_offset))]
#[cfg_attr(target_os = "windows", hook(by_offset))]
fn render_help_dialog(help_address: usize) {
  let help = df::game::GameMainInterfaceHelp::borrow_mut_at(help_address);

  // save end offset of word vector of each text,
  // and leave only one word in the vector to get screen coord for addst_top.
  let mut stored_end = [0; 20];
  for (i, text) in &mut help.text.iter_mut().enumerate() {
    stored_end[i] = text.word.end;
    text.word.end = text.word.begin + 8;
  }

  unsafe { original!(help_address) };

  // restore saved end offset of word vector of each text,
  // so the translation can be disabled at any point.
  for (i, text) in &mut help.text.iter_mut().enumerate() {
    text.word.end = stored_end[i];
  }
}
