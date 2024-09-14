use std::ops::Deref;

use indexmap::IndexMap;

#[derive(Debug, Default)]
pub struct Dictionary {
  pub dict: IndexMap<String, String>,
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
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.dict
  }
}
