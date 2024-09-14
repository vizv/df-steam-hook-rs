use super::super::super::data;
use super::{pipeline, word};

pub fn wildcard_matcher<'a, M, R>(
  wildcard_table: &'a data::WildcardTable,
  match_placeholder: M,
  replace_placeholder: R,
) -> pipeline::MatchFn
where
  M: Fn(&'a str) -> Vec<word::WordMatch<'a>> + 'a,
  R: Fn(String, String) -> String + 'a,
{
  Box::new(move |remaining| {
    let mut ret = pipeline::MatchResults::default();

    let matches = match_wildcard_table_prefix(wildcard_table, remaining);
    for (remaining, dict) in matches.into_iter() {
      let matches = match_placeholder(remaining);
      for word::WordMatch { translated, remaining } in matches.into_iter() {
        let matches = word::deprecated_match_dictionary(dict, remaining);
        for word::WordMatch {
          translated: placeholder,
          remaining,
        } in matches.into_iter()
        {
          let translated = replace_placeholder(placeholder, translated.to_string());
          ret.push((remaining, translated));
        }
      }
    }

    ret
  })
}

fn match_wildcard_table_prefix<'a>(
  wildcard_table: &'a data::WildcardTable,
  remaining: &'a str,
) -> Vec<(&'a str, &'a data::Dictionary)> {
  let mut ret = Vec::<(&'a str, &'a data::Dictionary)>::default();

  // match empty prefix
  if let Some(dict) = wildcard_table.get("") {
    ret.push((remaining, dict));
  }

  for (i, (pos, _)) in remaining.match_indices(' ').enumerate() {
    let word_count = i + 1;
    if word_count > wildcard_table.max_count {
      break;
    }

    // split by the space, the the current prefix and remaining part
    let (prefix, remaining) = remaining.split_at(pos);
    let remaining = &remaining[1..];

    // lookup wildcard table for prefix, run the closure for each match with translated string
    if let Some(dict) = wildcard_table.get(prefix) {
      ret.push((remaining, dict));
    }
  }

  ret
}
