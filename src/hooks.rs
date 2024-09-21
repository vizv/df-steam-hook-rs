use anyhow::Result;
use raw::{delete_cxxstring, new_cxxstring_n_chars};
use retour::static_detour;

use super::{df, encodings, markup, offsets, screen, translator, utils};

use r#macro::hook;

pub unsafe fn attach_all() -> Result<()> {
  attach_addst()?;
  attach_top_addst()?;
  attach_addst_flag()?;

  #[cfg(target_os = "windows")]
  {
    attach_addchar()?;
    attach_addchar_flag()?;
  }

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
  enable_top_addst()?;
  enable_addst_flag()?;

  #[cfg(target_os = "windows")]
  {
    enable_addchar()?;
    enable_addchar_flag()?;
  }

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
  disable_top_addst()?;
  disable_addst_flag()?;

  #[cfg(target_os = "windows")]
  {
    disable_addchar()?;
    disable_addchar_flag()?;
  }

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

#[cfg_attr(target_os = "linux", hook)]
#[cfg_attr(target_os = "windows", hook(bypass))]
fn addst(gps: usize, string_address: usize, just: u8, space: i32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  let text = screen::Text::new(translator::TRANSLATOR.write().translate("addst", &string, &bt)).by_gps(gps);
  let width = screen::SCREEN.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space) };
  delete_cxxstring(dummy_ptr);
}

#[cfg_attr(target_os = "linux", hook)]
#[cfg_attr(target_os = "windows", hook(bypass))]
fn addst_flag(gps: usize, string_address: usize, just: u8, space: i32, sflag: u32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  let text =
    screen::Text::new(translator::TRANSLATOR.write().translate("addst_flag", &string, &bt)).by_gps(gps).with_sflag(sflag);
  let width = screen::SCREEN.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space, sflag) };
  delete_cxxstring(dummy_ptr);
}

#[cfg(target_os = "windows")]
#[static_init::dynamic]
static mut STRING_COLLECTOR: StringCollector = Default::default();

#[cfg(target_os = "windows")]
#[derive(Debug, Default)]
struct StringCollector {
  last_caller: String,
  last_coord: df::common::Coord<i32>,
  last_sflag: u32,
  last_color_info: df::gps::ColorInfo,
  chars: Vec<u8>,
}

#[cfg(target_os = "windows")]
impl StringCollector {
  fn push(&mut self, caller: String, gps: usize, ch: u8, sflag: u32) {
    if caller == "" && self.last_caller == "" {
      return;
    }

    let mut coord = df::gps::read_coord(gps);
    let color_info = df::gps::read_color_info(gps);
    if caller == ""
      || coord != self.last_coord
      || caller != self.last_caller
      || sflag != self.last_sflag
      || color_info != self.last_color_info
    {
      if self.last_caller != "" && !self.chars.is_empty() {
        df::gps::set_coord(gps, &self.last_coord);
        df::gps::set_color_info(gps, &self.last_color_info);

        let string = encodings::bytes_to_string(&self.chars);

        let text = screen::Text::new(translator::TRANSLATOR.write().translate("string_collector", &string, &self.last_caller))
          .by_gps(gps)
          .with_sflag(self.last_sflag);
        let width = screen::SCREEN.write().add_text(text);

        for _ in 0..width {
          unsafe { handle_addchar_flag.call(gps, ' ' as u8, 1, self.last_sflag) };
        }

        if coord == self.last_coord {
          coord = df::gps::read_coord(gps);
        }
        df::gps::set_coord(gps, &coord);
        df::gps::set_color_info(gps, &color_info);
        self.chars.clear();
      }
    }

    if caller == "" {
      *self = Default::default();
      return;
    }

    self.last_caller = caller;
    self.last_coord = coord;
    self.last_sflag = sflag;
    self.last_color_info = color_info;
    self.chars.push(ch);
  }
}

#[cfg(target_os = "windows")]
#[hook]
fn addchar(gps: usize, ch: u8, advance: u8) {
  if ch == 0 || ch == 219 || advance != 1 {
    STRING_COLLECTOR.write().push("".into(), *df::globals::GPS, 0, 0);
    unsafe { original!(gps, ch, advance) };
    return;
  }

  let caller = utils::backtrace();
  STRING_COLLECTOR.write().push(caller, gps, ch, 0);
}

