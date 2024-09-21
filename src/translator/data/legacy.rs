use std::collections::HashMap;

use crate::{config, utils};

#[derive(Debug, serde::Deserialize)]
struct Entry {
  text: String,
  translation: String,
}

#[derive(Debug, Default)]
pub struct Legacy {
  dict: HashMap<String, String>,
}

impl Legacy {
  pub fn get(&self, key: &String) -> Option<&String> {
    if !config::CONFIG.settings.use_legacy_dictionary {
      return None;
    }

    self.dict.get(key)
  }
}

#[static_init::dynamic]
pub static LEGACY: Legacy = {
  let mut ret = Legacy::default();

  utils::load_csv(
    utils::data_path("legacy-dictionary.csv"),
    |Entry { text, translation }| {
      ret.dict.insert(text.to_lowercase(), translation);
    },
  );

  ret
};
