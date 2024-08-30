use std::{collections::HashMap, mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{
  config::CONFIG,
  enums::ScreenTexPosFlag,
  font::{CJK_FONT_SIZE, FONT},
  raw,
};

const CANVAS_FONT_WIDTH: i32 = 8 * 2;
const CANVAS_FONT_HEIGHT: i32 = 12 * 2;

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new();

#[repr(C)]
struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
}

#[derive(Debug)]
#[repr(C)]
struct ColorInfo {
  pub screenf: u8,
  pub screenb: u8,
  pub screenbright: bool,
  pub use_old_16_colors: bool,
  pub screen_color_r: u8,
  pub screen_color_g: u8,
  pub screen_color_b: u8,
  pub screen_color_br: u8,
  pub screen_color_bg: u8,
  pub screen_color_bb: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorTuple {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub br: u8,
  pub bg: u8,
  pub bb: u8,
}

#[derive(Default)]
pub struct Screen {
  dimension: (u32, u32),
  canvas_ptr: usize,
  // cache: (content, color) => surface_ptr
  prev: HashMap<(String, ColorTuple), (usize, u32)>,
  next: HashMap<(String, ColorTuple), (usize, u32)>,
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

  pub fn add(&mut self, gps: usize, x: i32, y: i32, content: String, sflag: u32) -> usize {
    let color_base = gps + 0x8c; // TODO: check Windows offset
    let color = raw::deref::<ColorInfo>(color_base);
    let color = if color.use_old_16_colors {
      let fg = (color.screenf + if color.screenbright { 8 } else { 0 }) as usize;
      let bg = color.screenb as usize;
      let uccolor_base = color_base + 0xcc;
      ColorTuple {
        r: raw::deref::<u8>(uccolor_base + fg * 3 + 0),
        g: raw::deref::<u8>(uccolor_base + fg * 3 + 1),
        b: raw::deref::<u8>(uccolor_base + fg * 3 + 2),
        br: raw::deref::<u8>(uccolor_base + bg * 3 + 0),
        bg: raw::deref::<u8>(uccolor_base + bg * 3 + 1),
        bb: raw::deref::<u8>(uccolor_base + bg * 3 + 2),
      }
    } else {
      ColorTuple {
        r: color.screen_color_r,
        g: color.screen_color_g,
        b: color.screen_color_b,
        br: color.screen_color_br,
        bg: color.screen_color_bg,
        bb: color.screen_color_bb,
      }
    };

    // early return: we only renders the bottom half by shift up by half font height
    if ScreenTexPosFlag::from_bits_retain(sflag).contains(ScreenTexPosFlag::TOP_OF_TEXT) {
      // TODO: may need to return actual width?
      return 0;
    }

    // render text or get from cache
    let key = (content.clone(), color);
    let (surface_ptr, width) = match self.prev.get(&key) {
      Some((ptr, width)) => (ptr.to_owned() as *mut sdl::SDL_Surface, width.to_owned()),
      None => {
        let mut font = FONT.write();
        let (ptr, width) = font.render(content);
        let ptr = ptr as *mut sdl::SDL_Surface;
        mem::drop(font);

        unsafe { sdl::SDL_SetSurfaceColorMod(ptr, color.r, color.g, color.b) };

        (ptr, width)
      }
    };
    self.next.insert(key, (surface_ptr as usize, width));

    // calculate render offset
    let x = CANVAS_FONT_WIDTH * x;
    let mut y: i32 = CANVAS_FONT_HEIGHT as i32 * y;
    if ScreenTexPosFlag::from_bits_retain(sflag).contains(ScreenTexPosFlag::BOTTOM_OF_TEXT) {
      // shift up by half font height for bottom half
      y -= CANVAS_FONT_HEIGHT / 2;
    }

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
    let screen =
      raw::deref::<ScreenInfo>(renderer + CONFIG.offset.as_ref().unwrap().renderer_offset_screen_info.unwrap());

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
