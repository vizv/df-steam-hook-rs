use std::hash::{DefaultHasher, Hash, Hasher};

use crate::df;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
  pub str: String,
  pub color: df::common::Color,
}

impl Data {
  pub fn new(str: String) -> Self {
    Self {
      str,
      color: df::common::Color::rgb(255, 255, 255),
    }
  }

  pub fn with_color(mut self, color: df::common::Color) -> Self {
    self.color = color;
    self
  }

  pub fn key(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }
}
