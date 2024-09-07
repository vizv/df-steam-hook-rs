use std::fs::File;

use super::dictionary;

#[static_init::dynamic]
pub static ITEMS: Items = Items::new();

#[derive(Debug, serde::Deserialize)]
struct Item {
  noun_single: String,
  noun_plural: String,
  noun_translation: String,
  adjective: String,
  adjective_translation: String,
}

#[derive(Debug, serde::Deserialize)]
struct ItemOther {
  noun: String,
  noun_translation: String,
}

#[derive(Debug, Default)]
pub struct Items {
  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
}

impl Items {
  fn new() -> Self {
    let mut items: Items = Default::default();

    let file = File::open("./dfint-data/translations/items.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let Item {
        noun_single,
        noun_plural,
        noun_translation,
        adjective,
        adjective_translation,
      } = result.unwrap();

      items.nouns.insert(noun_single, noun_translation.clone());
      items.nouns.insert(noun_plural, noun_translation);
      items.adjectives.insert(adjective, adjective_translation);
    }

    let file = File::open("./dfint-data/translations/items-others.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let ItemOther { noun, noun_translation } = result.unwrap();
      items.nouns.insert(noun, noun_translation);
    }

    items
  }
}
