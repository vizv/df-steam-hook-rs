use super::{common, offsets};

#[derive(Debug)]
#[repr(C)]
pub struct MarkupTextWord {
  pub str: [u8; 32],
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub link_index: i32,
  pub px: i32,
  pub py: i32,
  pub flags: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MarkupTextBox {
  pub word: common::Vector,
  pub link: common::Vector,
  pub current_width: i32,
  pub max_y: i32,
  pub environment: usize,
}

impl MarkupTextBox {
  pub fn at_mut(addr: usize) -> &'static mut Self {
    unsafe { &mut *(addr as *mut Self) }
  }

  pub fn ptr(&self) -> usize {
    self as *const MarkupTextBox as usize
  }
}

#[derive(Debug)]
#[repr(C)]
pub struct GameMainInterfaceHelp {
  pub open: bool,
  pub flag: u32,
  pub context_flag: u32,
  pub context: u32,
  pub header: [u8; 32],
  pub text: [MarkupTextBox; 20],
}

impl GameMainInterfaceHelp {
  pub fn borrow_at(addr: usize) -> &'static Self {
    unsafe { &*((addr) as *const GameMainInterfaceHelp) }
  }

  pub fn borrow_from(addr: usize) -> &'static Self {
    Self::borrow_at(addr + offsets::GAME_MAIN_INTERFACE_HELP)
  }

  pub fn borrow_mut_at(addr: usize) -> &'static mut Self {
    unsafe { &mut *((addr) as *mut GameMainInterfaceHelp) }
  }

  pub fn borrow_mut_from(addr: usize) -> &'static mut Self {
    Self::borrow_mut_at(addr + offsets::GAME_MAIN_INTERFACE_HELP)
  }
}
