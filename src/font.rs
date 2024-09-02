use anyhow::Result;
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};
use std::cmp::min;
use std::collections::HashMap;
use std::io::Read;
use std::{mem, ptr};

use crate::config::CONFIG;
use crate::global::ENABLER;
use crate::{df, encodings, utils};

pub const CURSES_FONT_WIDTH: u32 = 16;
pub const CJK_FONT_SIZE: u32 = 24;

#[static_init::dynamic]
pub static mut FONT: Font = Font::new(&CONFIG.settings.font);

pub struct Font {
  font: fontdue::Font,
  cache: HashMap<char, usize>,
}

impl Font {
  fn new(path: &'static str) -> Self {
    Self {
      font: match Font::load(path) {
        Ok(value) => value,
        Err(_) => {
          log::error!("unable to load font {path}");
          utils::message_box(
            "dfint hook error",
            format!("Unable to load font {path}").as_str(),
            utils::MessageIconType::Warning,
          );
          panic!("unable to load font {path}")
        }
      },
      cache: Default::default(),
    }
  }

  pub fn get(&mut self, ch: char) -> (usize, bool) {
    if let Some(&code) = encodings::cp437::UTF8_CHAR_TO_CP437.get(&ch) {
      return (df::enabler::deref_curses_surface(ENABLER.to_owned(), code), true);
    };

    if !self.cache.contains_key(&ch) {
      let (metrics, bitmap) = self.font.rasterize(ch, CJK_FONT_SIZE as f32);
      if metrics.advance_width as u32 == CJK_FONT_SIZE && metrics.advance_height as u32 == CJK_FONT_SIZE {
        let mut surface = Surface::new(CJK_FONT_SIZE, CJK_FONT_SIZE, PixelFormatEnum::RGBA32).unwrap();
        surface.with_lock_mut(|buffer| {
          let dx = metrics.xmin;
          let dy = (CJK_FONT_SIZE as i32 - metrics.height as i32) - (metrics.ymin + 4); // Note: only for the "NotoSansMonoCJKsc-Bold" font
          let dy = if dy < 0 { 0 } else { dy };
          let width = min(metrics.width, CJK_FONT_SIZE as usize);
          let height = min(metrics.height, CJK_FONT_SIZE as usize);
          for y in 0..height {
            for x in 0..width {
              let alpha = (bitmap[y * metrics.width + x] as u16 * 255 / 255) as u8;
              let offset = ((y as i32 + dy) * CJK_FONT_SIZE as i32 + x as i32 + dx) as usize;
              buffer[offset * 4 + 0] = 255;
              buffer[offset * 4 + 1] = 255;
              buffer[offset * 4 + 2] = 255;
              buffer[offset * 4 + 3] = alpha;
            }
          }
        });
        let surface_ptr = surface.raw() as usize;
        mem::forget(surface);

        self.cache.insert(ch, surface_ptr);
      }
    }

    if let Some(surface_ptr) = self.cache.get(&ch) {
      return (surface_ptr.to_owned(), false);
    } else {
      // fallback to curses space glyph
      return (df::enabler::deref_curses_surface(ENABLER.to_owned(), ' ' as u8), true);
    }
  }

  pub fn render(&mut self, string: String) -> (usize, u32) {
    let width = CJK_FONT_SIZE * string.chars().count() as u32;
    let height = CJK_FONT_SIZE;
    let mut x = 0;
    let surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    for ch in string.chars() {
      let (surface_ptr, is_curses) = self.get(ch);
      let glyph_surface = surface_ptr as *mut sdl::SDL_Surface;
      let w = if is_curses { CURSES_FONT_WIDTH } else { CJK_FONT_SIZE };
      let h = CJK_FONT_SIZE;
      let mut rect = Rect::new(x, 0, w, h);
      unsafe { sdl::SDL_UpperBlitScaled(glyph_surface, ptr::null(), surface.raw(), rect.raw_mut()) };
      x += w as i32;
    }

    let surface_ptr = surface.raw() as usize;
    mem::forget(surface);

    return (surface_ptr, x as u32);
  }

  fn load(path: &str) -> Result<fontdue::Font> {
    let mut file = std::fs::File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).map_err(|err| anyhow::anyhow!(err))
  }
}

pub fn get_width(ch: char) -> u32 {
  if encodings::cjk::is_cjk(ch) {
    CJK_FONT_SIZE
  } else {
    CURSES_FONT_WIDTH
  }
}
