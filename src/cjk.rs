use crate::font::FONT;

pub fn is_cjk(ch: char) -> bool {
  let (_, is_curses) = FONT.write().get(ch);
  !is_curses
}

pub fn is_cjk_punctuation(ch: char) -> bool {
  match ch {
    '。' | '，' | '？' | '！' => true,
    _ => false,
  }
}
