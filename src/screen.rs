use std::{mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{font::FONT, raw};

pub const CANVAS_FONT_WIDTH: i32 = 8 * 2;
pub const CANVAS_FONT_HEIGHT: i32 = 12 * 2;

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
}

impl Screen {
  pub fn resize(&mut self, w: u32, h: u32) {
    self.dimension.0 = w;
    self.dimension.1 = h;

    if self.canvas_ptr != 0 {
      let canvas = unsafe { Surface::from_ll(self.canvas_ptr as *mut sdl::SDL_Surface) };
      mem::drop(canvas);
    }

    let canvas = Surface::new(
      w * CANVAS_FONT_WIDTH as u32,
      h * CANVAS_FONT_HEIGHT as u32,
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

    let x = CANVAS_FONT_WIDTH * x;
    let mut y = CANVAS_FONT_HEIGHT as i32 * y;
    if flag & 0b1000 != 0 {
      // shift down by half font height
      y += CANVAS_FONT_HEIGHT / 2;
    }

    unsafe {
      let mut font = FONT.write();
      let surface = font.render(content);
      let mut rect = Rect::new(x, y, surface.width(), surface.height());
      sdl::SDL_UpperBlitScaled(surface.raw(), ptr::null(), canvas, rect.raw_mut());
    };
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
        self.dimension.0 * CANVAS_FONT_WIDTH as u32,
        self.dimension.1 * CANVAS_FONT_HEIGHT as u32,
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
