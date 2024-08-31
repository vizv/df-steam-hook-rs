use super::{common, offsets, utils};

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
  pub fn deref(addr: usize) -> &'static Self {
    unsafe { &*((addr + offsets::GAME_MAIN_INTERFACE_HELP) as *const GameMainInterfaceHelp) }
  }
}
