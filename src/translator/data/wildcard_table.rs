use std::ops::Deref;

use indexmap::IndexMap;

use super::dictionary;

#[derive(Debug, Default)]
pub struct WildcardTable {
  pub dict: IndexMap<String, dictionary::Dictionary>,
  pub max_count: usize,
}

impl WildcardTable {
  pub fn insert(&mut self, wildcard: String, wildcard_translation: String) -> Option<(String, String)> {
    if wildcard.is_empty() || wildcard_translation.is_empty() {
      return None;
    }

    let pair: Vec<&str> = wildcard.split("{}").collect();
    if pair.len() != 2 {
      return None;
    }
    let prefix = pair[0].trim_end();
    let suffix = pair[1].trim_start();

    if !self.dict.contains_key(prefix) {
      let count = prefix.split(" ").count();
      if count > self.max_count {
        self.max_count = count;
      }

      self.dict.insert(prefix.to_string(), Default::default());
    }
    let dict = self.dict.get_mut(prefix).unwrap();

    let count = suffix.split(" ").count();
    if count > dict.max_count {
      dict.max_count = count;
    }

    dict.insert(suffix.to_string(), wildcard_translation);

    return Some((prefix.to_string(), suffix.to_string()));
  }
}

impl Deref for WildcardTable {
  type Target = IndexMap<String, dictionary::Dictionary>;

  fn deref(&self) -> &Self::Target {
    &self.dict
  }
}
