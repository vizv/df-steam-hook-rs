pub mod colored_text;
pub mod constants;
pub mod text;

use std::{collections::HashMap, mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{
  df,
  font::{CJK_FONT_SIZE, FONT},
};

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new();

#[derive(Default)]
pub struct Screen {
  dimension: (u32, u32),
  canvas_ptr: usize,
  // cache: hash(colored_text) => (surface_ptr, width)
  prev: HashMap<u64, (usize, u32)>,
  next: HashMap<u64, (usize, u32)>,
}

impl Screen {
  pub fn new() -> Self {
    Self {
      dimension: Default::default(),
      canvas_ptr: Default::default(),
      prev: Default::default(),
      next: Default::default(),
    }
  }

  pub fn resize(&mut self, w: u32, h: u32) {
    self.dimension.0 = w;
    self.dimension.1 = h;

    if self.canvas_ptr != 0 {
      let canvas = unsafe { Surface::from_ll(self.canvas_ptr as *mut sdl::SDL_Surface) };
      mem::drop(canvas);
    }

    let canvas = Surface::new(
      w * constants::CANVAS_FONT_WIDTH as u32,
      h * constants::CANVAS_FONT_HEIGHT as u32,
      PixelFormatEnum::RGBA32,
    )
    .unwrap();
    self.canvas_ptr = canvas.raw() as usize;
    mem::forget(canvas);
  }

  pub fn add_text(&mut self, text: text::ScreenText) -> usize {
    let text::ScreenText {
      data: text,
      coord: df::common::Coord { x, y },
      render,
    } = text;

    if !render {
      return 0;
    }

    // render text or get from cache
    let key = text.key();
    let (surface_ptr, width) = match self.prev.get(&key) {
      Some((ptr, width)) => (ptr.to_owned() as *mut sdl::SDL_Surface, width.to_owned()),
      None => {
        let mut font = FONT.write();
        let (ptr, width) = font.render(text.content);
        let ptr = ptr as *mut sdl::SDL_Surface;
        mem::drop(font);

        unsafe { sdl::SDL_SetSurfaceColorMod(ptr, text.color.r, text.color.g, text.color.b) };

        (ptr, width)
      }
    };
    self.next.insert(key, (surface_ptr as usize, width));

    // render on canvas
    unsafe {
      let mut rect = Rect::new(x, y, width, CJK_FONT_SIZE);
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_UpperBlit(surface_ptr, ptr::null(), canvas, rect.raw_mut());
    };

    (width as f32 / constants::CANVAS_FONT_WIDTH as f32).ceil() as usize
  }

  pub fn clear(&mut self) {
    unsafe {
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_FillRect(canvas, ptr::null(), 0);
    }

    for (k, (ptr, _)) in self.prev.iter() {
      if !self.next.contains_key(k) {
        unsafe { sdl::SDL_FreeSurface(ptr.to_owned() as *mut sdl::SDL_Surface) };
      }
    }

    self.prev = mem::take(&mut self.next);
  }

  pub fn render(&mut self, renderer: usize) {
    if self.canvas_ptr == 0 {
      return;
    }

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
    let sdl_renderer = df::renderer::deref_sdl_renderer(renderer);

    let srcrect = Rect::new(
      0,
      0,
      self.dimension.0 * constants::CANVAS_FONT_WIDTH as u32,
      self.dimension.1 * constants::CANVAS_FONT_HEIGHT as u32,
    );

    let df::renderer::ScreenInfo {
      dispx_z,
      dispy_z,
      origin_x,
      origin_y,
    } = df::renderer::deref_screen_info(renderer);
    let dstrect = Rect::new(
      origin_x as i32,
      origin_y as i32,
      self.dimension.0 * dispx_z as u32,
      self.dimension.1 * dispy_z as u32,
    );

    unsafe {
      let texture = sdl::SDL_CreateTextureFromSurface(sdl_renderer, canvas);
      sdl::SDL_SetTextureScaleMode(texture, sdl::SDL_ScaleMode::SDL_ScaleModeLinear);
      sdl::SDL_RenderCopy(sdl_renderer, texture, srcrect.raw(), dstrect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
