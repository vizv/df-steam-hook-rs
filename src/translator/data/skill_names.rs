use crate::utils;

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
    let mut ret = SkillNames::default();
    utils::load_csv(
      utils::translations_path("skill_names.csv"),
      |SkillName {
         noun,
         noun_translation,
         noun_dwarf_single,
         noun_dwarf_plural,
         noun_dwarf_translation,
       }| {
        ret.nouns.insert(noun, noun_translation.clone());
        ret.nouns.insert(noun_dwarf_single, noun_dwarf_translation.clone());
        ret.nouns.insert(noun_dwarf_plural, noun_dwarf_translation);
      },
    );
    ret
  }
}
