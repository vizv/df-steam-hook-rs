use std::{collections::HashMap, ops::Deref};

use super::{ITEMS, MATERIALS, MATERIALS_TEMPLATES, SKILL_LEVELS, SKILL_NAMES};

#[static_init::dynamic]
pub static mut MEGA: MegaDictionary = Default::default();

#[derive(Debug, Default)]
pub struct MegaDictionary {
  pub dict: HashMap<String, String>,
  loaded: bool,
}

impl MegaDictionary {
  pub fn load(&mut self) {
    if self.loaded {
      return;
    }

    let dicts = vec![
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
        self.dict.insert(k.clone(), v.clone());
      }
    }
  }
}

impl Deref for MegaDictionary {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.dict
  }
}
