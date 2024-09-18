use std::hash::{DefaultHasher, Hash, Hasher};

use crate::df;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
  pub str: String,
  pub fg: df::common::Color,
  pub bg: df::common::Color,
}

impl Data {
  pub fn new(str: String) -> Self {
    Self {
      str,
      fg: df::common::Color::rgb(255, 255, 255),
      bg: df::common::Color::rgb(0, 0, 0),
    }
  }

  pub fn with_fg_color(mut self, color: df::common::Color) -> Self {
    self.fg = color;
    self
  }

  pub fn with_bg_color(mut self, color: df::common::Color) -> Self {
    self.bg = color;
    self
  }

  pub fn key(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }
}
