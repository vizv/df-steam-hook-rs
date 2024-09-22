use std::ops::Deref;

use indexmap::IndexMap;

use super::{ITEMS, MATERIALS, MATERIALS_TEMPLATES, PLANTS, SKILL_LEVELS, SKILL_NAMES};

#[static_init::dynamic]
pub static MEGA: MegaDictionary = MegaDictionary::new();

#[derive(Debug, Default)]
pub struct MegaDictionary {
  pub dict: IndexMap<String, String>,
}

impl MegaDictionary {
  pub fn new() -> Self {
    let mut ret = Self::default();
    let dicts = vec![
      &PLANTS.nouns,
      &ITEMS.adjectives,
      &ITEMS.nouns,
      &MATERIALS_TEMPLATES.adjectives,
      &MATERIALS_TEMPLATES.nouns,
      &MATERIALS.adjectives,
      &MATERIALS.nouns,
      &SKILL_LEVELS.adjectives,
      &SKILL_NAMES.nouns,
    ];
    for &dict in dicts.iter() {
      for (k, v) in dict.iter() {
        ret.dict.insert(k.to_owned(), v.to_owned());
      }
    }
    ret
  }
}

impl Deref for MegaDictionary {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.dict
  }
}
