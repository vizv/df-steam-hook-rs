use bitflags::bitflags;

bitflags! {
  pub struct ScreenTexPosFlag: u32 {
    const TOP_OF_TEXT    = 0b00001000;
    const BOTTOM_OF_TEXT = 0b00010000;
  }
}
