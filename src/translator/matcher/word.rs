use super::super::data;
use super::common;

// (remaining, translated)
pub type WordMatch<'a> = (&'a str, String);

pub enum DictType<'a> {
  Dictionary(&'a data::Dictionary),
  Dictionaries(Vec<&'a data::Dictionary>),
}

pub fn word_matcher(dict: DictType) -> common::MatchFn {
  let dicts = match dict {
    DictType::Dictionary(dict) => vec![dict],
    DictType::Dictionaries(dicts) => dicts,
  };

  Box::new(move |remaining| {
    let mut results = common::MatchResults::default();

    for &dict in dicts.iter() {
      for (i, (pos, _)) in remaining.match_indices(' ').enumerate() {
        let word_count = i + 1;
        if word_count > dict.max_count {
          break;
        }

        // split by the space, the the current prefix and remaining part
        let (prefix, remaining) = remaining.split_at(pos);
        let remaining = &remaining[1..];

        // lookup dictionary for prefix, run the closure for each match with translated string
        if let Some(translated) = dict.get(prefix) {
          results.push((remaining, translated.to_owned()));
        }
      }

      // lookup dictionary for the whole remaining part (split at end of remaining part)
      if let Some(translated) = dict.get(remaining) {
        results.push(("", translated.to_owned()));
      }
    }

    results
  })
}
