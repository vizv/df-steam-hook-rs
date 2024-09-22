use std::collections::HashMap;

use crate::utils;

use super::{dictionary, wildcard_table};

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
struct ItemWildcard {
  wildcard: String,
  wildcard_translation: String,
  use_noun_for_adj: String,
  use_standard_plural: String,
}

#[derive(Debug, Default)]
pub struct Items {
  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
  pub wildcard_table: wildcard_table::WildcardTable,
  pub should_use_noun_for_adj: HashMap<(String, String), bool>,
}

impl Items {
  fn new() -> Self {
    let mut items = Items::default();
    utils::load_csv(
      utils::translations_path("items.csv"),
      |Item {
         noun_single,
         noun_plural,
         noun_translation,
         adjective,
         adjective_translation,
       }| {
        items.nouns.insert(noun_single, noun_translation.clone());
        items.nouns.insert(noun_plural, noun_translation);
        items.adjectives.insert(adjective, adjective_translation);
      },
    );
    utils::load_csv(
      utils::translations_path("items-builtin.csv"),
      |ItemWildcard {
         mut wildcard,
         wildcard_translation,
         use_noun_for_adj,
         use_standard_plural,
       }| {
        if wildcard.contains("{}") {
          if let Some(key) = items.wildcard_table.insert(wildcard.clone(), wildcard_translation.clone()) {
            items.should_use_noun_for_adj.insert(key, !use_noun_for_adj.is_empty());
          }
          if !use_standard_plural.is_empty() {
            wildcard.push('s');
            if let Some(key) = items.wildcard_table.insert(wildcard, wildcard_translation) {
              items.should_use_noun_for_adj.insert(key, !use_noun_for_adj.is_empty());
            }
          }
        } else {
          items.nouns.insert(wildcard.clone(), wildcard_translation.clone());
          if !use_standard_plural.is_empty() {
            wildcard.push('s');
            items.nouns.insert(wildcard, wildcard_translation);
          }
        }
      },
    );
    items
  }
}
