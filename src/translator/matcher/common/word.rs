use super::{super::super::data, pipeline};

pub struct WordMatch<'a> {
  pub translated: String,
  pub remaining: &'a str,
}

pub fn word_matcher(dict: &data::Dictionary) -> pipeline::MatchFn {
  Box::new(move |remaining| {
    let mut results = pipeline::MatchResults::default();

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

    results
  })
}

pub fn deprecated_match_dictionaries<'a>(dicts: Vec<&'a data::Dictionary>, remaining: &'a str) -> Vec<WordMatch<'a>> {
  let mut ret = Vec::<WordMatch>::default();

  for &dict in dicts.iter() {
    ret.append(&mut deprecated_match_dictionary(dict, remaining));
  }

  ret
}

pub fn deprecated_match_dictionary<'a>(dict: &'a data::Dictionary, remaining: &'a str) -> Vec<WordMatch<'a>> {
  word_matcher(dict)(&remaining)
    .into_iter()
    .map(|(remaining, translated)| WordMatch { remaining, translated })
    .collect()
}
