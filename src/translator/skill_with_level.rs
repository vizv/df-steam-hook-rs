use super::{data, matcher};

pub fn translate_skill_with_level(string: &String) -> Option<String> {
  let skill_levels_matcher = matcher::word_matcher(&data::SKILL_LEVELS.adjectives);
  let skill_names_matcher = matcher::word_matcher(&data::SKILL_NAMES.nouns);

  let remaining = string;
  for (remaining, skill_level_translated) in skill_levels_matcher(remaining) {
    for (remaining, skill_name_translated) in skill_names_matcher(remaining) {
      if remaining.is_empty() {
        let translated = vec![skill_level_translated, skill_name_translated].concat();
        return Some(translated);
      }
    }
  }

  None
}
