use std::{mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{
  font::{self, FONT},
  raw,
};

const CANVAS_FONT_WIDTH: f32 = (font::CJK_FONT_SIZE as f32) * 2.0 / 3.0;
const CANVAS_FONT_HEIGHT: f32 = font::CJK_FONT_SIZE as f32;

#[static_init::dynamic]
pub static mut SCREEN: Screen = Default::default();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Default::default();

pub struct Text {
  x: i32,
  y: i32,
  content: String,
  flag: u32,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenInfo {
  pub origin_x: i32,
  pub origin_y: i32,
  pub cur_w: i32,
  pub cur_h: i32,
}

#[derive(Default)]
pub struct Screen {
  dimension: (u32, u32),
  canvas_ptr: usize,
  texts: Vec<Text>,
}

impl Screen {
  pub fn resize(&mut self, w: u32, h: u32) {
    log::debug!("resize: {},{}", w, h);
    self.dimension.0 = w;
    self.dimension.1 = h;

    if self.canvas_ptr != 0 {
      let canvas = unsafe { Surface::from_ll(self.canvas_ptr as *mut sdl::SDL_Surface) };
      mem::drop(canvas);
    }

    let canvas = Surface::new(
      (w as f32 * CANVAS_FONT_WIDTH).round() as u32,
      (h as f32 * CANVAS_FONT_HEIGHT).round() as u32,
      PixelFormatEnum::RGBA32,
    )
    .unwrap();
    self.canvas_ptr = canvas.raw() as usize;
    mem::forget(canvas);
  }

  pub fn add(&mut self, x: i32, y: i32, content: String, flag: u32) {
    self.texts.push(Text { x, y, content, flag });
  }

  pub fn clear(&mut self) {
    self.texts.clear();
    unsafe {
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_FillRect(canvas, ptr::null(), 0);
    }
  }

  pub fn render(&mut self, renderer: usize) {
    if self.canvas_ptr == 0 {
      return;
    }

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
    let info = raw::deref::<ScreenInfo>(renderer + 0x168);

    for text in &self.texts {
      let mut x = CANVAS_FONT_WIDTH * text.x as f32;
      let y = CANVAS_FONT_HEIGHT * text.y as f32;
      text.content.chars().for_each(|ch| {
        let unicode = ch as u16;
        if unicode < 256 {
          return;
        }

        unsafe {
          let glyph_surface = FONT.write().render(unicode) as *mut sdl::SDL_Surface;
          let mut rect = Rect::new(
            x.round() as i32,
            y.round() as i32,
            font::CJK_FONT_SIZE,
            font::CJK_FONT_SIZE,
          );
          sdl::SDL_UpperBlitScaled(glyph_surface, ptr::null(), canvas, rect.raw_mut());
        }
        x += font::CJK_FONT_SIZE as f32;
      });
    }

    unsafe {
      let sdl_renderer = raw::deref(renderer + 0x108);
      let texture = sdl::SDL_CreateTextureFromSurface(sdl_renderer, canvas);
      sdl::SDL_SetTextureScaleMode(texture, sdl::SDL_ScaleMode::SDL_ScaleModeLinear);
      let srcrect = Rect::new(
        0,
        0,
        (self.dimension.0 as f32 * CANVAS_FONT_WIDTH).round() as u32,
        (self.dimension.1 as f32 * CANVAS_FONT_HEIGHT).round() as u32,
      );
      let dstrect = Rect::new(
        (info.origin_x as f32 * 2.0 / 3.0) as i32,
        (info.origin_y as f32 * 2.0 / 3.0) as i32,
        (info.cur_w - info.origin_x) as u32,
        (info.cur_h - info.origin_y) as u32,
      );
      sdl::SDL_RenderCopy(sdl_renderer, texture, srcrect.raw(), dstrect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
