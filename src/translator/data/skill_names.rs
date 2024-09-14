use std::fs::File;

use super::dictionary;

#[static_init::dynamic]
pub static SKILL_NAMES: SkillNames = SkillNames::new();

#[derive(Debug, serde::Deserialize)]
struct SkillName {
  noun: String,
  noun_translation: String,
  noun_dwarf_single: String,
  noun_dwarf_plural: String,
  noun_dwarf_translation: String,
}

#[derive(Debug, Default)]
pub struct SkillNames {
  pub nouns: dictionary::Dictionary,
}

impl SkillNames {
  fn new() -> Self {
    let mut skill_names = SkillNames::default();

    let file = File::open("./dfint-data/translations/skill_names.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let SkillName {
        noun,
        noun_translation,
        noun_dwarf_single,
        noun_dwarf_plural,
        noun_dwarf_translation,
      } = result.unwrap();

      skill_names.nouns.insert(noun, noun_translation.clone());
      skill_names.nouns.insert(noun_dwarf_single, noun_dwarf_translation.clone());
      skill_names.nouns.insert(noun_dwarf_plural, noun_dwarf_translation);
    }

    skill_names
  }
}
