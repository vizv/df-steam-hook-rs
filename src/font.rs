use anyhow::Result;
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};
use std::collections::HashMap;
use std::io::Read;
use std::{mem, ptr};

use crate::config::CONFIG;
use crate::{df, encodings, utils};

pub const CJK_FONT_SIZE: u32 = 24;
pub const BUF_SIZE: isize = (CJK_FONT_SIZE * CJK_FONT_SIZE) as isize;

#[static_init::dynamic]
pub static mut FONT: Font = Font::new(&CONFIG.settings.font_file);

pub struct Font {
  font: fontdue::Font,
  cache: HashMap<char, usize>,
}

impl Font {
  fn new(path: &'static str) -> Self {
    Self {
      font: match Font::load(path) {
        Ok(value) => value,
        Err(message) => {
          let message = &format!("加载字体文件失败：{message}");
          utils::show_error_dialog(&message);
          panic!("{}", message);
        }
      },
      cache: Default::default(),
    }
  }

  pub fn get(&mut self, ch: char) -> usize {
    if let Some(code) = encodings::utf8_char_to_ch437_byte(ch) {
      return df::enabler::get_curses_surface(*df::globals::ENABLER, code);
    };

    if !self.cache.contains_key(&ch) {
      let (metrics, bitmap) = self.font.rasterize(ch, CJK_FONT_SIZE as f32);
      if metrics.advance_width as u32 == CJK_FONT_SIZE && metrics.advance_height as u32 == CJK_FONT_SIZE {
        let mut surface = Surface::new(CJK_FONT_SIZE, CJK_FONT_SIZE, PixelFormatEnum::RGBA32).unwrap();
        surface.with_lock_mut(|buffer| {
          let dx = metrics.xmin;
          let dy = (CJK_FONT_SIZE as i32 - metrics.height as i32) - (metrics.ymin + 3); // Note: only for the "NotoSansMonoCJKsc-Bold" font
          let dy = if dy < 0 { 0 } else { dy };
          for y in 0..metrics.height {
            for x in 0..metrics.width {
              let alpha = (bitmap[y * metrics.width + x] as u16 * 255 / 255) as u8;

              let offset = ((y as i32 + dy) * CJK_FONT_SIZE as i32 + x as i32 + dx) as isize;
              if offset < 0 || offset >= BUF_SIZE {
                continue;
              }
              let offset = offset as usize;

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

    if let Some(&surface_ptr) = self.cache.get(&ch) {
      return surface_ptr;
    } else {
      // fallback to curses space glyph
      return df::enabler::get_curses_surface(*df::globals::ENABLER, ' ' as u8);
    }
  }

  pub fn render(&mut self, string: String, fg: df::common::Color, bg: Option<df::common::Color>) -> (usize, u32) {
    let width = encodings::string_width_in_pixels(&string);
    let height = CJK_FONT_SIZE;
    let mut x = 0;
    let text_surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    for ch in string.chars() {
      let surface_ptr = self.get(ch);
      let glyph_surface = surface_ptr as *mut sdl::SDL_Surface;
      let w = encodings::char_width_in_pixels(ch);
      let h = CJK_FONT_SIZE;
      let mut rect = Rect::new(x, 0, w, h);
      unsafe { sdl::SDL_UpperBlitScaled(glyph_surface, ptr::null(), text_surface.raw(), rect.raw_mut()) };
      x += w as i32;
    }
    let df::common::Color { r, g, b } = fg;
    unsafe { sdl::SDL_SetSurfaceColorMod(text_surface.raw(), r, g, b) };

    let surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    if let Some(df::common::Color { r, g, b }) = bg {
      let bc = sdl2::pixels::Color::RGB(r, g, b).to_u32(&surface.pixel_format());
      unsafe { sdl::SDL_FillRect(surface.raw(), ptr::null(), bc) };
    }
    unsafe { sdl::SDL_UpperBlit(text_surface.raw(), ptr::null(), surface.raw(), ptr::null_mut()) };

    let surface_ptr = surface.raw() as usize;
    mem::forget(surface);

    return (surface_ptr, width);
  }

  fn load(path: &str) -> Result<fontdue::Font> {
    let mut file = std::fs::File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).map_err(|err| anyhow::anyhow!(err))
  }
}
