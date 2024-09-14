use super::super::super::data;
use super::{pipeline, word, WordMatch};

pub fn wildcard_matcher<'a, M, R>(
  wildcard_table: &'a data::WildcardTable,
  match_placeholder: M,
  replace_placeholder: R,
) -> pipeline::MatchFn
where
  M: Fn(&'a str) -> Vec<word::WordMatch<'a>> + 'a,
  R: Fn(String, String, &'a str, &'a str, &'a str) -> String + 'a,
{
  Box::new(move |remaining| {
    let mut ret = pipeline::MatchResults::default();

    let matches = match_wildcard_table_prefix(wildcard_table, remaining);
    for WildcardPrefixMatch {
      dict,
      prefix,
      remaining,
    } in matches.into_iter()
    {
      let matches = match_placeholder(remaining);
      for word::WordMatch {
        translated,
        original,
        remaining,
      } in matches.into_iter()
      {
        let matches = word::deprecated_match_dictionary(dict, remaining);
        for word::WordMatch {
          translated: placeholder,
          original: suffix,
          remaining,
        } in matches.into_iter()
        {
          let translated = replace_placeholder(placeholder, translated.to_string(), prefix, original, suffix);
          ret.push((remaining, original, translated));
        }
      }
    }

    ret
  })
}

pub fn match_wildcard_table<'a, M, R>(
  wildcard_table: &'a data::WildcardTable,
  remaining: &'a str,
  match_placeholder: M,
  replace_placeholder: R,
) -> Vec<word::WordMatch<'a>>
where
  M: Fn(&'a str) -> Vec<word::WordMatch<'a>> + 'a,
  R: Fn(String, String, &'a str, &'a str, &'a str) -> String + 'a,
{
  wildcard_matcher(&wildcard_table, match_placeholder, replace_placeholder)(remaining)
    .into_iter()
    .map(|(remaining, original, translated)| WordMatch {
      remaining,
      original,
      translated,
    })
    .collect()
}

struct WildcardPrefixMatch<'a> {
  dict: &'a data::Dictionary,
  prefix: &'a str,
  remaining: &'a str,
}

fn match_wildcard_table_prefix<'a>(
  wildcard_table: &'a data::WildcardTable,
  remaining: &'a str,
) -> Vec<WildcardPrefixMatch<'a>> {
  let mut ret: Vec<WildcardPrefixMatch<'a>> = Default::default();

  // match empty prefix
  if let Some(dict) = wildcard_table.get("") {
    ret.push(WildcardPrefixMatch {
      dict,
      prefix: "",
      remaining,
    });
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
      ret.push(WildcardPrefixMatch {
        dict,
        prefix,
        remaining,
      });
    }
  }

  ret
}
