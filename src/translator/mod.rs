use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use data::MEGA;
use item_name::translate_item_name;
use skill_with_level::translate_skill_with_level;

use crate::utils;

mod data;
mod matcher;

mod item_name;
mod skill_with_level;

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
  pub fn translate<'a>(&'a mut self, context: &'static str, string: &'a String) -> &'a String {
    if string.starts_with("FPS: ") {
      return string;
    }

    MEGA.write().load();

    let key = StringWithContext { context, string }.key();
    if !self.cache.contains_key(&key) {
      let lower_string = &string.to_lowercase();
      let content = if let Some(translated) = data::HELP.get(string) {
        translated.to_owned()
      } else if let Some(translated) = translate_skill_with_level(lower_string) {
        translated
      } else if let Some(translated) = translate_item_name(lower_string) {
        translated
      } else if let Some(translated) = matcher::match_workshop_string(lower_string) {
        translated
      } else if let Some(translated) = data::MEGA.read().get(lower_string) {
        translated.to_owned()
      } else {
        string.to_owned()
      };

      if string == &content {
        let bt = utils::backtrace();
        log::warn!("missing translation for {context} @ {bt}: 「{string}」");
      } else {
        // log::debug!("translate for {context}: {string} -> {content}");
      }
      self.cache.insert(key, content);
    }

    self.cache.get(&key).unwrap()
  }
}

#[static_init::dynamic]
pub static mut TRANSLATOR: Translator = Translator::default();
