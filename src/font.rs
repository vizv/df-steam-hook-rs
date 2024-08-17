use anyhow::Result;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use std::char;
use std::collections::HashMap;
use std::io::{self, prelude::*};

use crate::constants::PATH_FONT;
use crate::utils;

pub const CJK_FONT_SIZE: u32 = 16;

#[static_init::dynamic]
pub static mut FONT: Font = Font::new(PATH_FONT);

const GLYPH_DATA_SIZE: usize = (CJK_FONT_SIZE * CJK_FONT_SIZE / 8) as usize;
type Glyph = (u16, [u8; GLYPH_DATA_SIZE]);

pub struct Font {
  glyphs: HashMap<u16, Glyph>,
  textures: HashMap<u16, usize>,
}

impl Font {
  fn new(path: &'static str) -> Self {
    Self {
      glyphs: match Font::load(path) {
        Ok(value) => value,
        Err(_) => {
          log::error!("unable to load font {path}");
          utils::message_box(
            "dfint hook error",
            format!("Unable to load font {path}").as_str(),
            utils::MessageIconType::Warning,
          );
          Default::default()
        }
      },
      textures: Default::default(),
    }
  }

  pub fn render(&mut self, unicode: u16) -> usize {
    if let Some(surface_ptr) = self.textures.get(&unicode) {
      return surface_ptr.to_owned();
    }

    if let Some(glyph) = self.glyphs.get(&unicode) {
      let mut surface = Surface::new(CJK_FONT_SIZE, CJK_FONT_SIZE, PixelFormatEnum::RGBA32).unwrap();
      surface.with_lock_mut(|buffer| {
        let data = glyph.1;
        for i in 0..GLYPH_DATA_SIZE {
          let mut byte = data[i];
          for j in 0..8 {
            let offset = (i * 8 + j) * 4;
            let value = if byte & 1 == 1 { 0xff } else { 0 };
            buffer[offset..offset + 4].fill(value);
            byte >>= 1;
          }
        }
      });
      let surface_ptr = surface.raw() as usize;
      std::mem::forget(surface);
      self.textures.insert(unicode, surface_ptr);
      log::debug!(
        "render new glyph {} for {} to 0x{:x}",
        unicode,
        unsafe { char::from_u32_unchecked(unicode as u32) },
        surface_ptr
      );

      return surface_ptr;
    }

    return 0;
  }

  pub fn size(&self) -> usize {
    self.glyphs.len()
  }

  fn load(path: &str) -> Result<HashMap<u16, Glyph>> {
    let file = std::fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut map = HashMap::<u16, Glyph>::new();
    let mut buf = [0 as u8; size_of::<Glyph>()];
    while let Ok(()) = reader.read_exact(&mut buf) {
      let glyph: Glyph = unsafe { std::mem::transmute(buf) };
      map.insert(glyph.0, glyph);
    }
    Ok(map)
  }
}
