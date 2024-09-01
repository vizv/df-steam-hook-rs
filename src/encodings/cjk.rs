use super::cp437;

pub fn is_cjk(ch: char) -> bool {
  !cp437::UTF8_CHAR_TO_CP437.contains_key(&ch)
}

pub fn is_cjk_punctuation(ch: char) -> bool {
  match ch {
    '。' | '，' | '？' | '！' => true,
    _ => false,
  }
}