#[cfg(target_os = "windows")]
#[hook]
fn addchar_flag(gps: usize, ch: u8, advance: i8, sflag: u32) {
  if ch == 0 || ch == 219 || advance != 1 {
    STRING_COLLECTOR.write().push("".into(), *df::globals::GPS, 0, 0);
    unsafe { original!(gps, ch, advance, sflag) };
    return;
  }

  let caller = utils::backtrace();
  STRING_COLLECTOR.write().push(caller, gps, ch, sflag);
}

#[hook]
fn top_addst(gps: usize, string_address: usize, just: u8, space: i32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  // in order to get the correct coord for help markup text,
  // we need to render it here and skip the content from original text.
  let help = df::game::GameMainInterfaceHelp::borrow(*df::globals::GAME);
  for text in &help.text {
    if let Some(word) = text.word.first_address() {
      // if we're rendering a help text - rendering its first word
      if string_address == word.to_owned() {
        markup::MARKUP.write().render(gps, raw::ptr(text));
        return;
      }
    }
  }

  let text = screen::Text::new(translator::TRANSLATOR.write().translate("top_addst", &string, &bt)).by_gps(gps);
  let width = screen::SCREEN_TOP.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space) };
  delete_cxxstring(dummy_ptr);
}

#[hook]
fn draw_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    screen::SCREEN.write().add_cover(cover);
  }
}

#[hook]
fn draw_horizontal_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    screen::SCREEN.write().add_cover(cover);
  }
}

#[hook]
fn gps_allocate(renderer: usize, x: i32, y: i32, screen_x: u32, screen_y: u32, tile_dim_x: u32, tile_dim_y: u32) {
  // graphicst::resize is inlined in Windows, hook gps_allocate instead
  unsafe { original!(renderer, x, y, screen_x, screen_y, tile_dim_x, tile_dim_y) };
  screen::SCREEN.write().resize(x, y);
  screen::SCREEN_TOP.write().resize(x, y);
}

#[hook]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };

  if df::gps::top_in_use(*df::globals::GPS) {
    screen::SCREEN_TOP.write().render(renderer);
    screen::SCREEN_TOP.write().clear();
  }
}

#[hook]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = df::gps::borrow_dim(*df::globals::GPS);

  // hack to render text after the last update_tile in update_all
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  #[cfg(target_os = "windows")]
  {
    STRING_COLLECTOR.write().push("".into(), *df::globals::GPS, 0, 0);
  }
  screen::SCREEN.write().render(renderer);
  screen::SCREEN.write().clear();
}

#[hook]
fn mtb_process_string_to_lines(text: usize, string_address: usize) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  unsafe { original!(text, string_address) };

  // TODO: may need regexp for some scenarios like world generation status (0x22fa459)
  // TODO: log unknown text (during world generation)
  // examples: (they are coming from "data/vanilla/vanilla_buildings/objects/building_custom.txt")
  // * 0x7ffda475bbb8 Use tallow (rendered fat) or oil here with lye to make soap. 24
  // * 0x7ffda4663918 A useful workshop for pressing liquids from various sources. Some plants might need to be milled first before they can be used.  Empty jugs are required to store the liquid products. 24

  markup::MARKUP.write().add(text, translator::TRANSLATOR.write().translate("addst", &string, &bt).0);
}

#[hook]
fn mtb_set_width(text_address: usize, current_width: i32) {
  let max_y = markup::MARKUP.write().layout(text_address, current_width);

  // skip original function for help texts
  let help = df::game::GameMainInterfaceHelp::borrow_mut(*df::globals::GAME);
  for text in &mut help.text {
    // if we're rendering a help text
    if text as *const df::game::MarkupTextBox as usize == text_address {
      // adjust the px and py to 0 (was -1 before original function call),
      // this helps the screen coord is correct for top_addst.
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

#[hook]
fn render_help_dialog(help_address: usize) {
  let help = raw::as_ref_mut::<df::game::GameMainInterfaceHelp>(help_address);

  // save end offset of word vector of each text,
  // and leave only one word in the vector to get screen coord for top_addst.
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
