use std::{mem, ptr};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};

use crate::{font::FONT, raw};

#[static_init::dynamic]
pub static mut SCREEN: Screen = Screen::new();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Screen::new();

pub struct Text {
  x: i32,
  y: i32,
  content: String,
  flag: u32,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenInfo {
  pub dispx_z: i32,
  pub dispy_z: i32,
  pub origin_x: i32,
  pub origin_y: i32,
  pub cur_w: i32,
  pub cur_h: i32,
}

#[derive(Default)]
pub struct Screen {
  info: ScreenInfo,
  surface_size: (u32, u32),
  surface_ptr: usize,
  texts: Vec<Text>,
}

impl Screen {
  fn new() -> Self {
    let mut screen: Screen = Default::default();

    let displays_count = unsafe { sdl::SDL_GetNumVideoDisplays() };
    for i in 0..displays_count {
      unsafe {
        let mut out = mem::MaybeUninit::uninit();
        if sdl::SDL_GetDisplayBounds(i, out.as_mut_ptr()) == 0 {
          let rect = Rect::from_ll(out.assume_init());
          if rect.width() > screen.surface_size.0 {
            screen.surface_size.0 = rect.width()
          }
          if rect.height() > screen.surface_size.1 {
            screen.surface_size.1 = rect.height()
          }
        }
      };
    }
    let surface = Surface::new(screen.surface_size.0, screen.surface_size.1, PixelFormatEnum::RGBA32).unwrap();
    screen.surface_ptr = surface.raw() as usize;
    mem::forget(surface);

    screen
  }

  // pub fn resize(&mut self, x: u32, y: u32) {
  //   if self.surface_ptr != 0 {
  //     let surface = unsafe { Surface::from_ll(self.surface_ptr as *mut sdl::SDL_Surface) };
  //     mem::drop(surface);
  //   }

  //   let surface = Surface::new(x, y, PixelFormatEnum::RGBA32).unwrap();
  //   self.surface_ptr = surface.raw() as usize;
  //   mem::forget(surface);
  // }

  pub fn add(&mut self, x: i32, y: i32, content: String, flag: u32) {
    self.texts.push(Text { x, y, content, flag });
  }

  pub fn clear(&mut self) {
    self.texts.clear();
    unsafe {
      let surface = self.surface_ptr as *mut sdl::SDL_Surface;
      sdl::SDL_FillRect(surface, ptr::null(), 0);
    }
  }

  pub fn render(&mut self, renderer: usize) {
    let surface = self.surface_ptr as *mut sdl::SDL_Surface;

    let new_info = raw::deref::<ScreenInfo>(renderer + 0x160);
    if self.info != new_info {
      log::debug!("new_info: {:?}", new_info);
      self.info = new_info;
    }

    for text in &self.texts {
      let mut x = self.info.dispx_z * text.x + self.info.origin_x;
      let y = self.info.dispy_z * text.y + self.info.origin_y;
      text.content.chars().for_each(|ch| {
        let unicode = ch as u16;
        if unicode < 256 {
          return;
        }

        unsafe {
          let glyph_surface = FONT.write().render(unicode) as *mut sdl::SDL_Surface;
          let mut rect = Rect::new(x, y, self.info.dispy_z as u32, self.info.dispy_z as u32);
          sdl::SDL_UpperBlitScaled(glyph_surface, ptr::null(), surface, rect.raw_mut());
        }
        x += self.info.dispy_z;
      });
    }

    unsafe {
      let sdl_renderer = raw::deref(renderer + 0x108);
      let texture = sdl::SDL_CreateTextureFromSurface(sdl_renderer, surface);
      let rect = Rect::new(0, 0, self.info.cur_w as u32, self.info.cur_h as u32);
      sdl::SDL_RenderCopy(sdl_renderer, texture, rect.raw(), rect.raw());
      sdl::SDL_DestroyTexture(texture);
    }
  }
}
