use std::{collections::HashMap, mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{font::FONT, raw};

const CANVAS_FONT_WIDTH: i32 = 8 * 2;
const CANVAS_FONT_HEIGHT: i32 = 12 * 2;

// TODO: move to config
#[cfg(target_os = "linux")]
const SCREEN_INFO_OFFSET: usize = 0x160;
#[cfg(target_os = "windows")]
const SCREEN_INFO_OFFSET: usize = 0x168;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
  // cache: (content, sc) => surface_ptr
  prev: HashMap<(String, ScreenChar), usize>,
  next: HashMap<(String, ScreenChar), usize>,
}

impl Screen {
  pub fn new(is_top: bool) -> Self {
    Self {
      is_top,
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

  pub fn add(&mut self, gps: usize, x: i32, y: i32, content: String, width: usize) {
    // get screen character struct and flags for this text
    let offset = (x * self.dimension.1 as i32 + y) as usize;
    let screen_offset = if self.is_top { 0x228 } else { 0x1e0 };

    let screen_base = raw::deref::<usize>(gps + screen_offset);
    let sc = raw::deref::<ScreenChar>(screen_base + offset * 8);

    let flag_base = raw::deref::<usize>(gps + screen_offset + 0x38);
    let flag = raw::deref::<u32>(flag_base + offset * 4);

    // early return: we only renders the top half by shift down by half font height
    if flag & ScreenTexPosFlag::BOTTOM_OF_TEXT as u32 != 0 {
      return;
    }

    // render text or get from cache
    let key = (content.clone(), sc);
    let surface_ptr = match self.prev.get(&key) {
      Some(ptr) => ptr.to_owned() as *mut sdl::SDL_Surface,
      None => {
        let mut font = FONT.write();
        let ptr = font.render(content, width as u32 * CANVAS_FONT_WIDTH as u32) as *mut sdl::SDL_Surface;
        mem::drop(font);

        unsafe { sdl::SDL_SetSurfaceColorMod(ptr, sc.r, sc.g, sc.b) };

        ptr
      }
    };
    self.next.insert(key, surface_ptr as usize);

    // calculate render offset
    let x = CANVAS_FONT_WIDTH * x;
    let mut y: i32 = CANVAS_FONT_HEIGHT as i32 * y;
    if flag & ScreenTexPosFlag::TOP_OF_TEXT as u32 != 0 {
      // shift down by half font height for top half
      y += CANVAS_FONT_HEIGHT / 2;
    }

    // render on canvas
    unsafe {
      let mut rect = Rect::new(x, y, (*surface_ptr).w as u32, (*surface_ptr).h as u32);
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_UpperBlit(surface_ptr, ptr::null(), canvas, rect.raw_mut());
    };
  }

  pub fn clear(&mut self) {
    unsafe {
      let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_FillRect(canvas, ptr::null(), 0);
    }

    for (k, v) in self.prev.iter() {
      if !self.next.contains_key(k) {
        unsafe { sdl::SDL_FreeSurface(v.to_owned() as *mut sdl::SDL_Surface) };
      }
    }

    self.prev = mem::take(&mut self.next);
  }

  pub fn render(&mut self, renderer: usize) {
    if self.canvas_ptr == 0 {
      return;
    }

    let canvas = self.canvas_ptr as *mut sdl::SDL_Surface;
    let screen = raw::deref::<ScreenInfo>(renderer + SCREEN_INFO_OFFSET);

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
