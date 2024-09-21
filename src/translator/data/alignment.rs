#[derive(Debug)]
pub enum Alignment {
  LEFT,
  CENTER,
  RIGHT,
}

impl Default for Alignment {
  fn default() -> Self {
    Self::LEFT
  }
}

impl From<&str> for Alignment {
  fn from(value: &str) -> Self {
    Self::new(value).unwrap_or(Self::default())
  }
}

impl Alignment {
  pub fn new(str: &str) -> Option<Self> {
    match str.to_uppercase().as_str() {
      "LEFT" => Some(Self::LEFT),
      "CENTER" => Some(Self::CENTER),
      "RIGHT" => Some(Self::RIGHT),
      _ => None,
    }
  }
}
