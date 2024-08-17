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

struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
}

#[derive(Default)]
pub struct Screen {
  dimension: (u32, u32),
  canvas_ptr: usize,
  // texts: Vec<Text>,
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
    if flag & 0b10000 != 0 {
      // we only renders the top half
      return;
    }

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;

    let mut x = CANVAS_FONT_WIDTH * x as f32;
    let mut y = CANVAS_FONT_HEIGHT * y as f32;
    if flag & 0b1000 != 0 {
      // shift down by half font height
      y += CANVAS_FONT_HEIGHT / 2.0;
    }
    content.chars().for_each(|ch| {
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
        sdl::SDL_UpperBlit(glyph_surface, ptr::null(), canvas, rect.raw_mut());
      }
      x += font::CJK_FONT_SIZE as f32;
    });
  }

  pub fn clear(&mut self) {
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
    let info = raw::deref::<ScreenInfo>(renderer + 0x160);

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
        info.origin_x as i32,
        info.origin_y as i32,
        self.dimension.0 * info.dispx_z as u32,
        self.dimension.1 * info.dispy_z as u32,
      );
      sdl::SDL_RenderCopy(sdl_renderer, texture, srcrect.raw(), dstrect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
