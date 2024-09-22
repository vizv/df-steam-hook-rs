use crate::utils;

use super::dictionary;

#[static_init::dynamic]
pub static SKILL_LEVELS: SkillLevels = SkillLevels::new();

#[derive(Debug, serde::Deserialize)]
struct SkillLevel {
  adjective: String,
  adjective_translation: String,
}

#[derive(Debug, Default)]
pub struct SkillLevels {
  pub adjectives: dictionary::Dictionary,
}

impl SkillLevels {
  fn new() -> Self {
    let mut ret = SkillLevels::default();
    utils::load_csv(
      utils::translations_path("skill_levels.csv"),
      |SkillLevel {
         adjective,
         adjective_translation,
       }| {
        ret.adjectives.insert(adjective, adjective_translation);
      },
    );
    ret
  }
}
