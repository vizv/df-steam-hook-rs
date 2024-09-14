use super::super::data;
use super::common;

pub fn match_skill_level(string: &String) -> Option<String> {
  let (matched, translated_opt) = common::match_dictionary(
    &data::SKILL_LEVELS.adjectives,
    string,
    Some(|remaining| common::match_dictionary(&data::SKILL_NAMES.nouns, remaining, None)),
  );
  if matched {
    if let Some(translated) = translated_opt {
      return Some(translated);
    }
  }
  None
}
