use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Default)]
pub struct Dictionary {
  pub dict: HashMap<String, String>,
  pub max_count: usize,
}

impl Dictionary {
  pub fn insert(&mut self, original: String, translated: String) {
    if original.is_empty() || translated.is_empty() {
      return;
    }

    let count = original.split(" ").count();
    if count > self.max_count {
      self.max_count = count;
    }

    self.dict.insert(original.to_lowercase(), translated);
  }
}

impl Deref for Dictionary {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.dict
  }
}
