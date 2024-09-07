use std::fs::File;

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
    let mut skill_levels: SkillLevels = Default::default();

    let file = File::open("./dfint-data/translations/skill_levels.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let SkillLevel {
        adjective,
        adjective_translation,
      } = result.unwrap();

      skill_levels.adjectives.insert(adjective, adjective_translation);
    }

    skill_levels
  }
}
