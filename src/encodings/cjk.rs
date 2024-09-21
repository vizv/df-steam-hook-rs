use super::cp437;

pub fn is_cjk(ch: char) -> bool {
  cp437::utf8_char_to_ch437_byte(ch).is_none()
}

pub fn is_cjk_punctuation(ch: char) -> bool {
  match ch {
    '。' | '，' | '？' | '！' => true,
    _ => false,
  }
}
