use super::cjk;

pub const CURSES_FONT_WIDTH: u32 = 16;
pub const CJK_FONT_WIDTH: u32 = 24;

pub fn string_width_in_pixels(string: &str) -> u32 {
  string.chars().into_iter().map(char_width_in_pixels).sum()
}

pub fn char_width_in_pixels(ch: char) -> u32 {
  if cjk::is_cjk(ch) {
    CJK_FONT_WIDTH
  } else {
    CURSES_FONT_WIDTH
  }
}
