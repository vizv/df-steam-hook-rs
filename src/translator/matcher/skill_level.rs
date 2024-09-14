use super::super::data;
use super::common;

use SkillLevelMatchingState::*;

#[derive(Debug, Clone, Copy)]
enum SkillLevelMatchingState {
  Init,         // initial state, can match level
  LevelMatched, // can match skill
  Matched,      // finish state
}

impl Default for SkillLevelMatchingState {
  fn default() -> Self {
    Self::Init
  }
}

#[derive(Debug, Default, Clone, Copy)]
struct SkillLevelCandidate<'a> {
  state: SkillLevelMatchingState,
  remaining: &'a str,

  level: &'a str,
  skill: &'a str,
}

impl<'a> SkillLevelCandidate<'a> {
  fn should_match_level(&self) -> bool {
    matches!(self.state, Init)
  }

  fn match_level(&self, translated: &'a str, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.level = translated;
    candidate.state = LevelMatched;
    candidate.remaining = remaining;
    candidate
  }

  fn should_match_skill(&self) -> bool {
    matches!(self.state, LevelMatched)
  }

  fn match_skill(&self, translated: &'a str, remaining: &'a str) -> Self {
    let mut candidate = self.clone();
    candidate.skill = translated;
    candidate.state = Matched;
    candidate.remaining = remaining;
    candidate
  }

  fn matched(&self) -> bool {
    matches!(self.state, Matched) && self.remaining.is_empty()
  }

  fn build(self) -> String {
    let Self { level, skill, .. } = self;
    format!("{level}{skill}")
  }
}

pub fn match_skill_level(string: &String) -> Option<String> {
  let mut candidates = Vec::new();
  let mut candidate = SkillLevelCandidate::default();
  candidate.remaining = string.as_str();
  candidates.push(candidate);

  while !candidates.is_empty() {
    let mut next_candidates = Vec::new();

    for candidate in candidates.iter_mut() {
      if candidate.matched() {
        return Some(candidate.build());
      }

      if candidate.should_match_level() {
        let matches = common::match_dictionary(&data::SKILL_LEVELS.adjectives, candidate.remaining);
        for &common::WordMatch {
          translated, remaining, ..
        } in matches.iter()
        {
          next_candidates.push(candidate.match_level(translated, remaining));
        }
      }

      if candidate.should_match_skill() {
        let matches = common::match_dictionary(&data::SKILL_NAMES.nouns, candidate.remaining);
        for &common::WordMatch {
          translated, remaining, ..
        } in matches.iter()
        {
          next_candidates.push(candidate.match_skill(translated, remaining));
        }
      }
    }

    candidates = next_candidates;
  }

  None
}
