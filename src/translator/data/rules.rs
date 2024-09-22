use std::collections::HashMap;

use indexmap::IndexMap;
use regex::Regex;

use super::{placeholder, transformer};

// TODO: move to a separate mod
#[derive(Debug, serde::Deserialize)]
pub struct Special {
  key: String,
  field: String,
  word: String,
  word_translation: String,
}

// TODO: move to a separate mod
#[derive(Debug, serde::Deserialize)]
pub struct Rule {
  rule: String,
  source: String,
  target: String,
  match_word: String,
  build_word: String,
  match_translation: String,
  build_translation: String,
}

#[static_init::dynamic]
static RULE_REGEX: Regex = Regex::new(r"^[A-Z]{3}$").unwrap();

pub type RuleContext<'a> = IndexMap<&'a str, (String, String)>;

#[derive(Debug, Default)]
pub struct RuleSet {
  // (target, rule) => (source, word_transformer, translation_transformer)
  rules: HashMap<(String, String), (String, transformer::Transformer, transformer::Transformer)>,
  // (key, field) => (word, word_translation)
  specials: HashMap<(String, String), (String, String)>,
}

impl RuleSet {
  pub fn insert_rule(&mut self, rule: Rule) {
    let Rule {
      rule,
      source,
      target,
      match_word: word_match,
      build_word: word_generate,
      match_translation: translation_match,
      build_translation: translation_generate,
    } = rule;
    self.rules.insert(
      (target, rule),
      (
        source,
        transformer::Transformer::new(
          placeholder::Placeholder::new(&word_match),
          placeholder::Placeholder::new(&word_generate),
        ),
        transformer::Transformer::new(
          placeholder::Placeholder::new(&translation_match),
          placeholder::Placeholder::new(&translation_generate),
        ),
      ),
    );
  }

  pub fn insert_special(&mut self, special: Special) {
    let Special {
      key,
      field,
      word,
      word_translation,
    } = special;
    self.specials.insert((key, field), (word, word_translation));
  }

  pub fn process(&self, key: &str, context: &mut RuleContext) {
    for field in context.keys().cloned().collect::<Vec<&str>>() {
      let (rule, _) = context.get(field).unwrap();
      let rule = rule.to_owned();

      if rule == "IGN" || !RULE_REGEX.is_match(&rule) {
        continue;
      }

      if rule == "SPL" {
        if let Some((word, translated)) = self.specials.get(&(key.to_owned(), field.to_owned())) {
          context.insert(field, (word.to_owned(), translated.to_owned()));
        } else {
          panic!("cannot find special form for field {field:?} of {key:?}");
        }
        continue;
      }

      if let Some((source, word_transformer, translation_transformer)) =
        self.rules.get(&(field.to_owned(), rule.to_owned()))
      {
        let source = source.to_owned();
        let (word, translated) = context.get(source.as_str()).unwrap();
        let word = word_transformer.transform(word).unwrap();
        let translated = translation_transformer.transform(translated).unwrap();
        context.insert(field, (word, translated));
      } else {
        panic!("unknown rule {rule:?}");
      }
    }
  }
}
