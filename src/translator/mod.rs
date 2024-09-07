use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use data::MEGA;
use matcher::match_skill_level;

use crate::dictionary::DICTIONARY;

mod data;
mod matcher;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StringWithContext<'a> {
  pub context: &'static str,
  pub string: &'a String,
}

impl<'a> StringWithContext<'a> {
  pub fn key(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }
}

#[derive(Default)]
pub struct Translator {
  cache: HashMap<u64, String>,
}

impl Translator {
  pub fn translate(&mut self, context: &'static str, string: &String) -> &String {
    MEGA.write().load();

    let key = StringWithContext { context, string }.key();
    if !self.cache.contains_key(&key) {
      let content = if let Some(translated) = matcher::match_workshop_string(string) {
        translated
      } else if let Some(translated) = match_skill_level(string) {
        translated
      } else if let Some(translated) = data::MEGA.read().get(&string.to_lowercase()) {
        translated.to_owned()
      } else if let Some(translated) = DICTIONARY.get(string) {
        translated.to_owned()
      } else {
        string.to_owned()
      };

      // if string == &content {
      //   log::warn!("missing translation for {context}: {string}");
      // } else {
      //   log::debug!("translate for {context}: {string} -> {content}");
      // }
      self.cache.insert(key, content);
    }

    self.cache.get(&key).unwrap()
  }
}

#[static_init::dynamic]
pub static mut TRANSLATOR: Translator = Translator::default();
