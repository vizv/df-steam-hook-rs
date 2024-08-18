use anyhow::Result;
use cosmic_text::fontdb::{Database, Source};
use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use std::path::PathBuf;

use crate::config::CONFIG;
use crate::utils;

pub const CJK_FONT_SIZE: u32 = 24;

#[static_init::dynamic]
pub static mut FONT: Font = Font::new(&CONFIG.settings.font);

pub struct Font {
  font_system: FontSystem,
}

impl Font {
  fn new(path: &'static str) -> Self {
    Self {
      font_system: match Font::load(path) {
        Ok(value) => value,
        Err(_) => {
          log::error!("unable to load font {path}");
          utils::message_box(
            "dfint hook error",
            format!("Unable to load font {path}").as_str(),
            utils::MessageIconType::Warning,
          );
          FontSystem::new()
        }
      },
    }
  }

  pub fn render(&mut self, string: String) -> Surface {
    let metrics = Metrics::new(CJK_FONT_SIZE as f32, CJK_FONT_SIZE as f32);
    let mut buffer = Buffer::new(&mut self.font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut self.font_system);
    buffer.set_text(&string, Attrs::new(), Shaping::Advanced);
    buffer.shape_until_scroll(true);
    const TEXT_COLOR: Color = Color::rgb(0xff, 0xff, 0xff);
    let mut swash_cache = SwashCache::new();
    let width = CJK_FONT_SIZE * string.len() as u32;
    let height = CJK_FONT_SIZE;
    let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    buffer.draw(&mut swash_cache, TEXT_COLOR, |x, y, w, h, c| {
      surface.with_lock_mut(|buffer| {
        if c.a() == 0 || x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
          return;
        }

        let offset = y as usize * width as usize + x as usize;
        buffer[offset * 4 + 0] = c.r();
        buffer[offset * 4 + 1] = c.g();
        buffer[offset * 4 + 2] = c.b();
        buffer[offset * 4 + 3] = c.a();
      });
    });

    return surface;
  }

  fn load(path: &str) -> Result<FontSystem> {
    let font = Source::File(PathBuf::from(path));
    let mut db = Database::new();
    db.load_font_source(font);

    Ok(FontSystem::new_with_locale_and_db(Default::default(), db))
  }
}
