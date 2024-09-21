use crate::offsets;

use super::common;

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
  pub fn borrow(addr: usize) -> &'static Self {
    raw::as_ref(addr + offsets::FIELDS.get("game.main_interface.help").unwrap())
  }

  pub fn borrow_mut(addr: usize) -> &'static mut Self {
    raw::as_ref_mut(addr + offsets::FIELDS.get("game.main_interface.help").unwrap())
  }
}
