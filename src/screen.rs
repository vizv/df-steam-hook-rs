use std::{mem, ptr};

use sdl2::{
  pixels::{Color, PixelFormatEnum},
  rect::Rect,
  surface::Surface,
  sys as sdl,
};

use crate::{font::FONT, raw};

const CANVAS_FONT_WIDTH: i32 = 8 * 2;
const CANVAS_FONT_HEIGHT: i32 = 12 * 2;

// TODO: consider to use bitflags crate
#[allow(dead_code, non_camel_case_types)]
#[repr(u32)]
enum ScreenTexPosFlag {
  None = 0,
  GRAYSCALE = 0b1,
  ADDCOLOR = 0b10,
  ANCHOR_SUBORDINATE = 0b100,
  TOP_OF_TEXT = 0b1000,
  BOTTOM_OF_TEXT = 0b10000,
  ANCHOR_USE_SCREEN_COLOR = 0b100000,
  ANCHOR_X_COORD = 0b111111000000,
  X_COORD_SHIFT = 6,
  ANCHOR_Y_COORD = 0b111111000000000000,
  ANCHOR_Y_COORD_SHIFT = 12,
}

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new(false);

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new(true);

#[repr(C)]
struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ScreenChar {
  pub ch: u8,
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub br: u8,
  pub bg: u8,
  pub bb: u8,
  pub _reserved: u8,
}

#[derive(Default)]
pub struct Screen {
  is_top: bool,
  dimension: (u32, u32),
  canvas_ptr: usize,
}

impl Screen {
  pub fn new(is_top: bool) -> Self {
    Self {
      is_top,
      dimension: Default::default(),
      canvas_ptr: Default::default(),
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

  pub fn add(&mut self, gps: usize, x: i32, y: i32, content: String, width: usize) {
    let offset = (x * self.dimension.1 as i32 + y) as usize;
    let screen_offset = if self.is_top { 0x228 } else { 0x1e0 };

    let screen_base = raw::deref::<usize>(gps + screen_offset);
    let sc = raw::deref::<ScreenChar>(screen_base + offset * 8);

    let flag_base = raw::deref::<usize>(gps + screen_offset + 0x38);
    let flag = raw::deref::<u32>(flag_base + offset * 4);

    if flag & ScreenTexPosFlag::BOTTOM_OF_TEXT as u32 != 0 {
      // we only renders the top half
      return;
    }

    let x = CANVAS_FONT_WIDTH * x;
    let mut y = CANVAS_FONT_HEIGHT as i32 * y;
    if flag & ScreenTexPosFlag::TOP_OF_TEXT as u32 != 0 {
      // shift down by half font height
      y += CANVAS_FONT_HEIGHT / 2;
    }

    unsafe {
      let mut font = FONT.write();
      let width = (width * CANVAS_FONT_WIDTH as usize) as u32;
      let mut surface = font.render(content, width);
      let mut rect = Rect::new(x, y, surface.width(), surface.height());
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      surface.set_color_mod(Color::RGB(sc.r, sc.g, sc.b));

      sdl::SDL_UpperBlit(surface.raw(), ptr::null(), canvas, rect.raw_mut());
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
    let screen = raw::deref::<ScreenInfo>(renderer + 0x160);

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
        screen.origin_x as i32,
        screen.origin_y as i32,
        self.dimension.0 * screen.dispx_z as u32,
        self.dimension.1 * screen.dispy_z as u32,
      );
      sdl::SDL_RenderCopy(sdl_renderer, texture, srcrect.raw(), dstrect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
