use std::{
  collections::HashMap,
  hash::{DefaultHasher, Hash, Hasher},
  mem, ptr,
};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{
  df,
  font::{CJK_FONT_SIZE, FONT},
};

pub const CANVAS_FONT_WIDTH: i32 = 8 * 2;
pub const CANVAS_FONT_HEIGHT: i32 = 12 * 2;

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new();

pub struct ScreenText {
  coord: df::common::Coord<i32>,
  data: ColoredText,
  render: bool,
}

impl ScreenText {
  pub fn new(content: String) -> Self {
    Self {
      coord: Default::default(),
      data: ColoredText::new(content),
      render: true,
    }
  }

  pub fn by_coord(mut self, coord: df::common::Coord<i32>) -> Self {
    self.coord = coord;
    self
  }

  pub fn by_graphic(self, gps: usize) -> Self {
    self.color_by_graphic(gps).coord_by_graphic(gps)
  }

  pub fn color_by_graphic(mut self, gps: usize) -> Self {
    let color = df::graphic::deref_color(gps);
    self.data = self.data.with_color(color);
    self
  }

  pub fn coord_by_graphic(self, gps: usize) -> Self {
    let mut coord = df::graphic::deref_coord(gps);
    coord.x *= CANVAS_FONT_WIDTH;
    coord.y *= CANVAS_FONT_HEIGHT;
    self.by_coord(coord)
  }

  pub fn with_offset(mut self, offset_x: i32, offset_y: i32) -> Self {
    self.coord.x += offset_x;
    self.coord.y += offset_y;
    self
  }

  pub fn with_sflag(mut self, sflag: u32) -> Self {
    let flag = df::flags::ScreenTexPosFlag::from_bits_retain(sflag);

    if flag.contains(df::flags::ScreenTexPosFlag::TOP_OF_TEXT) {
      self.render = false;
    }

    if flag.contains(df::flags::ScreenTexPosFlag::BOTTOM_OF_TEXT) {
      self.coord.y -= CANVAS_FONT_HEIGHT / 2
    }

    self
  }

  pub fn with_color(mut self, color: df::common::Color) -> Self {
    self.data = self.data.with_color(color);
    self
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColoredText {
  pub content: String,
  pub color: df::common::Color,
}

impl ColoredText {
  pub fn new(content: String) -> Self {
    Self {
      content,
      color: df::common::Color::rgb(255, 255, 255),
    }
  }

  pub fn with_color(mut self, color: df::common::Color) -> Self {
    self.color = color;
    self
  }

  fn key(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }
}

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
      w * CANVAS_FONT_WIDTH as u32,
      h * CANVAS_FONT_HEIGHT as u32,
      PixelFormatEnum::RGBA32,
    )
    .unwrap();
    self.canvas_ptr = canvas.raw() as usize;
    mem::forget(canvas);
  }

  pub fn add_text(&mut self, text: ScreenText) -> usize {
    let ScreenText {
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

  pub fn render(&mut self, renderer: usize) {
    if self.canvas_ptr == 0 {
      return;
    }

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
    let sdl_renderer = df::renderer::deref_sdl_renderer(renderer);

    let srcrect = Rect::new(
      0,
      0,
      self.dimension.0 * CANVAS_FONT_WIDTH as u32,
      self.dimension.1 * CANVAS_FONT_HEIGHT as u32,
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
