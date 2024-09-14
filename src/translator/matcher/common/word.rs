use super::super::super::data;

pub struct WordMatch<'a> {
  pub original: &'a str,
  pub translated: &'a str,
  pub remaining: &'a str,
}

pub fn match_dictionaries<'a>(dicts: Vec<&'a data::Dictionary>, remaining: &'a str) -> Vec<WordMatch<'a>> {
  let mut ret: Vec<WordMatch> = Default::default();

  for &dict in dicts.iter() {
    ret.append(&mut match_dictionary(dict, remaining));
  }

  ret
}

pub fn match_dictionary<'a>(dict: &'a data::Dictionary, remaining: &'a str) -> Vec<WordMatch<'a>> {
  let mut ret: Vec<WordMatch<'a>> = Default::default();

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
      ret.push(WordMatch {
        translated,
        original: prefix,
        remaining,
      });
    }
  }

  // lookup dictionary for the whole remaining part (split at end of remaining part)
  if let Some(translated) = dict.get(remaining) {
    ret.push(WordMatch {
      translated,
      original: remaining,
      remaining: "",
    });
  }

  ret
}
