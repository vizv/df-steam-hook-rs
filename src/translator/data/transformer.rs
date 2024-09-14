use super::placeholder;

#[derive(Debug, Default)]
pub struct Transformer {
  match_placeholder: placeholder::Placeholder,
  build_placeholder: placeholder::Placeholder,
}

impl Transformer {
  pub fn new(match_placeholder: placeholder::Placeholder, build_placeholder: placeholder::Placeholder) -> Self {
    Self {
      match_placeholder,
      build_placeholder,
    }
  }

  pub fn transform(&self, string: &str) -> Option<String> {
    Some(self.build_placeholder.wrap(self.match_placeholder.unwrap(string)?))
  }
}
