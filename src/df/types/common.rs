#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Coord<T> {
  pub x: T,
  pub y: T,
}

impl<T> Coord<T> {
  pub fn read(addr: usize) -> Self {
    raw::read(addr)
  }

  pub fn borrow(addr: usize) -> &'static Self {
    raw::as_ref(addr)
  }
}

pub type Dimension<T> = Coord<T>;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub fn read(addr: usize) -> Self {
    raw::read(addr)
  }

  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector {
  pub begin: usize,
  pub end: usize,
  pub capacity: usize,
}

impl Vector {
  pub fn first_address(&self) -> Option<usize> {
    if self.begin == 0 || self.begin == self.end {
      None
    } else {
      Some(unsafe { *(self.begin as *const usize) })
    }
  }

  pub fn first_mut<T>(&self) -> Option<&'static mut T> {
    self.first_address().map(|addr| unsafe { &mut *(addr as *mut T) })
  }
}
