use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::utils;

mod data;
mod matcher;

mod context;

mod interface;
mod item_name;
mod skill_with_level;
mod version;
use interface::translate_interface;
use item_name::translate_item_name;
use skill_with_level::translate_skill_with_level;
use version::translate_version;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StringWithContext<'a> {
  pub func: &'static str,
  pub bt: &'a str,
  pub view_opt: Option<&'a str>,
  pub string: &'a str,
}

impl<'a> StringWithContext<'a> {
  pub fn key(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }
}

#[derive(Default)]
pub struct TranslatedText {
  pub text: String,
  pub horizontal_shift: i32,
}

#[derive(Default)]
pub struct Translator {
  cache: HashMap<u64, TranslatedText>,
}

impl Translator {
  pub fn translate<'a>(&'a mut self, func: &'static str, string: &'a String, bt: &str) -> (&'a str, i32) {
    if string.starts_with("FPS: ") {
      return (string, 0);
    }

    let view_opt = utils::get_view();
    let view_opt = view_opt.as_deref();
    let key = StringWithContext {
      func,
      bt,
      view_opt,
      string,
    }
    .key();

    let mut is_legacy = false;
    if !self.cache.contains_key(&key) {
      let location_opt = context::get_context_location(view_opt, bt);
      let lower_string = &string.to_lowercase();
      let (text, horizontal_shift) = if let Some(translated) = translate_version(view_opt, string) {
        (translated, 0)
      } else if let Some(translation_tuple) = translate_interface(view_opt, location_opt, string) {
        translation_tuple
      } else if let Some(translated) = data::HELP.get(string) {
        (translated.to_owned(), 0)
      } else if let Some(translated) = translate_skill_with_level(lower_string) {
        (translated, 0)
      } else if let Some(translated) = translate_item_name(lower_string) {
        (translated, 0)
      } else if let Some(translated) = matcher::match_workshop_string(lower_string) {
        (translated, 0)
      } else if let Some(translated) = data::MEGA.get(lower_string) {
        (translated.to_owned(), 0)
      } else if let Some(translated) = data::LEGACY.get(lower_string) {
        is_legacy = true;
        (translated.to_owned(), 0)
      } else {
        (string.to_owned(), 0)
      };

      if string == &text {
        log::debug!("missing translation for {func}:\n{view_opt:?}/{location_opt:?} @ {bt}:\n{string:?}\n");
      } else {
        if is_legacy {
          log::warn!(
            "use legacy translation for {func}:\n{view_opt:?}/{location_opt:?} @ {bt}:\n- {string:?}\n+ {text:?}\n"
          );
        } else {
          log::trace!("found translation for {func}:\n{view_opt:?}/{location_opt:?} @ {bt}:\n- {string:?}\n+ {text:?}\n");
        }
      }
      self.cache.insert(key, TranslatedText { text, horizontal_shift });
    }

    let TranslatedText { text, horizontal_shift } = self.cache.get(&key).unwrap();
    (text, *horizontal_shift)
  }
}

#[static_init::dynamic]
pub static mut TRANSLATOR: Translator = Translator::default();
