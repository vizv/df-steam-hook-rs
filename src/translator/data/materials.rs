use std::{collections::HashMap, fs::File};

use super::dictionary;

#[static_init::dynamic]
pub static MATERIALS: Materials = Materials::new();

#[derive(Debug, serde::Deserialize)]
struct MaterialGenerationRule {
  rule: String,
  state: String,
  prefix: String,
  suffix: String,
  template: String,
}

#[derive(Debug, serde::Deserialize)]
struct MaterialNoun {
  rules: String,
  noun: String,
  noun_translation: String,
}

#[derive(Debug, serde::Deserialize)]
struct MaterialAdjective {
  source_noun: String,
  adjective: String,
  adjective_translation_override: String,
}

#[derive(Debug, Default)]
pub struct Materials {
  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
}

impl Materials {
  fn new() -> Self {
    let mut materials = Materials::default();
    let mut rule_set: HashMap<String, (String, String, String, String)> = HashMap::new();

    let file = File::open("./dfint-data/translations/materials-generation-rules.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let MaterialGenerationRule {
        rule,
        state,
        prefix,
        suffix,
        template,
      } = result.unwrap();

      rule_set.insert(rule, (state, prefix, suffix, template));
    }

    let file = File::open("./dfint-data/translations/materials-nouns.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let MaterialNoun {
        rules,
        noun,
        noun_translation,
      } = result.unwrap();

      if noun_translation.is_empty() {
        continue;
      }

      materials.nouns.insert(noun.clone(), noun_translation.clone());
      for rule in rules.split('/').into_iter() {
        if rule.is_empty() {
          continue;
        }

        let (state, prefix, suffix, template) = &rule_set[rule];
        if state == "plural" {
          let mut noun = noun.clone();
          noun.push('s');
          materials.nouns.insert(noun, noun_translation.clone());
          continue;
        }

        let mut buf = prefix.clone();
        if !buf.is_empty() {
          buf.push(' ');
        }
        buf.push_str(&noun);
        if !suffix.is_empty() {
          buf.push_str(suffix);
        }
        let noun = buf;
        let translation = template.replace("{}", &noun_translation);
        materials.nouns.insert(noun, translation);
      }
    }

    let file = File::open("./dfint-data/translations/materials-adjectives.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let MaterialAdjective {
        source_noun,
        adjective,
        adjective_translation_override,
      } = result.unwrap();

      let source_noun = if source_noun.is_empty() {
        adjective.clone()
      } else {
        source_noun
      };

      if adjective_translation_override.is_empty() {
        if let Some(noun_translation) = materials.nouns.get(&source_noun) {
          materials.adjectives.insert(adjective, noun_translation.clone());
        }
      } else {
        materials.adjectives.insert(adjective, adjective_translation_override);
      };
    }

    // println!("{materials:#?}");
    // println!("{:#?}", materials.adjectives);

    // TODO:
    // materials.nouns.insert(noun_stone, noun_stone_translation);
    // materials.nouns.insert(noun_block_single, noun_block_translation.clone());
    // materials.nouns.insert(noun_block_plural, noun_block_translation);

    materials
  }
}
