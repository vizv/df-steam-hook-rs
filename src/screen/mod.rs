pub mod constants;
pub mod cover;
pub mod data;
pub mod text;

pub use constants::CANVAS_FONT_HEIGHT;
pub use constants::CANVAS_FONT_WIDTH;
pub use cover::Cover;
use data::Data;
pub use text::Text;

use crate::{
  df,
  font::{CJK_FONT_SIZE, FONT},
};
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};
use std::{collections::HashMap, mem, ptr};

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new();

#[derive(Default)]
pub struct Screen {
  dimension: df::common::Dimension<i32>,
  canvas_ptr: usize,
  // cache: hash(data) => (surface_ptr, width)
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

  pub fn resize(&mut self, x: i32, y: i32) {
    self.dimension = df::common::Dimension { x, y };

    if self.canvas_ptr != 0 {
      let canvas = unsafe { Surface::from_ll(self.canvas_ptr as *mut sdl::SDL_Surface) };
      mem::drop(canvas);
    }

    let canvas = Surface::new(
      (x * CANVAS_FONT_WIDTH) as u32,
      (y * CANVAS_FONT_HEIGHT) as u32,
      PixelFormatEnum::RGBA32,
    )
    .unwrap();
    self.canvas_ptr = canvas.raw() as usize;
    mem::forget(canvas);
  }

  pub fn add_text(&mut self, text: Text) -> usize {
    let Text {
      data,
      coord: df::common::Coord { x, y },
      render,
    } = text;

    if !render {
      return 0;
    }

    // render text or get from cache
    let key = data.key();
    let (surface_ptr, width) = match self.prev.get(&key) {
      Some((ptr, width)) => (ptr.to_owned() as *mut sdl::SDL_Surface, width.to_owned()),
      None => {
        let Data { str, fg, bg } = data;
        let bg = if bg.r == 0 && bg.g == 0 && bg.b == 0 || bg.r == 160 && bg.g == 160 && bg.b == 160 {
          None
        } else {
          Some(bg)
        };

        let mut font = FONT.write();
        let (ptr, width) = font.render(str, fg, bg);
        let ptr = ptr as *mut sdl::SDL_Surface;
        mem::drop(font);

        (ptr, width)
      }
    };
    self.next.insert(key, (surface_ptr as usize, width));

    // render on canvas
    unsafe {
      let mut srcrect = Rect::new(0, 0, width, CJK_FONT_SIZE);
      let mut dstrect = Rect::new(x, y, width, CJK_FONT_SIZE);
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_UpperBlit(surface_ptr, srcrect.raw_mut(), canvas, dstrect.raw_mut());
    };

    (width as f32 / CANVAS_FONT_WIDTH as f32).ceil() as usize
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

  pub fn add_cover(&mut self, cover: Cover) {
    let Cover {
      coord: df::common::Coord { x, y },
      dimension: df::common::Dimension { x: w, y: h },
    } = cover;

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
    let cover_rect = Rect::new(x, y, w, h);
    unsafe { sdl::SDL_FillRect(canvas, cover_rect.raw(), 0) };
  }

  pub fn render(&mut self, renderer: usize) {
    if self.canvas_ptr == 0 {
      return;
    }

    let srcrect = Rect::new(
      0,
      0,
      (self.dimension.x * CANVAS_FONT_WIDTH) as u32,
      (self.dimension.y * CANVAS_FONT_HEIGHT) as u32,
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
      (self.dimension.x * dispx_z) as u32,
      (self.dimension.y * dispy_z) as u32,
    );

    unsafe {
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      let sdl_renderer = df::renderer::deref_sdl_renderer(renderer);
      let texture = sdl::SDL_CreateTextureFromSurface(sdl_renderer, canvas);
      sdl::SDL_SetTextureScaleMode(texture, sdl::SDL_ScaleMode::SDL_ScaleModeLinear);
      sdl::SDL_RenderCopy(sdl_renderer, texture, srcrect.raw(), dstrect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
