use super::super::data;
use super::common;

use WorkshopStringMatchingState::*;

#[static_init::dynamic]
static PREFIX: data::Dictionary = {
  let mut dict = data::Dictionary::default();
  dict.insert("assemble".into(), "组装".into());
  dict.insert("forge".into(), "打造".into());
  dict.insert("make".into(), "制作".into());
  dict
};

#[derive(Debug, Clone, Copy)]
enum WorkshopStringMatchingState {
  Init,               // initial state, can match prefix
  PrefixMatched,      // can match item_adj, material_adj, item_noun
  ItemAdjMatched,     // can match material_adj, item_noun
  MaterialAdjMatched, // can match item_noun
  Matched,            // finish state
}

impl Default for WorkshopStringMatchingState {
  fn default() -> Self {
    Self::Init
  }
}

#[derive(Debug, Default, Clone)]
struct WorkshopStringCandidate<'a> {
  state: WorkshopStringMatchingState,
  remaining: &'a str,

  prefix: String,
  item_adj: String,
  material_adj: String,
  item_noun: String,
}

impl<'a> WorkshopStringCandidate<'a> {
  fn should_match_prefix(&self) -> bool {
    matches!(self.state, Init)
  }

  fn match_prefix(&self, translated: String, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.prefix = translated;
    candidate.state = PrefixMatched;
    candidate.remaining = remaining;
    candidate
  }

  fn should_match_item_adjective(&self) -> bool {
    matches!(self.state, PrefixMatched)
  }

  fn match_item_adjective(&self, translated: String, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.item_adj = translated;
    candidate.state = ItemAdjMatched;
    candidate.remaining = remaining;
    candidate
  }

  fn should_match_material_adjective(&self) -> bool {
    matches!(self.state, PrefixMatched | ItemAdjMatched)
  }

  fn match_material_adjective(&self, translated: String, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.material_adj = translated;
    candidate.state = MaterialAdjMatched;
    candidate.remaining = remaining;
    candidate
  }

  fn should_match_item_noun(&self) -> bool {
    matches!(self.state, PrefixMatched | ItemAdjMatched | MaterialAdjMatched)
  }

  fn match_item_noun(&self, translated: String, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.item_noun = translated;
    candidate.state = Matched;
    candidate.remaining = remaining;
    candidate
  }

  fn matched(&self) -> bool {
    matches!(self.state, Matched) && self.remaining.is_empty()
  }

  fn build(&self) -> String {
    let Self {
      prefix,
      item_adj,
      material_adj,
      item_noun,
      ..
    } = self;
    format!("{prefix}{material_adj}{item_adj}{item_noun}")
  }
}

pub fn match_workshop_string(string: &str) -> Option<String> {
  let mut candidates = Vec::new();
  let mut candidate = WorkshopStringCandidate::default();
  candidate.remaining = string;
  candidates.push(candidate);

  while !candidates.is_empty() {
    let mut next_candidates = Vec::new();

    for candidate in candidates.iter_mut() {
      if candidate.matched() {
        return Some(candidate.build());
      }

      if candidate.should_match_prefix() {
        let matches = common::deprecated_match_dictionary(&PREFIX, candidate.remaining);
        for common::WordMatch {
          translated, remaining, ..
        } in matches.into_iter()
        {
          next_candidates.push(candidate.match_prefix(translated, remaining));
        }
      }

      if candidate.should_match_item_adjective() {
        let matches = common::deprecated_match_dictionary(&data::ITEMS.adjectives, candidate.remaining);
        for common::WordMatch {
          translated, remaining, ..
        } in matches.into_iter()
        {
          next_candidates.push(candidate.match_item_adjective(translated, remaining));
        }
      }

      if candidate.should_match_material_adjective() {
        let matches = common::deprecated_match_dictionaries(
          vec![&data::MATERIALS_TEMPLATES.adjectives, &data::MATERIALS.adjectives],
          candidate.remaining,
        );
        for common::WordMatch {
          translated, remaining, ..
        } in matches.into_iter()
        {
          next_candidates.push(candidate.match_material_adjective(translated, remaining));
        }
      }

      if candidate.should_match_item_noun() {
        let matches = common::deprecated_match_dictionary(&data::ITEMS.nouns, candidate.remaining);
        for common::WordMatch {
          translated, remaining, ..
        } in matches.into_iter()
        {
          next_candidates.push(candidate.match_item_noun(translated, remaining));
        }
      }
    }

    candidates = next_candidates;
  }

  None
}
