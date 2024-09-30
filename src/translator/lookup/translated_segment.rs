use std::rc::Rc;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TranslatedSegment {
  // text to be translated
  pub text: Rc<String>,
  // skip first part
  pub skip: usize,
  // pos of remaining
  pub pos: usize,
  // translated text up to pos
  pub translated: String,
}

impl TranslatedSegment {
  pub fn new(text: Rc<String>, skip: usize) -> Self {
    Self {
      text,
      skip,
      ..Default::default()
    }
  }

  pub fn text(&self) -> &str {
    &self.text[self.skip..]
  }

  pub fn prefix(&self) -> &str {
    &self.text()[0..self.pos].trim_end_matches(' ')
  }

  pub fn remaining(&self) -> &str {
    &self.text()[self.pos..]
  }

  pub fn split(&self) -> Option<(&str, &str)> {
    let prefix = self.prefix();
    let remaining = self.remaining();
    if remaining.starts_with(' ') {
      return None;
    }
    Some((prefix, remaining))
  }

  pub fn append(&self, next: &TranslatedSegment) -> TranslatedSegment {
    let mut ret = self.skip(next.prefix().len());
    ret.translated = format!("{}{}", self.translated, next.translated);

    ret
  }

  pub fn next_word(&self) -> String {
    let remaining = self.remaining();
    if let Some(word_len) = remaining.find(|ch| ch == ' ') {
      remaining[0..word_len].to_owned()
    } else {
      remaining.to_owned()
    }
  }

  pub fn skip(&self, len: usize) -> TranslatedSegment {
    let mut ret = self.to_owned();
    if len == 0 {
      return ret;
    }
    let remaining = &self.remaining()[len..];
    let space_len = if remaining.is_empty() { 0 } else { 1 };
    ret.pos += len + space_len;

    ret
  }

  pub fn next(&self) -> (String, TranslatedSegment) {
    let next_word = self.next_word();
    let skip_len = next_word.len();
    (next_word, self.skip(skip_len))
  }
}

pub type TranslatedSegments = Vec<TranslatedSegment>;

impl From<&str> for TranslatedSegment {
  fn from(value: &str) -> Self {
    TranslatedSegment::new(Rc::new(value.to_lowercase()), 0)
  }
}
